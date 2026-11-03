use std::{ffi::c_void,fs::{self,File,OpenOptions},io::{Read,Write},path::{Path,Component},sync::{Arc,atomic::{AtomicBool,Ordering}},time::{Duration,Instant}};
use std::os::windows::fs::{MetadataExt,OpenOptionsExt};
use windows_sys::Win32::{Foundation::*,Networking::WinHttp::*,Security::Cryptography::*,Storage::Compression::*};
use crate::response_offset;
#[path="assets.rs"]mod assets;
pub const MODEL_FILE:&str="TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf";
pub const MODEL_SIZE:u64=1_082_015_808;
pub const MODEL_SHA:&str="b5056635be0c9bb832f0c964dc063bd4b4462e253f9f9d94eebf0aa0960b7798";
pub const MODEL_HOST:&str="huggingface.co";
pub const MODEL_PATH:&str="/qvac/TranslatePsy-AfriSLM-0.8B-Q8-GGUF/resolve/55220bcc74bed219ad9db9ab378f6aae31850974/TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf";
pub const OWNER:&str="LughaDeskLauncher-38AB9659-3991-4463-8995-88A76853623E";
pub struct SetupLock{_file:File}
pub fn acquire_lock(root:&Path)->Result<SetupLock,String>{
 let parent=root.parent().ok_or("Missing setup parent folder.")?;
 let name=root.file_name().and_then(|s|s.to_str()).ok_or("Invalid setup folder name.")?;
 let path=safe_path(parent,&format!(".{name}.setup.lock"))?;
 fs::create_dir_all(parent).map_err(|e|e.to_string())?;
 let file=OpenOptions::new().create(true).read(true).write(true).share_mode(0).open(path)
  .map_err(|_|"Setup is already open in another session, or Windows cannot lock the setup folder. Close other launchers and retry.".to_owned())?;
 Ok(SetupLock{_file:file})
}
pub fn wide(s:&str)->Vec<u16>{s.encode_utf16().chain(Some(0)).collect()}
pub fn sha256(path:&Path)->Result<String,String>{
 let mut input=File::open(path).map_err(|e|format!("Cannot read setup file: {e}"))?;
 unsafe{
  let mut provider=std::ptr::null_mut();if BCryptOpenAlgorithmProvider(&mut provider,BCRYPT_SHA256_ALGORITHM,std::ptr::null(),0)<0{return Err("Windows hashing is unavailable.".into())}
  struct Provider(BCRYPT_ALG_HANDLE);impl Drop for Provider{fn drop(&mut self){unsafe{BCryptCloseAlgorithmProvider(self.0,0);}}}
  let provider=Provider(provider);let mut size=0u32;let mut used=0;
  if BCryptGetProperty(provider.0,BCRYPT_OBJECT_LENGTH,&mut size as *mut _ as _,4,&mut used,0)<0||used!=4||size==0||size>1_048_576{return Err("Invalid Windows hash buffer.".into())}
  let mut workspace=vec![0u8;size as usize];let mut hash=std::ptr::null_mut();
  if BCryptCreateHash(provider.0,&mut hash,workspace.as_mut_ptr(),size,std::ptr::null(),0,0)<0{return Err("Cannot create verification hash.".into())}
  struct Hash(BCRYPT_HASH_HANDLE);impl Drop for Hash{fn drop(&mut self){unsafe{BCryptDestroyHash(self.0);}}}
  let hash=Hash(hash);let mut buffer=vec![0;1024*1024];loop{let n=input.read(&mut buffer).map_err(|e|e.to_string())?;if n==0{break}if BCryptHashData(hash.0,buffer.as_ptr(),n as u32,0)<0{return Err("Cannot hash setup data.".into())}}
  let mut digest=[0u8;32];if BCryptFinishHash(hash.0,digest.as_mut_ptr(),32,0)<0{return Err("Cannot finish verification.".into())}
  Ok(digest.iter().map(|b|format!("{b:02x}")).collect())
 }
}
pub fn safe_path(root:&Path,relative:&str)->Result<std::path::PathBuf,String>{
 if relative.is_empty()||Path::new(relative).components().any(|c|!matches!(c,Component::Normal(_)))||relative.contains(':'){return Err("Unsafe setup file path.".into())}
 let target=root.join(relative);for ancestor in target.ancestors(){if let Ok(m)=fs::symlink_metadata(ancestor){if m.file_attributes()&0x400!=0{return Err("Setup cannot use redirected or linked folders. Choose a normal local folder.".into())}}}
 Ok(target)
}
fn write_atomic(root:&Path,name:&str,bytes:&[u8])->Result<(),String>{
 let file=safe_path(root,name)?;let part=safe_path(root,&format!("{name}.installing"))?;
 fs::create_dir_all(file.parent().ok_or("Missing parent.")?).map_err(|e|e.to_string())?;
 let mut output=File::create(&part).map_err(|e|e.to_string())?;output.write_all(bytes).and_then(|_|output.sync_all()).map_err(|e|e.to_string())?;drop(output);
 fs::rename(&part,&file).map_err(|e|format!("Cannot finish installing a file: {e}"))
}
pub fn prepare(root:&Path)->Result<(),String>{
 safe_path(root,".lugha-desk-owner")?;
 if root.exists(){
  let mut entries=fs::read_dir(root).map_err(|e|e.to_string())?;
  if entries.next().is_some()&&fs::read_to_string(root.join(".lugha-desk-owner")).ok().as_deref()!=Some(OWNER){return Err("Setup folder contains unrelated files. Nothing was replaced.".into())}
 }
 fs::create_dir_all(root).map_err(|e|e.to_string())?;
 write_atomic(root,".lugha-desk-owner",OWNER.as_bytes())?;
 for asset in assets::ASSETS {
  let path=safe_path(root,asset.path)?;
  if path.is_file()&&sha256(&path)?==asset.sha{continue}
  let mut output=vec![0u8;asset.len];let mut used=0usize;
  unsafe{
   let mut handle=std::ptr::null_mut();if CreateDecompressor(COMPRESS_ALGORITHM_XPRESS_HUFF,std::ptr::null(),&mut handle)==0{return Err("Windows decompression is unavailable.".into())}
   let ok=Decompress(handle,asset.data.as_ptr() as _,asset.data.len(),output.as_mut_ptr() as _,output.len(),&mut used);CloseDecompressor(handle);
   if ok==0||used!=asset.len{return Err("Embedded app data failed to decompress.".into())}
  }
  write_atomic(root,asset.path,&output)?;
  if sha256(&path)?!=asset.sha{return Err("An installed app file failed verification.".into())}
 }
 fs::create_dir_all(root.join("model")).map_err(|e|e.to_string())?;safe_path(root,&format!("model/{MODEL_FILE}"))?;Ok(())
}
pub fn complete(root:&Path)->Result<bool,String>{
 let file=safe_path(root,&format!("model/{MODEL_FILE}"))?;
 if !file.is_file(){return Ok(false)}
 if file.metadata().map_err(|e|e.to_string())?.len()!=MODEL_SIZE{return Ok(false)}
 Ok(sha256(&file)?==MODEL_SHA)
}
#[derive(Debug,Clone)]pub struct Progress{pub done:u64,pub total:u64,pub message:String}
struct Http(*mut c_void);impl Drop for Http{fn drop(&mut self){unsafe{WinHttpCloseHandle(self.0);}}}
fn handle(h:*mut c_void)->Result<Http,String>{if h.is_null(){Err(format!("Could not connect (Windows {}). Saved progress is kept.",unsafe{GetLastError()}))}else{Ok(Http(h))}}
unsafe fn header(request:*mut c_void,which:u32)->Option<String>{
 let mut buffer=vec![0u16;2048];let mut len=(buffer.len()*2) as u32;
 if WinHttpQueryHeaders(request,which,std::ptr::null(),buffer.as_mut_ptr() as _,&mut len,std::ptr::null_mut())==0{return None}
 let count=buffer.iter().position(|&c|c==0).unwrap_or(buffer.len());Some(String::from_utf16_lossy(&buffer[..count]))
}
pub fn download(root:&Path,cancel:Arc<AtomicBool>,mut progress:impl FnMut(Progress))->Result<(),String>{
 if complete(root)?{progress(Progress{done:MODEL_SIZE,total:MODEL_SIZE,message:"Ready for offline translation.".into()});return Ok(())}
 transfer(root,MODEL_HOST,443,MODEL_PATH,true,MODEL_SIZE,MODEL_SHA,cancel,&mut progress)
}
fn transfer(root:&Path,host:&str,port:u16,path:&str,secure:bool,expected:u64,sha:&str,cancel:Arc<AtomicBool>,progress:&mut impl FnMut(Progress))->Result<(),String>{
 if !secure && host!="127.0.0.1"{return Err("HTTPS is required.".into())}
 let part=safe_path(root,&format!("model/{MODEL_FILE}.part"))?;
 let final_path=safe_path(root,&format!("model/{MODEL_FILE}"))?;
 let mut offset=fs::metadata(&part).map(|m|m.len()).unwrap_or(0);
 if offset>expected{File::create(&part).map_err(|e|e.to_string())?;offset=0;}
 if offset==expected{
  if sha256(&part)?==sha{return fs::rename(&part,&final_path).map_err(|e|e.to_string())}
  File::create(&part).map_err(|e|e.to_string())?;offset=0;
 }
 if cancel.load(Ordering::Relaxed){return Err("Paused. Your download is saved; choose Resume setup.".into())}
 progress(Progress{done:offset,total:expected,message:"Connecting securely…".into()});
 unsafe {
  let session=handle(WinHttpOpen(wide("LughaDeskSetup/0.3").as_ptr(),WINHTTP_ACCESS_TYPE_AUTOMATIC_PROXY,std::ptr::null(),std::ptr::null(),0))?;
  if WinHttpSetTimeouts(session.0,5000,10000,15000,15000)==0{return Err("Cannot set download time limits.".into())}
  let connection=handle(WinHttpConnect(session.0,wide(host).as_ptr(),port,0))?;
  let request=handle(WinHttpOpenRequest(connection.0,wide("GET").as_ptr(),wide(path).as_ptr(),std::ptr::null(),std::ptr::null(),std::ptr::null(),if secure{WINHTTP_FLAG_SECURE}else{0}))?;
  for (option,value) in [(WINHTTP_OPTION_REDIRECT_POLICY,WINHTTP_OPTION_REDIRECT_POLICY_DISALLOW_HTTPS_TO_HTTP),(WINHTTP_OPTION_AUTOLOGON_POLICY,WINHTTP_AUTOLOGON_SECURITY_LEVEL_HIGH)]{
   if WinHttpSetOption(request.0,option,&value as *const _ as _,4)==0{return Err("Cannot enforce secure download settings.".into())}
  }
  let headers=wide(&format!("Accept-Encoding: identity\r\n{}",if offset>0{format!("Range: bytes={offset}-\r\n")}else{String::new()}));
  if WinHttpSendRequest(request.0,headers.as_ptr(),(headers.len()-1) as u32,std::ptr::null(),0,0,0)==0||WinHttpReceiveResponse(request.0,std::ptr::null_mut())==0{return Err(format!("Connection interrupted (Windows {}). Your saved download can resume.",GetLastError()))}
  let status=header(request.0,WINHTTP_QUERY_STATUS_CODE).and_then(|s|s.parse().ok()).ok_or("Cannot read server status.")?;
  offset=response_offset(status,header(request.0,WINHTTP_QUERY_CONTENT_RANGE).as_deref(),offset,expected)?;
  if let Some(length)=header(request.0,WINHTTP_QUERY_CONTENT_LENGTH){if length.parse::<u64>().map_err(|_|"Invalid download length.")?!=expected-offset{return Err("Server file size did not match the expected model.".into())}}
  let mut file=OpenOptions::new().create(true).write(true).append(offset>0).truncate(offset==0).open(&part).map_err(|e|format!("Cannot save download: {e}"))?;
  let start=offset;let started=Instant::now();let mut last=Instant::now()-Duration::from_secs(1);let mut buffer=vec![0u8;64*1024];
  loop{
   if cancel.load(Ordering::Relaxed){file.sync_all().map_err(|e|e.to_string())?;return Err("Paused. Your download is saved; choose Resume setup.".into())}
   let mut count=0;if WinHttpReadData(request.0,buffer.as_mut_ptr() as _,buffer.len() as u32,&mut count)==0{return Err("The connection stopped. Choose Resume setup to continue from saved progress.".into())}
   if count==0{break}
   if offset+count as u64>expected{return Err("The server sent more data than expected. Setup stopped safely.".into())}
   file.write_all(&buffer[..count as usize]).map_err(|e|format!("Cannot save download. Check free disk space: {e}"))?;offset+=count as u64;
   if last.elapsed()>=Duration::from_millis(200){
    let rate=(offset-start) as f64/started.elapsed().as_secs_f64().max(0.001)/1_000_000.0;
    progress(Progress{done:offset,total:expected,message:format!("Downloading model · {rate:.1} MB/s")});last=Instant::now();
   }
  }
  file.sync_all().map_err(|e|e.to_string())?;drop(file);
  if offset!=expected{return Err("Download incomplete. Choose Resume setup to continue.".into())}
 }
 progress(Progress{done:expected,total:expected,message:"Verifying model integrity…".into()});
 if sha256(&part)?!=sha{
  File::create(&part).map_err(|e|e.to_string())?;
  return Err("The model failed verification. Choose Resume setup to start a fresh, verified download.".into())
 }
 fs::rename(&part,&final_path).map_err(|e|format!("Cannot finish model setup: {e}"))?;
 progress(Progress{done:expected,total:expected,message:"Ready for offline translation.".into()});Ok(())
}

