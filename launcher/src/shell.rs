use std::{ffi::{c_void,OsString},os::windows::ffi::{OsStrExt,OsStringExt},path::{Path,PathBuf}};
use windows_sys::{core::GUID,Win32::{System::Com::*,UI::{Shell::*,WindowsAndMessaging::*},Storage::FileSystem::*}};
use crate::setup::wide;
fn path_wide(p:&Path)->Vec<u16>{p.as_os_str().encode_wide().chain(Some(0)).collect()}
pub fn known(id:&GUID)->Result<PathBuf,String>{unsafe{
 let mut out=std::ptr::null_mut();if SHGetKnownFolderPath(id,0,std::ptr::null_mut(),&mut out)<0||out.is_null(){return Err("Windows could not locate your application folder.".into())}
 let mut n=0;while *out.add(n)!=0{n+=1}let path=PathBuf::from(OsString::from_wide(std::slice::from_raw_parts(out,n)));CoTaskMemFree(out as _);Ok(path)
}}
pub fn root()->Result<PathBuf,String>{Ok(known(&FOLDERID_LocalAppData)?.join("Programs/Lugha Desk Launcher"))}
pub fn open_url(url:&str)->Result<(),String>{unsafe{if ShellExecuteW(std::ptr::null_mut(),wide("open").as_ptr(),wide(url).as_ptr(),std::ptr::null(),std::ptr::null(),SW_SHOWNORMAL) as isize<=32{Err("Could not open the official Microsoft download. Visit learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist".into())}else{Ok(())}}}
pub fn runtime_ready()->bool{unsafe{
 let Ok(windows)=std::env::var("SystemRoot") else{return false};
 for dll in ["msvcp140.dll","vcruntime140.dll","vcruntime140_1.dll"]{
  let file=path_wide(&PathBuf::from(&windows).join("System32").join(dll));let mut ignored=0;
  let size=GetFileVersionInfoSizeW(file.as_ptr(),&mut ignored);if size==0||size>1024*1024{return false}
  let mut data=vec![0u8;size as usize];if GetFileVersionInfoW(file.as_ptr(),0,size,data.as_mut_ptr() as _)==0{return false}
  let mut info=std::ptr::null_mut();let mut len=0;
  if VerQueryValueW(data.as_ptr() as _,wide("\\").as_ptr(),&mut info,&mut len)==0||info.is_null()||len<(std::mem::size_of::<VS_FIXEDFILEINFO>() as u32){return false}
  let v=std::ptr::read_unaligned(info as *const VS_FIXEDFILEINFO);
  if v.dwSignature!=0xfeef04bd||(v.dwFileVersionMS>>16,v.dwFileVersionMS&65535,v.dwFileVersionLS>>16,v.dwFileVersionLS&65535)<(14,50,35719,0){return false}
 }true
}}
unsafe fn method<T:Copy>(object:*mut c_void,index:usize)->T{let vtable=*(object as *mut *mut *mut c_void);std::mem::transmute_copy(&*vtable.add(index))}
unsafe fn release(object:*mut c_void){let f:unsafe extern "system" fn(*mut c_void)->u32=method(object,2);f(object);}
pub fn shortcut(target:&Path,link:&Path,working:&Path)->Result<(),String>{unsafe{
 let hr=CoInitializeEx(std::ptr::null(),COINIT_APARTMENTTHREADED as u32);if hr<0{return Err("Windows shortcuts are unavailable in this environment.".into())}
 struct Com;impl Drop for Com{fn drop(&mut self){unsafe{CoUninitialize()}}}let _com=Com;
 let cls=GUID::from_u128(0x00021401_0000_0000_c000_000000000046);let iid=GUID::from_u128(0x000214f9_0000_0000_c000_000000000046);
 let mut object=std::ptr::null_mut();if CoCreateInstance(&cls,std::ptr::null_mut(),CLSCTX_INPROC_SERVER,&iid,&mut object)<0{return Err("Windows could not create the app shortcut.".into())}
 struct Obj(*mut c_void);impl Drop for Obj{fn drop(&mut self){unsafe{release(self.0)}}}let object=Obj(object);
 let set_path:unsafe extern "system" fn(*mut c_void,*const u16)->i32=method(object.0,20);
 let set_dir:unsafe extern "system" fn(*mut c_void,*const u16)->i32=method(object.0,9);
 if set_path(object.0,path_wide(target).as_ptr())<0||set_dir(object.0,path_wide(working).as_ptr())<0{return Err("Could not set shortcut target.".into())}
 let query:unsafe extern "system" fn(*mut c_void,*const GUID,*mut *mut c_void)->i32=method(object.0,0);
 let mut persist=std::ptr::null_mut();if query(object.0,&GUID::from_u128(0x0000010b_0000_0000_c000_000000000046),&mut persist)<0{return Err("Could not save shortcut.".into())}
 let persist=Obj(persist);let save:unsafe extern "system" fn(*mut c_void,*const u16,i32)->i32=method(persist.0,6);
 if save(persist.0,path_wide(link).as_ptr(),1)<0{return Err("Windows refused to save the shortcut.".into())}Ok(())
}}
pub fn finish(root:&Path)->Result<(),String>{
 let source=std::env::current_exe().map_err(|e|e.to_string())?;let installed=root.join("LughaDeskLauncher.exe");
 if source!=installed{std::fs::copy(source,&installed).map_err(|e|format!("Cannot save the launcher: {e}"))?;}
 let programs=known(&FOLDERID_Programs)?.join("Lugha Desk");std::fs::create_dir_all(&programs).map_err(|e|e.to_string())?;
 shortcut(&installed,&programs.join("Lugha Desk.lnk"),root)?;
 shortcut(&installed,&known(&FOLDERID_Desktop)?.join("Lugha Desk.lnk"),root)
}

#[cfg(test)]mod tests{
 use super::*;
 #[test]fn windows_runtime_probe_is_non_mutating(){let _=runtime_ready();}
 #[test]fn unicode_known_folder_available(){assert!(root().unwrap().is_absolute());}
 #[test]fn creates_real_windows_shortcut_in_test_folder(){
  let folder=std::env::temp_dir().join(format!("lugha-shortcut-{}",std::process::id()));std::fs::create_dir_all(&folder).unwrap();
  let target=std::env::current_exe().unwrap();let link=folder.join("Lugha Desk test.lnk");shortcut(&target,&link,&folder).unwrap();
  let data=std::fs::read(link).unwrap();assert!(data.len()>76);assert_eq!(&data[..4],&[76,0,0,0]);
 }
}
