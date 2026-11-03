// Build-time Windows PE metadata. No runtime dependency or product behavior change.
use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let shared = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let shared = if shared.join("packaging/windows-resource.rs").is_file() {
        shared.join("packaging/windows-resource.rs")
    } else {
        shared.join("../packaging/windows-resource.rs")
    };
    println!("cargo:rerun-if-changed={}", shared.display());
    assert_eq!(env::var("CARGO_CFG_TARGET_ENV").unwrap(), "msvc",
        "Release metadata currently requires the Windows MSVC toolchain");
    let launcher = env::var("CARGO_PKG_NAME").unwrap() == "lugha-desk-launcher";
    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let numeric: Vec<u16> = version.split('-').next().unwrap().split('.')
        .map(|part| part.parse().expect("Invalid release version")).collect();
    assert_eq!(numeric.len(), 3);
    let numeric = format!("{},{},{},0", numeric[0], numeric[1], numeric[2]);
    let file = if launcher { "lugha-desk-launcher.exe" } else { "lugha-desk.exe" };
    let description = if launcher { "Lugha Desk Windows Launcher" } else { "Lugha Desk Offline Translation" };
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let source = out.join("lugha-version.rc");
    let resource = out.join("lugha-version.res");
    fs::write(&source, format!(r#"
1 VERSIONINFO
FILEVERSION {numeric}
PRODUCTVERSION {numeric}
FILEFLAGSMASK 0x3fL
FILEFLAGS 0x2L
FILEOS 0x40004L
FILETYPE 0x1L
FILESUBTYPE 0x0L
BEGIN
 BLOCK "StringFileInfo"
 BEGIN
  BLOCK "040904b0"
  BEGIN
   VALUE "CompanyName", "Ruth Shallom\0"
   VALUE "FileDescription", "{description}\0"
   VALUE "FileVersion", "{version}\0"
   VALUE "OriginalFilename", "{file}\0"
   VALUE "ProductName", "Lugha Desk\0"
   VALUE "ProductVersion", "{version}\0"
   VALUE "LegalCopyright", "Copyright 2026 Ruth Shallom\0"
  END
 END
 BLOCK "VarFileInfo"
 BEGIN
  VALUE "Translation", 0x0409, 1200
 END
END
"#)).unwrap();
    let sdk = PathBuf::from(env::var_os("ProgramFiles(x86)").expect("Windows SDK location missing"))
        .join("Windows Kits/10/bin");
    let mut candidates: Vec<PathBuf> = fs::read_dir(&sdk)
        .expect("Install the Windows SDK resource compiler")
        .filter_map(Result::ok).map(|entry| entry.path().join("x64/rc.exe"))
        .filter(|path| path.is_file()).collect();
    candidates.sort();
    let rc = candidates.last().expect("Windows SDK x64 rc.exe not found");
    let status = Command::new(rc).arg("/nologo").arg("/fo").arg(&resource)
        .arg(&source).status().expect("Could not start Windows resource compiler");
    assert!(status.success(), "Windows version-resource compilation failed");
    println!("cargo:rustc-link-arg-bins={}", resource.display());
}
