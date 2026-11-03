# Windows installation and removal

Lugha Desk is experimental Windows 10/11 x64 software. Current published downloads
are unsigned; see the [code signing policy](CODE_SIGNING.md).

## Installation changes

The launcher saves app files, runtime files, notices and its downloaded model in:

`%LOCALAPPDATA%\Programs\Lugha Desk Launcher`

The model is a separate user-requested download of approximately 1.08 GB. A paused
download is saved in that directory. After its initial local setup check, even
before the model is downloaded, the launcher attempts to save a copy of itself
as `LughaDeskLauncher.exe` and create `Lugha Desk.lnk` on the user's
Windows Desktop and inside the user's Start Menu Programs `Lugha Desk` folder.
Windows known folders are used, so the Desktop can be redirected, for example to
OneDrive. Setup uses a lock file named `.Lugha Desk Launcher.setup.lock` beside
the installation folder. It does not register an Apps & Features uninstaller.

Microsoft Visual C++ x64 runtime is an external prerequisite. If needed, the
launcher opens [Microsoft's official page](https://learn.microsoft.com/cpp/windows/latest-supported-vc-redist)
and the user completes Microsoft's installer separately. No copied Windows
system DLLs are bundled by this build process.

## Removal

1. Close the Lugha Desk app and launcher; allow the local model process to exit.
2. Open `%LOCALAPPDATA%\Programs` in File Explorer. Check that `Lugha Desk Launcher`
   is the Lugha installation folder, then delete that folder. This also removes
   the downloaded model and any paused download, so reinstalling needs a new
   model download unless you saved it separately.
3. Delete the `Lugha Desk` shortcut from your actual Windows Desktop and the
   `Lugha Desk` folder under the user's Start Menu Programs folder. Open the
   latter using `shell:programs` in Windows Run.
4. If present, remove the closed launcher's `.Lugha Desk Launcher.setup.lock`
   file beside the installation folder. Remove the original downloaded launcher
   or ZIP from Downloads if you no longer need it.

Do not remove unrelated files or Windows/Microsoft runtime components. Microsoft
Visual C++ may be required by other programs and is managed separately in Windows.

These paths are derived from launcher source. Clean-Windows installation,
reopen and removal checks remain required before publishing a newly signed build.
