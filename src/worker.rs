//! One private, job-isolated worker per open application session.
use std::{io::{BufRead,Read,Write},process::{Child,ChildStdin,Command,Stdio},sync::{mpsc,Arc,atomic::{AtomicBool,Ordering}},time::{Duration,Instant}};
use std::os::windows::{io::AsRawHandle,process::CommandExt};
use windows_sys::Win32::{Foundation::{HANDLE,CloseHandle},System::JobObjects::*};
struct Job(HANDLE);
unsafe impl Send for Job {}
impl Drop for Job{fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
pub struct Worker{child:Child,input:ChildStdin,output:mpsc::Receiver<Result<serde_json::Value,String>>,_job:Job}
impl Drop for Worker{fn drop(&mut self){let _=self.child.kill();let _=self.child.wait();}}
impl Worker{
 pub fn start()->Result<Self,String>{
  let mut child=Command::new(std::env::current_exe().map_err(|_|"Cannot locate app.")?).arg("--session-worker")
   .creation_flags(0x08000000).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null())
   .spawn().map_err(|_|"Cannot start the local translation worker.")?;
  // The worker blocks for its first request until the whole tree is isolated.
  let job=unsafe{
   let h=CreateJobObjectW(std::ptr::null(),std::ptr::null());
   if h.is_null(){None}else{
    let job=Job(h);let mut limits:JOBOBJECT_EXTENDED_LIMIT_INFORMATION=std::mem::zeroed();
    limits.BasicLimitInformation.LimitFlags=JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
    if SetInformationJobObject(h,JobObjectExtendedLimitInformation,&limits as *const _ as _,std::mem::size_of_val(&limits) as u32)==0||AssignProcessToJobObject(h,child.as_raw_handle() as HANDLE)==0{None}else{Some(job)}
   }
  };
  let job=match job{Some(v)=>v,None=>{let _=child.kill();let _=child.wait();return Err("Windows could not isolate the local worker. Translation was not started.".into());}};
  let input=child.stdin.take().ok_or("Worker input unavailable.")?;
  let stdout=child.stdout.take().ok_or("Worker output unavailable.")?;
  let (tx,rx)=mpsc::sync_channel(8);
  std::thread::spawn(move||{
   let mut reader=std::io::BufReader::new(stdout);
   loop{
    let mut bytes=Vec::new();
    let result=match reader.by_ref().take(16386).read_until(b'\n',&mut bytes){
     Ok(0)=>Err("Local worker stopped. Choose Translate to restart it.".into()),
     Ok(n) if n<=16384&&bytes.ends_with(b"\n")=>serde_json::from_slice(&bytes).map_err(|_|"Invalid local worker response.".into()),
     _=>Err("Incomplete or oversized local worker response.".into())
    };
    let stop=result.is_err();if tx.send(result).is_err()||stop{break;}
   }
  });
  Ok(Self{child,input,output:rx,_job:job})
 }
 pub fn translate(&mut self,direction:&str,text:&str,cancel:&Arc<AtomicBool>,mut progress:impl FnMut(&str))->Result<String,String>{
  lugha_desk::validate(direction,text)?;
  let data=serde_json::json!({"direction":direction,"text":text});
  writeln!(self.input,"{data}").and_then(|_|self.input.flush()).map_err(|_|"Local worker input closed. Choose Translate to restart it.")?;
  wait_response(&self.output,cancel,&mut progress,Duration::from_secs(90),Duration::from_secs(240))
 }
}
 fn wait_response(output:&mpsc::Receiver<Result<serde_json::Value,String>>,cancel:&AtomicBool,progress:&mut impl FnMut(&str),startup:Duration,total:Duration)->Result<String,String>{
  let started=Instant::now();let mut translating=false;
  loop{
   if cancel.load(Ordering::Relaxed){return Err("Translation cancelled. Choose Translate when ready.".into())}
   if started.elapsed()>total||(!translating&&started.elapsed()>startup){return Err("Local translation timed out. Choose Translate to restart the model.".into())}
   match output.recv_timeout(Duration::from_millis(100)){
    Ok(Ok(v))=>{
     if let Some(phase)=v["phase"].as_str(){
      if !matches!(phase,"Checking saved model"|"Loading local model"|"Translating offline"){return Err("Invalid worker progress.".into())}
      if phase=="Translating offline"{translating=true;}progress(phase);
     }else if let Some(s)=v["result"].as_str(){if s.trim().is_empty(){return Err("Empty local translation.".into())}return Ok(s.to_owned())}
     else if let Some(e)=v["error"].as_str(){return Err(e.to_owned())}
     else{return Err("Invalid local worker response.".into())}
    },
    Ok(Err(e))=>return Err(e),
    Err(mpsc::RecvTimeoutError::Disconnected)=>return Err("Local worker stopped. Choose Translate to restart it.".into()),
    Err(mpsc::RecvTimeoutError::Timeout)=>{}
   }
  }
 }
pub fn run(session:&mut Option<Worker>,direction:&str,text:&str,cancel:Arc<AtomicBool>,mut progress:impl FnMut(&str))->Result<String,String>{
 lugha_desk::validate(direction,text)?;
 if session.is_none(){*session=Some(Worker::start()?);}
 let result=session.as_mut().unwrap().translate(direction,text,&cancel,&mut progress);
 // Drop job on cancellation/error/timeout; the next click starts a clean process.
 if result.is_err(){*session=None;}
 result
}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn bounded_wait_times_out_without_worker_messages(){
  let (_tx,rx)=mpsc::channel();let start=Instant::now();
  assert!(wait_response(&rx,&AtomicBool::new(false),&mut |_|{},Duration::from_millis(10),Duration::from_secs(1)).unwrap_err().contains("timed out"));
  assert!(start.elapsed()<Duration::from_secs(1));
 }
 #[test]fn cancellation_is_immediate(){
  let (_tx,rx)=mpsc::channel();
  assert!(wait_response(&rx,&AtomicBool::new(true),&mut |_|{},Duration::from_secs(1),Duration::from_secs(1)).unwrap_err().contains("cancelled"));
 }
 #[test]fn phase_progress_and_result_are_separate(){
  let (tx,rx)=mpsc::channel();
  tx.send(Ok(serde_json::json!({"phase":"Loading local model"}))).unwrap();
  tx.send(Ok(serde_json::json!({"phase":"Translating offline"}))).unwrap();
  tx.send(Ok(serde_json::json!({"result":"Habari"}))).unwrap();
  let mut phases=Vec::new();
  assert_eq!(wait_response(&rx,&AtomicBool::new(false),&mut |p|phases.push(p.to_owned()),Duration::from_secs(1),Duration::from_secs(1)).unwrap(),"Habari");
  assert_eq!(phases,["Loading local model","Translating offline"]);
 }
 #[test]fn dead_worker_and_invalid_messages_are_errors(){
  let (tx,rx)=mpsc::channel();drop(tx);
  assert!(wait_response(&rx,&AtomicBool::new(false),&mut |_|{},Duration::from_secs(1),Duration::from_secs(1)).is_err());
  let (tx,rx)=mpsc::channel();tx.send(Ok(serde_json::json!({"phase":"unknown"}))).unwrap();
  assert!(wait_response(&rx,&AtomicBool::new(false),&mut |_|{},Duration::from_secs(1),Duration::from_secs(1)).is_err());
 }
}
