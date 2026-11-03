#![windows_subsystem="windows"]
use std::{cell::RefCell,path::PathBuf,sync::{mpsc,Arc,atomic::{AtomicBool,Ordering}}};
use lugha_desk_launcher::setup::{self,wide,Progress};
mod shell;
use windows_sys::Win32::{Foundation::*,Graphics::Gdi::*,System::Threading::*,UI::{Controls::*,WindowsAndMessaging::*}};
use windows_sys::Win32::UI::Input::KeyboardAndMouse::EnableWindow;
enum Event{Progress(Progress),Finished(Result<bool,String>)}
struct Ui{root:PathBuf,status:HWND,details:HWND,bar:HWND,button:HWND,pause:HWND,runtime:HWND,ready:bool,busy:bool,closing:bool,cancel:Arc<AtomicBool>,rx:RefCell<Option<mpsc::Receiver<Event>>>}
fn preview()->bool{cfg!(debug_assertions)&&std::env::var_os("LUGHA_PREVIEW_ROOT").is_some()}
unsafe fn text(h:HWND,s:&str){SetWindowTextW(h,wide(s).as_ptr());}
unsafe fn control(parent:HWND,class:&str,label:&str,x:i32,y:i32,w:i32,h:i32,id:usize,style:u32)->HWND{
 let handle=CreateWindowExW(0,wide(class).as_ptr(),wide(label).as_ptr(),WS_CHILD|WS_VISIBLE|style,x,y,w,h,parent,id as HMENU,GetModuleHandleW(std::ptr::null()),std::ptr::null());
 SendMessageW(handle,WM_SETFONT,GetStockObject(DEFAULT_GUI_FONT) as usize,1);handle
}
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
unsafe fn begin(ui:&mut Ui,download:bool){
 if ui.busy{return}ui.busy=true;ui.ready=false;ui.cancel=Arc::new(AtomicBool::new(false));let cancel=ui.cancel.clone();let root=ui.root.clone();let (tx,rx)=mpsc::channel();ui.rx.replace(Some(rx));
 EnableWindow(ui.button,0);EnableWindow(ui.pause,download as i32);text(ui.status,if download{"Preparing your offline workspace…"}else{"Checking saved setup files…"});
 std::thread::spawn(move||{
  let result=(||{setup::prepare(&root)?;if setup::complete(&root)?{return Ok(true)}if !download{return Ok(false)}
   setup::download(&root,cancel,|p|{let _=tx.send(Event::Progress(p));})?;Ok(true)})();let _=tx.send(Event::Finished(result));
 });
}
unsafe extern "system" fn procedure(hwnd:HWND,msg:u32,wp:WPARAM,lp:LPARAM)->LRESULT{
 let ptr=GetWindowLongPtrW(hwnd,GWLP_USERDATA) as *mut Ui;
 if ptr.is_null(){return DefWindowProcW(hwnd,msg,wp,lp)}let ui=&mut *ptr;
 match msg{
  WM_COMMAND=>{match wp&65535{
   1=>if ui.ready{
    if !shell::runtime_ready(){text(ui.status,"One Windows component is needed. Use the Microsoft button, finish its setup, then try Open again.");EnableWindow(ui.runtime,1);return 0}
    match std::process::Command::new(ui.root.join("lugha-desk.exe")).current_dir(&ui.root).spawn(){Ok(_)=>{DestroyWindow(hwnd);},Err(e)=>text(ui.status,&format!("Could not open Lugha Desk: {e}"))}
   }else{begin(ui,true)},
   2=>{ui.cancel.store(true,Ordering::Relaxed);text(ui.status,"Pausing safely… Saved progress will resume next time.");EnableWindow(ui.pause,0);},
   3=>{if let Err(e)=shell::open_url("https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist"){text(ui.status,&e)}else{text(ui.status,"In Microsoft's page choose the x64 runtime, install it and approve its own prompts. Then return here and choose Open.");}},
   _=>{}
  }0},
  WM_TIMER=>{
   loop{let event=ui.rx.borrow().as_ref().map(|r|r.try_recv());match event{
    Some(Ok(Event::Progress(p)))=>{text(ui.status,&p.message);text(ui.details,&format!("{:.1} MB of {:.1} MB · {:.0}%",p.done as f64/1e6,p.total as f64/1e6,100.0*p.done as f64/p.total as f64));SendMessageW(ui.bar,PBM_SETPOS,((p.done as f64/p.total as f64)*1000.0) as usize,0);},
    Some(Ok(Event::Finished(result)))=>{
     ui.busy=false;ui.rx.replace(None);EnableWindow(ui.button,1);EnableWindow(ui.pause,0);
     match result{Ok(true)=>{ui.ready=true;text(ui.button,"Open Lugha Desk");SendMessageW(ui.bar,PBM_SETPOS,1000,0);
      text(ui.details,"Your model is verified. Translation works offline.");
      let note=if preview(){"Preview: model verified for offline use; no desktop or Start Menu changes.".to_owned()}else{match shell::finish(&ui.root){Ok(())=>"Ready. A Lugha Desk shortcut is on your desktop and Start Menu.".to_owned(),Err(e)=>format!("Ready for offline use. Shortcut creation was unavailable: {e}")}};
      text(ui.status,&note);
      if !shell::runtime_ready(){text(ui.status,"Model ready. Install Microsoft's free x64 Windows runtime using the button below, then choose Open.");EnableWindow(ui.runtime,1);}
     },Ok(false)=>{
      if !preview(){let _=shell::finish(&ui.root);}
      let saved=std::fs::metadata(ui.root.join(format!("model/{}.part",setup::MODEL_FILE))).map(|m|m.len().min(setup::MODEL_SIZE)).unwrap_or(0);
      text(ui.button,if saved>0{"Resume setup"}else{"Download model"});text(ui.status,"One-time setup: download the model, then translate offline.");
      if saved>0{text(ui.details,&format!("Saved: {:.1} MB of 1082.0 MB. Choose Resume setup. Allow at least 1.3 GB free space.",saved as f64/1e6));SendMessageW(ui.bar,PBM_SETPOS,(1000.0*saved as f64/setup::MODEL_SIZE as f64) as usize,0);}
      else{text(ui.details,"Model download: 1.08 GB. You can pause and resume. Allow at least 1.3 GB free space.");}
     },Err(e)=>{text(ui.status,&e);text(ui.button,"Resume setup");}}
     if ui.closing{DestroyWindow(hwnd);return 0}break
    },Some(Err(mpsc::TryRecvError::Disconnected))=>{ui.busy=false;ui.rx.replace(None);text(ui.status,"Setup worker stopped. Reopen the launcher to resume safely.");EnableWindow(ui.button,1);if ui.closing{DestroyWindow(hwnd);}break},_=>break
   }}0
  },
  WM_CLOSE=>{if ui.busy{ui.closing=true;ui.cancel.store(true,Ordering::Relaxed);text(ui.status,"Saving progress before closing…");EnableWindow(ui.button,0);EnableWindow(ui.pause,0);}else{DestroyWindow(hwnd);}0},
  WM_DESTROY=>{KillTimer(hwnd,1);SetWindowLongPtrW(hwnd,GWLP_USERDATA,0);drop(Box::from_raw(ptr));PostQuitMessage(0);0},
  _=>DefWindowProcW(hwnd,msg,wp,lp)
 }
}
fn run()->Result<(),String>{unsafe{
 let mutex=CreateMutexW(std::ptr::null(),0,wide("Local\\LughaDeskLauncherSetup03").as_ptr());if mutex.is_null(){return Err("Cannot reserve setup workspace.".into())}
 struct Mutex(HANDLE);impl Drop for Mutex{fn drop(&mut self){unsafe{CloseHandle(self.0);}}}let _mutex=Mutex(mutex);
 if GetLastError()==ERROR_ALREADY_EXISTS{return Err("Lugha Desk setup is already open. Continue in its existing window.".into())}
 let root=if cfg!(debug_assertions){std::env::var_os("LUGHA_PREVIEW_ROOT").map(PathBuf::from).map(Ok).unwrap_or_else(shell::root)?}else{shell::root()?};let _file_lock=setup::acquire_lock(&root)?;let instance=GetModuleHandleW(std::ptr::null());let class=wide("LughaDeskSetupWindow");
 let controls=INITCOMMONCONTROLSEX{dwSize:std::mem::size_of::<INITCOMMONCONTROLSEX>() as u32,dwICC:ICC_PROGRESS_CLASS};InitCommonControlsEx(&controls);
 let mut wc:WNDCLASSW=std::mem::zeroed();wc.hInstance=instance;wc.lpfnWndProc=Some(procedure);wc.lpszClassName=class.as_ptr();wc.hCursor=LoadCursorW(std::ptr::null_mut(),IDC_ARROW);wc.hbrBackground=(COLOR_WINDOW+1) as HBRUSH;
 if RegisterClassW(&wc)==0{return Err("Cannot create setup window class.".into())}
 let hwnd=CreateWindowExW(0,class.as_ptr(),wide("Lugha Desk — Offline setup").as_ptr(),WS_OVERLAPPED|WS_CAPTION|WS_SYSMENU|WS_MINIMIZEBOX,CW_USEDEFAULT,CW_USEDEFAULT,700,440,std::ptr::null_mut(),std::ptr::null_mut(),instance,std::ptr::null());
 if hwnd.is_null(){return Err("Cannot open setup window.".into())}
 control(hwnd,"STATIC","LUGHA DESK  /  YOUR OFFLINE LANGUAGE WORKSPACE",24,20,640,24,10,0);
 control(hwnd,"STATIC","Set up once. Translate without internet afterwards.",24,56,640,24,11,0);
 let status=control(hwnd,"STATIC","Checking setup…",24,102,640,55,12,0);
 let details=control(hwnd,"STATIC","",24,164,640,42,13,0);
 let bar=control(hwnd,"msctls_progress32","",24,212,640,22,14,0);SendMessageW(bar,PBM_SETRANGE32,0,1000);
 let button=control(hwnd,"BUTTON","Checking…",24,252,390,36,1,WS_TABSTOP|BS_DEFPUSHBUTTON as u32);
 let pause=control(hwnd,"BUTTON","Pause",434,252,230,36,2,WS_TABSTOP);
 let runtime=control(hwnd,"BUTTON","Microsoft Windows component (only if needed)",24,306,640,28,3,WS_TABSTOP);
 control(hwnd,"STATIC","Free · No account · Experimental translations: check meaning before use",24,350,640,24,15,0);
 let mut ui=Box::new(Ui{root,status,details,bar,button,pause,runtime,ready:false,busy:false,closing:false,cancel:Arc::new(AtomicBool::new(false)),rx:RefCell::new(None)});
 begin(&mut ui,false);SetWindowLongPtrW(hwnd,GWLP_USERDATA,Box::into_raw(ui) as isize);SetTimer(hwnd,1,100,None);ShowWindow(hwnd,SW_SHOW);
 let mut msg:MSG=std::mem::zeroed();loop{let result=GetMessageW(&mut msg,std::ptr::null_mut(),0,0);if result==0{break}if result<0{return Err("Setup message loop failed.".into())}if IsDialogMessageW(hwnd,&msg)==0{TranslateMessage(&msg);DispatchMessageW(&msg);}}Ok(())
}}
fn main(){if let Err(e)=run(){unsafe{MessageBoxW(std::ptr::null_mut(),wide(&e).as_ptr(),wide("Lugha Desk setup").as_ptr(),MB_OK|MB_ICONERROR);}}}
