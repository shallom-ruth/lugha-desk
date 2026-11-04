# Microsoft Store build and submission

Lugha Desk's free Individual developer account is verified. Product `9NVZBTHPBR1C`
is reserved as an MSIX app and remains in draft. This document does not claim
Microsoft certification, a production signature or public Store availability.

While this workflow is still on the unmerged `store/msix` branch, run the existing
`Windows unsigned review build` workflow manually with that branch selected. Its
branch-only manual job invokes the Store workflow. Pull requests and ordinary
builds do not download the full model. Once merged, the separate `Microsoft Store
unsigned package` workflow can be dispatched directly.

## Build

Use Windows x64 with the pinned Rust toolchain, Python 3.13, the Windows SDK and
Visual C++ tools installed. Supply the official x64 VCLibs Desktop framework
package from Microsoft's Desktop framework SDK if automatic SDK discovery
cannot find it:

```powershell
./scripts/build-store.ps1 -FrameworkPackage C:\SDK\Microsoft.VCLibs.x64.14.00.Desktop.appx
```

The builder verifies Microsoft's framework signature/identity and uses a fresh
statically linked loader probe to check the actual pinned runtime against that
framework. A missing-framework negative test must fail; the supplied-framework
test must pass without substituting the machine's global VC DLLs. This is a
dependency-load check, not a substitute for actual packaged translation tests.

The app is built from locked source and tested. Runtime and model downloads use
the existing pinned URLs/checksums. The complete model must match its size/hash;
an explicitly supplied model is never overwritten. No weights or signing keys
belong in Git. End users need neither Rust nor Python.

The package is Windows x64, minimum Windows build `10.0.19041.0`, Store deployment
version `1.0.1.0`, app preview `0.3.4-preview`. The Store entry opens the app
directly with its packaged model/runtime; it does not run the GitHub EXE setup.
Store manages deployment/removal. Do not migrate or delete an existing GitHub
installation/model automatically.

`dist/store/` contains the unsigned review MSIX, checksums and reports. The SDK
validates, packs and unpacks it, then the verifier compares every allowed payload
file to its recorded hash and the exact reserved Store identity. Unsigned review
files are for Store submission/testing, not a trusted direct-download installer.
Microsoft supplies its production package signature only after approval.

## Required release evidence

Record observed results for the exact package and source SHA. Never infer these
from passing unpackaged tests:

- [ ] Pinned model/runtime hashes and compatible official framework confirmed.
- [ ] Actual MSIX unpacked, all allowed payload hashes and Store identity verified.
- [ ] WACK completed; report reviewed, material failures resolved.
- [ ] Installed/activated in a suitable isolated Windows environment.
- [ ] Offline forward/reverse translation, dictionary hits, model bypasses and Unicode tested.
- [ ] Loading/cancel, close/reopen and update tested.
- [ ] Existing GitHub setup/model unaffected; Store uninstall removes Store files cleanly.
- [ ] Actual Coverage and support dialog/Store graphics inspected.
- [ ] Listing/privacy/support URL, free pricing, AI disclosure and age ratings reviewed.
- [ ] Whole-branch review completed and findings resolved.
- [ ] Ruth approved the final certification submission.

The build report starts with WACK/lifecycle fields marked **not run**. Replace
those only with real results. If the test environment cannot perform packaged
execution, report the exact blocker and keep certification submission unsent.
Do not weaken protections on the maintainer's main Windows installation or
install a test certificate there without specific approval. Any isolated test
certificate must remain outside `dist/store/` and Git and must never be presented
as a Microsoft-issued signature.

## Store draft

Use the existing product and exact identity in `packaging/store/store-config.json`.
Use `packaging/store/listing.md` and `privacy.md` as reviewable drafts, not proof
that text has been published. Publish an accurate stable HTTPS privacy page
before submission and keep the app's privacy link accurate. Retain the component
licenses/attribution and disclose the local generative model and experimental
meaning errors. User feedback is voluntary; the app never automatically uploads
translation text. Keep price Free and do not add ads, purchases or subscriptions.

Upload only the validated artifact for final review. Do not click the final
certification submission until Ruth approves the completed package/listing and
test evidence. Keep the current GitHub Windows setup download available.
