# Lugha Desk Microsoft Store design

Proposed for approval, 3 November 2026. Base source: `56907708d6aec9ceb894199e39f8467d6c63eee3`.

## Purpose and agreed choices

Make Lugha Desk easy to install from Microsoft Store while retaining free pricing, open source and local offline translation. Ruth chose a complete app-and-model package, approximately 1.1 GB. The free Individual developer account is created and verified. The name Lugha Desk is reserved as an MSIX/PWA app in draft; it has not been published or certified.

## Approaches

- **Complete MSIX with model — selected.** One Store installation contains the app, native engine, model and notices. Translation works offline immediately after installation. Larger initial download; Store manages app installation and updates.
- Smaller MSIX with first-use model download — not selected. Smaller first download, but adds setup and requires separate writable model storage and recovery paths.
- Existing EXE submission — unsuitable for the free signing goal because Microsoft does not re-sign that installer. Keep the current GitHub Windows installer as a separate distribution option.

## Package and app behavior

1. Produce a Windows x64 full-trust desktop MSIX. Its Start menu entry opens the app directly. The current EXE setup launcher is not the Store entry point: it must not copy an app outside the package or create its own shortcuts during Store use.
2. Include the existing allowlisted CPU runtime, all notices and the unchanged pinned model at `model/TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf`. Verify model size 1,082,015,808 bytes and SHA-256 `b5056635be0c9bb832f0c964dc063bd4b4462e253f9f9d94eebf0aa0960b7798`. Fetch weights only into ignored build staging/artifacts, never Git history.
3. Retain the app-relative runtime/model layout so packaged files can stay read-only. Preserve the existing private local worker, model integrity checks, authenticated loopback server, cancellation and no-cloud-fallback behavior. No translation account, subscription, paid API, analytics or automatic text upload is added.
4. Resolve the native engine’s C++ runtime using Microsoft's supported `Microsoft.VCLibs.140.00.UWPDesktop` Store framework dependency. The builder must establish a compatible framework version from the actual pinned binaries and validate it through packaged execution. Do not copy System32 DLLs. If compatibility cannot be demonstrated, block submission and report the dependency issue rather than publishing a broken package or silently replacing the runtime.
5. Add a small local “Coverage and support” dialog using the existing coverage control. Retain language coverage information and add the support email, privacy/source links and instructions to report incorrect or inappropriate output voluntarily. Opening the dialog sends nothing; users choose whether and what to share. No automatic feedback collection or translation upload.
6. Keep app/launcher source versions consistent at the next preview version, `0.3.4-preview`, for the support-text change. Store package version starts at `1.0.0.0` as a separate deployment version; this is not a claim that experimental translation accuracy is complete. Existing inference settings and greeting/phrase corrections remain.
7. Store and GitHub installations coexist. Do not migrate, delete or modify a saved GitHub model/setup automatically. Store installation/removal handles its own packaged files. Original app/launcher source stays MIT; bundled components keep their own licenses.

## Exact Store identity

Read from the reserved product in Partner Center:

- Package identity name: `RuthShallom.LughaDesk`
- Publisher: `CN=417167AF-8330-486F-8BD2-5C96DFDA0EE8`
- Publisher display name: `Ruth Shallom`
- Package family name: `RuthShallom.LughaDesk_mj1zfg7cgz4xj`
- Store ID: `9NVZBTHPBR1C`

These are package identifiers, not signing secrets. No personal phone/address/identity documents belong in source or build artifacts.

## Build and listing

Create a separate manually triggered, pinned Windows CI packaging workflow using locked Rust dependencies and the existing runtime checksum lock. Build and stage from source, verify model/runtime hashes, create Store graphics from the app name, generate the identity-matched manifest and package with the Windows SDK. Use no production signing keys; Microsoft re-signs approved Store MSIX submissions. Review artifacts are not public Store releases and should have short retention.

Use accurate English listing text: experimental English routes with 19 African-language options, known meaning errors, local AI-generated translations, approximately 1.1 GB download and offline operation after installation. Price is Free; no purchases or ads. Include component licensing, support and a privacy page explaining local processing and voluntary feedback. Complete the generative-AI disclosure and age-rating questions accurately; no certification or quality guarantees. The builder may choose appropriate icon sizes, layout details and an appropriate supported minimum Windows build, documenting those choices.

## Verification and completion

Run existing Rust, phrase/greeting integration and staging tests; add meaningful packaging checks for exact identity, dependency declarations, hashes, allowed files, excluded private data and complete notices. Validate/unpack the actual MSIX and compare its payload hashes. Run the Windows App Certification Kit and actual packaged install/launch/offline translation/cancel/reopen/update/coexistence/removal checks on a suitable Windows test machine. CI-only EXE tests do not prove packaged behavior. Do not weaken security on Ruth’s main Windows installation to perform testing; use an isolated test environment or obtain specific approval for any required trust/security change.

Prepare the Store submission as a reviewable draft with package, screenshots, support/privacy links, free pricing, disclosures, ratings and certification notes. Clearly report tests that cannot be completed and block certification submission until material failures are resolved. Ruth reviews the final submission before it is sent to Microsoft. Store approval is not guaranteed.

## Sources

- [Microsoft MSIX requirements](https://learn.microsoft.com/en-us/windows/apps/publish/publish-your-app/msix/app-package-requirements)
- [C++ runtime framework deployment](https://learn.microsoft.com/en-us/troubleshoot/developer/visualstudio/cpp/libraries/c-runtime-packages-desktop-bridge)
- [Packaged desktop behavior](https://learn.microsoft.com/en-us/windows/msix/desktop/desktop-to-uwp-behind-the-scenes)
- [Microsoft Store policies](https://learn.microsoft.com/en-us/windows/apps/publish/store-policies)
