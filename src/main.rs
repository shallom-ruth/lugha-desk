#![windows_subsystem = "windows"]
use std::{cell::RefCell,io::Write, path::PathBuf, process::{Command,Stdio}, sync::{mpsc,Arc,atomic::{AtomicBool,Ordering}}, time::{Duration,Instant}};
use std::os::windows::{io::AsRawHandle,process::CommandExt};
use lugha_desk::{validate,wide,routes};
mod native;
use windows_sys::Win32::{Foundation::*,Graphics::Gdi::*,System::{JobObjects::*,LibraryLoader::GetModuleHandleW},UI::WindowsAndMessaging::*};
use windows_sys::Win32::UI::{Controls::EM_SETLIMITTEXT,Input::KeyboardAndMouse::{EnableWindow,SetFocus}};

fn app_dir()->Result<PathBuf,String> {
    let exe=std::env::current_exe().map_err(|_|"Cannot locate application.")?;
    exe.parent().map(|p|p.to_owned()).ok_or("Cannot locate the extracted application folder.".into())
}
struct Job(HANDLE);
impl Drop for Job {fn drop(&mut self){unsafe{CloseHandle(self.0);}}}
fn run(direction:&str,text:&str,cancel:Arc<AtomicBool>)->Result<String,String> {
    validate(direction,text)?;
    let mut child=Command::new(std::env::current_exe().map_err(|_|"Cannot locate this executable.")?)
        .arg("--native-worker").arg(direction)
        .creation_flags(0x08000000).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
        .spawn().map_err(|_|"Cannot start the native Rust worker.")?;
    // Worker waits for stdin EOF: it cannot spawn the model before job assignment.
    let job_result=unsafe {
        let h=CreateJobObjectW(std::ptr::null(),std::ptr::null());
        if h.is_null(){None}else{
            let job=Job(h);
            let mut limits:JOBOBJECT_EXTENDED_LIMIT_INFORMATION=std::mem::zeroed();
            limits.BasicLimitInformation.LimitFlags=JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE;
            if SetInformationJobObject(h,JobObjectExtendedLimitInformation,&limits as *const _ as _,std::mem::size_of_val(&limits) as u32)==0
                || AssignProcessToJobObject(h,child.as_raw_handle() as HANDLE)==0 {None}else{Some(job)}
        }
    };
    let _job=match job_result {Some(j)=>j,None=>{let _=child.kill();let _=child.wait();return Err("Windows could not isolate the model process. Translation was not started.".into());}};
    let sent=child.stdin.take().ok_or("Worker input unavailable.")?.write_all(text.as_bytes());
    if sent.is_err(){return Err("Cannot pass text to the local worker.".into());}
    let deadline=Instant::now()+Duration::from_secs(330);
    loop {
        if cancel.load(Ordering::Relaxed){return Err("Translation cancelled.".into());}
        if Instant::now()>deadline{return Err("Translation timed out. Try a shorter sentence.".into());}
        match child.try_wait(){Ok(Some(_))=>break,Ok(None)=>std::thread::sleep(Duration::from_millis(100)),Err(_)=>return Err("Cannot monitor local inference.".into())}
    }
    let out=child.wait_with_output().map_err(|_|"Cannot read local inference output.")?;
    if !out.status.success(){
        let message=String::from_utf8_lossy(&out.stderr).trim().to_owned();
        return Err(if message.is_empty(){"Local translation failed.".into()}else{message});
    }
    let result=String::from_utf8(out.stdout).map_err(|_|"Invalid UTF-8 output.")?;
    if result.trim().is_empty(){return Err("The model returned no translation.".into());}
    Ok(result)
}

struct Ui {direction:HWND,input:HWND,output:HWND,button:HWND,coverage:HWND,status:HWND,warning:HWND,in_label:HWND,out_label:HWND,
    receiver:RefCell<Option<mpsc::Receiver<Result<String,String>>>>,cancel:RefCell<Arc<AtomicBool>>}
