# Windows App Certification Kit findings

The native x64 WACK 10.0.26100.9457 run on 4 November 2026 inspected
`RuthShallom.LughaDesk` 1.0.0.0. It completed all selected tests (`PARTIAL_RUN=FALSE`):
22 PASS, one optional Blocked executables FAIL, and one DPI-awareness WARNING.
Report saving failed in the wizard; the complete `tempReport.xml` was recovered.
These results are local checks, not Microsoft Store approval.

## Optional blocked-executable scan

[Microsoft's Desktop Bridge test guidance](https://learn.microsoft.com/en-us/windows/uwp/debug-test-perf/windows-desktop-bridge-app-tests#42-blocked-executables)
distinguishes external executable launches from packaged components. The test
is informational for Store onboarding. Do not disable the test, patch model
bytes, rename DLLs, or change the report to obtain a green result.

The app's production process-launch sites are:

- `src/worker.rs`: absolute current app executable, fixed `--session-worker` argument.
- `src/native.rs`: the bundled `runtime/llama-server.exe`, fixed local/offline
  server options and pinned bundled model. Text is sent through the local API,
  not inserted into a shell command. The server and worker run in a kill-on-close job.

There is no app call to launch cmd, PowerShell, reg, bash or another shell.
Rust's Windows standard library includes batch-file handling in its generic
`Command` implementation, explaining a `cmd.exe` string in the linked app;
the app's targets above are `.exe` files, not batch scripts.
[Rust's Windows process implementation](https://github.com/rust-lang/rust/blob/1.90.0/library/std/src/sys/process/windows.rs)
documents that generic support.

The scanner also matched command-like strings inside the GGUF model, the
`CODE_SIGNING.md` prose, and bundled inference DLLs. A vocabulary string or a
raw binary match does not establish that a command was launched. The GGUF is
data read by the inference engine, and CODE_SIGNING.md is documentation.
The model and runtime must retain their pinned hashes and third-party notices.

WACK also reports CreateProcessW imports in packaged mtmd/llama-common DLLs.
This is an import/reference scan, not a process-execution trace. The supported
application path sends text-only inference requests; it does not expose a
shell/tool-execution feature. This audit does not claim that every unused
upstream runtime feature has been independently assessed or certify Windows S mode.

Expected disposition: retain and disclose the optional scan result with this
explanation for Microsoft review. Do not describe the entire WACK run as clean
or the warning as waived by Microsoft.

## DPI warning

The inspected app had no embedded DPI manifest and fixed-pixel layout. The fix
adds an embedded PerMonitorV2/asInvoker manifest, converts logical layout values
at the Win32 boundary, scales control fonts and minimum/window dimensions,
and handles WM_DPICHANGED using Windows' suggested window rectangle.

Verification includes the actual linked executable's embedded manifest, Rust
geometry tests from 100% through 300%, and normal translation regressions.
Native visual and WACK rerun evidence must be recorded against the rebuilt
artifact before treating this warning as resolved in the submission.