#[cfg(test)]mod tests{
 use super::*;
 fn root()->std::path::PathBuf{let root=std::env::temp_dir().join(format!("lugha-launcher-{}-{:?}",std::process::id(),std::thread::current().id()));fs::create_dir_all(&root).unwrap();root}
 #[test]fn embedded_resources_roundtrip(){let root=root();prepare(&root).unwrap();for a in assets::ASSETS{assert_eq!(sha256(&root.join(a.path)).unwrap(),a.sha);}assert!(!complete(&root).unwrap());}
 #[test]fn rejects_unrelated_folder_and_paths(){let root=root();fs::write(root.join("personal.txt"),b"do not replace").unwrap();assert!(prepare(&root).is_err());for p in ["../a","/a","C:/a","a/../b"]{assert!(safe_path(&root,p).is_err());}assert_eq!(fs::read(root.join("personal.txt")).unwrap(),b"do not replace");}
 #[test]fn setup_lock_is_exclusive_and_recovers_on_close(){let root=root();let lock=acquire_lock(&root).unwrap();assert!(acquire_lock(&root).is_err());drop(lock);assert!(acquire_lock(&root).is_ok());}
 fn fixture()->(std::path::PathBuf,String){let root=root();fs::create_dir_all(root.join("model")).unwrap();fs::write(root.join("fixture.txt"),b"0123456789").unwrap();let sha=sha256(&root.join("fixture.txt")).unwrap();(root,sha)}
 fn server(response:Vec<u8>)->(u16,std::thread::JoinHandle<String>){
  let listener=std::net::TcpListener::bind("127.0.0.1:0").unwrap();let port=listener.local_addr().unwrap().port();
  let join=std::thread::spawn(move||{let (mut stream,_)=listener.accept().unwrap();stream.set_read_timeout(Some(Duration::from_secs(10))).unwrap();let mut req=Vec::new();loop{let mut b=[0];stream.read_exact(&mut b).unwrap();req.push(b[0]);if req.ends_with(b"\r\n\r\n"){break}}
   stream.write_all(&response).unwrap();String::from_utf8_lossy(&req).into_owned()});(port,join)
 }
 #[test]fn interrupted_response_resumes_without_duplication(){
  let (root,sha)=fixture();let (port,first)=server(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\n0123".to_vec());
  assert!(transfer(&root,"127.0.0.1",port,"/model",false,10,&sha,Arc::new(AtomicBool::new(false)),&mut |_|{}).is_err());first.join().unwrap();
  assert_eq!(fs::read(root.join(format!("model/{MODEL_FILE}.part"))).unwrap(),b"0123");
  let (port,second)=server(b"HTTP/1.1 206 Partial Content\r\nContent-Length: 6\r\nContent-Range: bytes 4-9/10\r\nConnection: close\r\n\r\n456789".to_vec());
  transfer(&root,"127.0.0.1",port,"/model",false,10,&sha,Arc::new(AtomicBool::new(false)),&mut |_|{}).unwrap();
  assert!(second.join().unwrap().to_lowercase().contains("range: bytes=4-"));assert_eq!(fs::read(root.join(format!("model/{MODEL_FILE}"))).unwrap(),b"0123456789");
 }
 #[test]fn server_ignoring_range_restarts_safely(){
  let (root,sha)=fixture();fs::write(root.join(format!("model/{MODEL_FILE}.part")),b"0123").unwrap();
  let (port,join)=server(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\n0123456789".to_vec());
  transfer(&root,"127.0.0.1",port,"/",false,10,&sha,Arc::new(AtomicBool::new(false)),&mut |_|{}).unwrap();join.join().unwrap();assert_eq!(fs::read(root.join(format!("model/{MODEL_FILE}"))).unwrap(),b"0123456789");
 }
 #[test]fn corrupt_download_never_becomes_ready(){
  let (root,sha)=fixture();let (port,join)=server(b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\nConnection: close\r\n\r\nXXXXXXXXXX".to_vec());
  assert!(transfer(&root,"127.0.0.1",port,"/",false,10,&sha,Arc::new(AtomicBool::new(false)),&mut |_|{}).is_err());join.join().unwrap();
  assert!(!root.join(format!("model/{MODEL_FILE}")).exists());assert_eq!(fs::metadata(root.join(format!("model/{MODEL_FILE}.part"))).unwrap().len(),0);
 }
 #[test]fn cancelled_download_keeps_prefix(){let (root,sha)=fixture();fs::write(root.join(format!("model/{MODEL_FILE}.part")),b"0123").unwrap();assert!(transfer(&root,"127.0.0.1",1,"/",false,10,&sha,Arc::new(AtomicBool::new(true)),&mut |_|{}).is_err());assert_eq!(fs::read(root.join(format!("model/{MODEL_FILE}.part"))).unwrap(),b"0123");}
 #[test]#[ignore="large-model integration; set LUGHA_TEST_MODEL and run explicitly"]fn existing_real_model_requires_no_download(){
  let model=std::env::var_os("LUGHA_TEST_MODEL").expect("LUGHA_TEST_MODEL must name the verified fixture model");let root=root();prepare(&root).unwrap();fs::copy(model,root.join(format!("model/{MODEL_FILE}"))).unwrap();assert!(complete(&root).unwrap());
  download(&root,Arc::new(AtomicBool::new(true)),|_|{}).unwrap();
  if let Some(report)=std::env::var_os("LUGHA_TEST_INSTALL_REPORT"){fs::write(report,root.to_string_lossy().as_bytes()).unwrap();}
 }
 #[test]#[ignore="explicit public HTTPS endpoint probe; downloads only two small chunks"]fn public_https_model_resumes_after_pause(){
  let root=root();fs::create_dir_all(root.join("model")).unwrap();
  for _ in 0..2{
   let before=fs::metadata(root.join(format!("model/{MODEL_FILE}.part"))).map(|m|m.len()).unwrap_or(0);
   let cancel=Arc::new(AtomicBool::new(false));let flag=cancel.clone();
   let result=transfer(&root,MODEL_HOST,443,MODEL_PATH,true,MODEL_SIZE,MODEL_SHA,cancel,&mut |p|{if p.done>before{flag.store(true,Ordering::Relaxed)}});
   assert!(result.is_err());let after=fs::metadata(root.join(format!("model/{MODEL_FILE}.part"))).unwrap().len();assert!(after>before&&after<MODEL_SIZE);
  }
  assert!(!root.join(format!("model/{MODEL_FILE}")).exists());
 }
}
