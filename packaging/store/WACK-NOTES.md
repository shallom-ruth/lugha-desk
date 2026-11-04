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

## Verified native rerun and replacement identity

The saved 4 November rerun XML reports `RuthShallom.LughaDesk` 1.0.0.0,
`PARTIAL_RUN=FALSE`, overall PASS and DPIAwarenessValidation PASS. Its saved
run context identifies the input as the DPI-FIX MSIX, SHA256
`0fd9fe132a7ff64c52293156a898d8133168515a938630e224321080a432d04d`.
The report filename alone did not identify this input correctly. The DPI trace
explicitly recognized PerMonitorV2. There are 23 PASS results and the optional
Blocked executables FAIL remains; overall PASS does not mean every optional
test passed or that Microsoft has waived the finding.

Partner Center rejected uploading changed contents under the old 1.0.0.0
identity. The final replacement therefore uses 1.0.1.0, source
`f307d83b3f68c9f95e162b47d484732e3dbb4495`, MSIX SHA256
`e32c72b029b0fbe48d9e3b7ac894b93a6ebfb1a454a1e86627a1bb6f81eeba16`.
Build 41855066249 and downloaded deployment report 41863738757 cover that
package; all 57 payload hashes and its embedded EXE manifest were verified
locally. Its EXE hash differs from the native WACK-tested build, so the above
report must not be described as an exact native 1.0.1.0 WACK run. Native
installed-package GUI validation also remains separate from the ARM-hosted
deployment tests and prior native unpackaged GUI checks.