unsafe fn text(h:HWND,s:&str){SetWindowTextW(h,wide(s).as_ptr());}
unsafe fn control(parent:HWND,class:&str,value:&str,style:u32,id:usize)->HWND {
    let h=CreateWindowExW(0,wide(class).as_ptr(),wide(value).as_ptr(),WS_CHILD|WS_VISIBLE|style,0,0,10,10,parent,id as HMENU,GetModuleHandleW(std::ptr::null()),std::ptr::null());
    if !h.is_null(){SendMessageW(h,WM_SETFONT,GetStockObject(DEFAULT_GUI_FONT) as usize,1);}
    h
}
unsafe fn layout(hwnd:HWND,u:&Ui){
    let mut r:RECT=std::mem::zeroed();GetClientRect(hwnd,&mut r);let w=r.right.max(650);let h=r.bottom.max(500);
    MoveWindow(u.direction,24,22,w-240,260,1);MoveWindow(u.button,w-190,20,166,34,1);
    MoveWindow(u.warning,24,66,w-48,50,1);
    MoveWindow(u.in_label,24,124,w-48,22,1);MoveWindow(u.out_label,24,158+(h-260)/2,w-48,22,1);
    MoveWindow(u.input,24,150,w-48,(h-270)/2,1);MoveWindow(u.output,24,184+(h-260)/2,w-48,(h-270)/2,1);
    MoveWindow(u.status,24,h-40,w-255,32,1);MoveWindow(u.coverage,w-225,h-42,201,30,1);
}
unsafe extern "system" fn wndproc(hwnd:HWND,msg:u32,wp:usize,lp:isize)->isize {
    if msg==WM_GETMINMAXINFO {
        let limits=&mut *(lp as *mut MINMAXINFO);
        limits.ptMinTrackSize=POINT{x:720,y:580};return 0;
    }
    let ptr=GetWindowLongPtrW(hwnd,GWLP_USERDATA) as *mut Ui;
    if msg==WM_DESTROY {
        if !ptr.is_null(){let u=Box::from_raw(ptr);u.cancel.borrow().store(true,Ordering::Relaxed);SetWindowLongPtrW(hwnd,GWLP_USERDATA,0);}
        PostQuitMessage(0);return 0;
    }
    if ptr.is_null(){return DefWindowProcW(hwnd,msg,wp,lp);}
    // Win32 control calls may synchronously reenter this procedure. Never hold
    // exclusive Ui references or RefCell borrows across those calls.
    let u=&*ptr;
    match msg {
        WM_SIZE=>{layout(hwnd,u);0},
        WM_COMMAND if wp&0xffff==9 && (wp>>16)==0=>{
            MessageBoxW(hwnd,wide("KENYA\nSwahili and Somali: model installed; quality not certified.\nGĩkũyũ, Dholuo, Kikamba: native model integration pending.\nOther Kenyan language varieties need individual model checks.\n\nSOUTH AFRICA\nisiZulu, isiXhosa, Sesotho, Setswana, Afrikaans: installed experimental routes.\nSepedi, Xitsonga, Tshivenda, siSwati, Southern isiNdebele: model integration pending.\n\n19 model language options are installed. 120 distinct African languages is the long-term goal. A menu entry or a fluent output does not establish translation accuracy.").as_ptr(),wide("Language coverage — installed versus pending").as_ptr(),MB_OK);0
        },
        WM_COMMAND if wp&0xffff==1 && (wp>>16)==0=>{
            if u.receiver.borrow().is_some(){return 0;}
            let n=GetWindowTextLengthW(u.input) as usize;let mut buf=vec![0u16;n+1];GetWindowTextW(u.input,buf.as_mut_ptr(),buf.len() as i32);
            let source=String::from_utf16_lossy(&buf[..n]);
            let catalogue=routes();let index=SendMessageW(u.direction,CB_GETCURSEL,0,0);
            let Some((direction,_))=catalogue.get(index as usize).cloned()else{text(u.status,"Select an installed language route.");return 0;};
            if let Err(e)=validate(&direction,&source){text(u.status,&e);return 0;}
            let (tx,rx)=mpsc::channel();u.receiver.replace(Some(rx));let cancel=Arc::new(AtomicBool::new(false));u.cancel.replace(cancel.clone());
            text(u.output,"");text(u.status,"Loading and translating locally… Please wait. Close the window to cancel.");
            EnableWindow(u.button,0);EnableWindow(u.direction,0);EnableWindow(u.input,0);
            std::thread::spawn(move||{let result=run(&direction,&source,cancel);let _=tx.send(result);});0
        },
        WM_TIMER=>{
            let result=u.receiver.borrow().as_ref().map(|rx|rx.try_recv());
            let completed=match result{Some(Ok(r))=>Some(r),Some(Err(mpsc::TryRecvError::Disconnected))=>Some(Err("Translation worker stopped unexpectedly.".into())),_=>None};
            if let Some(result)=completed {
                u.receiver.replace(None);
                match result{Ok(s)=>{text(u.output,&s);text(u.status,"Done locally. Check the meaning carefully before using this translation.");},Err(e)=>text(u.status,&e)}
                EnableWindow(u.button,1);EnableWindow(u.direction,1);EnableWindow(u.input,1);
            }0
        },
        _=>DefWindowProcW(hwnd,msg,wp,lp)
    }
}
unsafe fn gui()->Result<(),String>{
    let class=wide("LughaDeskWindow");let instance=GetModuleHandleW(std::ptr::null());
    let mut wc:WNDCLASSW=std::mem::zeroed();wc.lpfnWndProc=Some(wndproc);wc.hInstance=instance;wc.lpszClassName=class.as_ptr();
    wc.hCursor=LoadCursorW(std::ptr::null_mut(),IDC_ARROW);wc.hbrBackground=(COLOR_WINDOW+1) as HBRUSH;
    if RegisterClassW(&wc)==0{return Err("Cannot register the application window.".into());}
    let hwnd=CreateWindowExW(0,class.as_ptr(),wide("Lugha Desk").as_ptr(),WS_OVERLAPPEDWINDOW,
        CW_USEDEFAULT,CW_USEDEFAULT,900,720,std::ptr::null_mut(),std::ptr::null_mut(),instance,std::ptr::null());
    if hwnd.is_null(){return Err("Cannot create the application window.".into());}
    let direction=control(hwnd,"COMBOBOX","",WS_TABSTOP|0x3|WS_VSCROLL,2);
    for (_,label) in routes(){SendMessageW(direction,CB_ADDSTRING,0,wide(&label).as_ptr() as isize);}
    SendMessageW(direction,CB_SETCURSEL,0,0);
    let input=control(hwnd,"EDIT","",WS_TABSTOP|WS_BORDER|WS_VSCROLL|4|64|4096,3);
    SendMessageW(input,EM_SETLIMITTEXT,1200,0);
    let output=control(hwnd,"EDIT","",WS_TABSTOP|WS_BORDER|WS_VSCROLL|4|64|2048,4);
    let u=Box::new(Ui {direction,input,output,button:control(hwnd,"BUTTON","Translate offline",WS_TABSTOP,1),
        coverage:control(hwnd,"BUTTON","Kenya / South Africa coverage",WS_TABSTOP,9),
        warning:control(hwnd,"STATIC","Experimental model: meaning errors remain. Additional languages are not independently validated.\r\n19 model language options installed; 120 African languages is a future goal. See coverage for missing local languages.",0,5),
        status:control(hwnd,"STATIC","Python-free · max 600 characters · no history saved",0,6),
        in_label:control(hwnd,"STATIC","Your text",0,7),out_label:control(hwnd,"STATIC","Translation — review before use",0,8),
        receiver:RefCell::new(None),cancel:RefCell::new(Arc::new(AtomicBool::new(false)))});
    if [u.direction,u.input,u.output,u.button,u.coverage,u.warning,u.status,u.in_label,u.out_label].iter().any(|h|h.is_null()){DestroyWindow(hwnd);return Err("Cannot create application controls.".into());}
    let ptr=Box::into_raw(u);SetWindowLongPtrW(hwnd,GWLP_USERDATA,ptr as isize);layout(hwnd,&*ptr);SetTimer(hwnd,1,150,None);
    ShowWindow(hwnd,SW_SHOW);SetFocus(input);
    let mut msg:MSG=std::mem::zeroed();loop {let r=GetMessageW(&mut msg,std::ptr::null_mut(),0,0);if r==0{break;}if r==-1{return Err("Windows message loop failed.".into());}if IsDialogMessageW(hwnd,&msg)==0{TranslateMessage(&msg);DispatchMessageW(&msg);}}
    Ok(())
}
fn main(){
    let args:Vec<String>=std::env::args().collect();
    if args.get(1).map(String::as_str)==Some("--native-worker") {
        use std::io::Read;let mut source=String::new();
        if std::io::stdin().take(8193).read_to_string(&mut source).is_err(){eprintln!("Invalid UTF-8 input.");std::process::exit(2);}
        let result=app_dir().and_then(|dir|native::translate(&dir,args.get(2).map(String::as_str).unwrap_or(""),&source));
        match result{Ok(s)=>print!("{s}"),Err(e)=>{eprintln!("{e}");std::process::exit(2);}}return;
    }
    if args.get(1).map(String::as_str)==Some("--list-routes") {
        println!("{}",serde_json::to_string(&routes()).unwrap());return;
    }
    if args.get(1).map(String::as_str)==Some("--translate") {
        use std::io::Read;let mut source=String::new();
        if std::io::stdin().take(8193).read_to_string(&mut source).is_err(){eprintln!("Invalid UTF-8 input.");std::process::exit(2);}
        match run(args.get(2).map(String::as_str).unwrap_or(""),&source,Arc::new(AtomicBool::new(false))){Ok(s)=>print!("{s}"),Err(e)=>{eprintln!("{e}");std::process::exit(2);}}
    }else if let Err(e)=unsafe{gui()}{unsafe{MessageBoxW(std::ptr::null_mut(),wide(&e).as_ptr(),wide("Offline Translation").as_ptr(),MB_OK|MB_ICONERROR);}}
}
