//! Windows-native local inference. No Python, shell, proxy, redirects or remote host.
use std::{ffi::c_void,fs::File,io::Read,net::TcpListener,path::Path,process::{Command,Stdio},time::{Duration,Instant}};
use std::os::windows::process::CommandExt;
use lugha_desk::{payload,translation_from,wide};
use windows_sys::Win32::{Networking::WinHttp::*,Security::Cryptography::*};

pub const MODEL_FILE:&str="TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf";
pub const MODEL_SHA:&str="b5056635be0c9bb832f0c964dc063bd4b4462e253f9f9d94eebf0aa0960b7798";
struct Algorithm(BCRYPT_ALG_HANDLE);
impl Drop for Algorithm{fn drop(&mut self){unsafe{BCryptCloseAlgorithmProvider(self.0,0);}}}
struct Hash(BCRYPT_HASH_HANDLE);
impl Drop for Hash{fn drop(&mut self){unsafe{BCryptDestroyHash(self.0);}}}
fn checked(status:i32)->Result<(),String>{if status<0{Err("Windows cryptography operation failed.".into())}else{Ok(())}}
pub fn sha256_file(path:&Path)->Result<String,String>{
    let mut file=File::open(path).map_err(|_|"The model file is missing. Extract the complete ZIP first.")?;
    unsafe {
        let mut a=std::ptr::null_mut();checked(BCryptOpenAlgorithmProvider(&mut a,BCRYPT_SHA256_ALGORITHM,std::ptr::null(),0))?;let algorithm=Algorithm(a);
        let mut len=0u32;let mut written=0;
        checked(BCryptGetProperty(algorithm.0,BCRYPT_OBJECT_LENGTH,&mut len as *mut _ as _,4,&mut written,0))?;
        if written!=4||len==0||len>1_048_576{return Err("Invalid Windows SHA-256 workspace size.".into());}
        let mut object=vec![0u8;len as usize];let mut h=std::ptr::null_mut();
        checked(BCryptCreateHash(algorithm.0,&mut h,object.as_mut_ptr(),len,std::ptr::null(),0,0))?;let hash=Hash(h);
        let mut buf=vec![0u8;4*1024*1024];
        loop {let n=file.read(&mut buf).map_err(|_|"Cannot read the local model.")?;if n==0{break;}checked(BCryptHashData(hash.0,buf.as_ptr(),n as u32,0))?;}
        let mut digest=[0u8;32];checked(BCryptFinishHash(hash.0,digest.as_mut_ptr(),32,0))?;
        Ok(digest.iter().map(|b|format!("{b:02x}")).collect())
    }
}
fn random_key()->Result<String,String>{
    let mut bytes=[0u8;32];unsafe{checked(BCryptGenRandom(std::ptr::null_mut(),bytes.as_mut_ptr(),32,BCRYPT_USE_SYSTEM_PREFERRED_RNG))?;}
    Ok(bytes.iter().map(|b|format!("{b:02x}")).collect())
}
struct Http(*mut c_void);
impl Drop for Http{fn drop(&mut self){unsafe{WinHttpCloseHandle(self.0);}}}
fn handle(h:*mut c_void)->Result<Http,String>{if h.is_null(){Err("Cannot open the local HTTP connection.".into())}else{Ok(Http(h))}}
pub fn request(port:u16,path:&str,key:&str,body:Option<&[u8]>,timeout:i32)->Result<serde_json::Value,String>{
    if port==0||!matches!(path,"/health"|"/v1/chat/completions"){return Err("Invalid local endpoint.".into());}
    unsafe {
        let session=handle(WinHttpOpen(wide("LughaDesk/0.2").as_ptr(),WINHTTP_ACCESS_TYPE_NO_PROXY,std::ptr::null(),std::ptr::null(),0))?;
        if WinHttpSetTimeouts(session.0,2000,2000,timeout,timeout)==0{return Err("Cannot set local request limits.".into());}
        let connection=handle(WinHttpConnect(session.0,wide("127.0.0.1").as_ptr(),port,0))?;
        let verb=wide(if body.is_some(){"POST"}else{"GET"});
        let req=handle(WinHttpOpenRequest(connection.0,verb.as_ptr(),wide(path).as_ptr(),std::ptr::null(),std::ptr::null(),std::ptr::null(),0))?;
        for (option,value) in [(WINHTTP_OPTION_REDIRECT_POLICY,WINHTTP_OPTION_REDIRECT_POLICY_NEVER),(WINHTTP_OPTION_AUTOLOGON_POLICY,WINHTTP_AUTOLOGON_SECURITY_LEVEL_HIGH)]{
            if WinHttpSetOption(req.0,option,&value as *const _ as _,4)==0{return Err("Cannot enforce local HTTP privacy options.".into());}
        }
        let headers=wide(&format!("Content-Type: application/json\r\nAuthorization: Bearer {key}\r\n"));let bytes=body.unwrap_or(&[]);
        if WinHttpSendRequest(req.0,headers.as_ptr(),(headers.len()-1) as u32,if bytes.is_empty(){std::ptr::null()}else{bytes.as_ptr() as _},bytes.len() as u32,bytes.len() as u32,0)==0||WinHttpReceiveResponse(req.0,std::ptr::null_mut())==0{return Err("The local model did not respond. No online fallback was used.".into());}
        let mut status=0u32;let mut len=4;
        if WinHttpQueryHeaders(req.0,WINHTTP_QUERY_STATUS_CODE|WINHTTP_QUERY_FLAG_NUMBER,std::ptr::null(),&mut status as *mut _ as _,&mut len,std::ptr::null_mut())==0{return Err("Cannot read the local response status.".into());}
        if status!=200{return Err(format!("Local model returned HTTP {status}; try a shorter input."));}
        let mut result=Vec::new();let mut buf=[0u8;8192];loop {
            let mut count=0;if WinHttpReadData(req.0,buf.as_mut_ptr() as _,buf.len() as u32,&mut count)==0{return Err("Could not read the complete local response.".into());}
            if count==0{break;}result.extend_from_slice(&buf[..count as usize]);if result.len()>1_048_576{return Err("Local response exceeded its size limit.".into());}
        }
        serde_json::from_slice(&result).map_err(|_|"The local runtime returned invalid JSON.".into())
    }
}
struct Server(std::process::Child);
impl Drop for Server{fn drop(&mut self){let _=self.0.kill();let _=self.0.wait();}}
pub fn translate(dir:&Path,direction:&str,text:&str)->Result<String,String>{
    let data=serde_json::to_vec(&payload(direction,text)?).map_err(|_|"Cannot encode translation request.")?;
    let runtime=dir.join("runtime/llama-server.exe");let model=dir.join("model").join(MODEL_FILE);
    if !runtime.is_file(){return Err("Native runtime is missing. Extract the complete ZIP first.".into());}
    if sha256_file(&model)?!=MODEL_SHA{return Err("Model integrity check failed. Re-extract a trusted copy of the ZIP.".into());}
    let listener=TcpListener::bind((std::net::Ipv4Addr::LOCALHOST,0)).map_err(|_|"Local loopback access is blocked.")?;
    let port=listener.local_addr().map_err(|_|"Cannot allocate local port.")?.port();drop(listener);
    let key=random_key()?;
    let mut server=Server(Command::new(runtime).args(["--model"]).arg(model).args(["--offline","--host","127.0.0.1","--port",&port.to_string(),"--api-key",&key,
        "--threads","2","--threads-batch","2","--ctx-size","2048","--parallel","1","--n-gpu-layers","0","--reasoning","off","--no-webui","--alias","afrislm-local"])
        .env("HF_HUB_OFFLINE","1").env("LLAMA_ARG_OFFLINE","1").current_dir(dir.join("runtime"))
        .creation_flags(0x08000000).stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null()).spawn().map_err(|_|"Cannot start native runtime; keep all bundled DLLs beside it.")?);
    let deadline=Instant::now()+Duration::from_secs(120);
    loop{
        if server.0.try_wait().map_err(|_|"Cannot monitor native runtime.")?.is_some(){return Err("Native runtime exited while loading. Check the extracted DLLs and available memory.".into());}
        if let Ok(v)=request(port,"/health",&key,None,2000){if v["status"]=="ok"{break;}}
        if Instant::now()>deadline{return Err("Model loading/local connection timed out.".into());}std::thread::sleep(Duration::from_millis(250));
    }
    translation_from(&request(port,"/v1/chat/completions",&key,Some(&data),180000)?)
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn reject_remote_paths(){assert!(request(1234,"https://example.invalid/","x",None,1).is_err());assert!(request(0,"/health","x",None,1).is_err());}
    #[test]fn sha256_and_missing_file(){
        let p=std::env::temp_dir().join(format!("ruth-sha-fixture-{}.txt",std::process::id()));
        std::fs::write(&p,b"abc").unwrap();assert_eq!(sha256_file(&p).unwrap(),"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");std::fs::remove_file(&p).unwrap();assert!(sha256_file(&p).is_err());
    }
    #[test]fn key_is_random_hex(){let a=random_key().unwrap();let b=random_key().unwrap();assert_eq!(a.len(),64);assert_ne!(a,b);assert!(a.bytes().all(|c|c.is_ascii_hexdigit()));}
}
