# Lugha Desk Microsoft Store Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans for native execution, or superpowers:subagent-driven-development if the user chooses that approach. Steps use checkbox syntax for tracking.

**Goal:** Build a reviewable complete offline MSIX for the reserved Lugha Desk Store product, keeping free/open-source operation and the existing GitHub installer.

**Architecture:** Keep the Rust app, private local worker and pinned inference/model. Add a separate deterministic Store payload/manifest builder and manual Windows CI workflow; package the model beside the app and let Store manage deployment. Treat dependency compatibility, packaged execution and certification as verification gates.

**Tech Stack:** Rust 1.90.0 MSVC, Python standard library, PowerShell, Windows SDK MakeAppx/WACK, Microsoft VCLibs Desktop framework, existing pinned llama.cpp CPU archive and GGUF model.

**Spec:** `docs/superpowers/specs/2026-11-03-store-msix-design.md` (approved by Ruth on 3 November 2026).

## Global Constraints

- Full Windows x64 package, approximately 1.1 GB; model size 1,082,015,808 and SHA-256 `b5056635be0c9bb832f0c964dc063bd4b4462e253f9f9d94eebf0aa0960b7798`; use the existing pinned publisher revision/URL.
- Identity `RuthShallom.LughaDesk`; Publisher `CN=417167AF-8330-486F-8BD2-5C96DFDA0EE8`; display `Ruth Shallom`; Store ID `9NVZBTHPBR1C`; initial package version `1.0.0.0`.
- App/launcher preview versions `0.3.4-preview`; no inference changes, cloud fallback, analytics, automatic translation upload, paid API or translation login.
- Use supported Microsoft VCLibs Desktop framework dependency, never System32 copies; prove compatibility or block submission.
- No model weights, private contact information, credentials, ID data or test signing keys in Git. Retain component licenses and keep the public GitHub installer working.
- Store submission stays a reviewable draft until Ruth approves the final submission. No claims of certification, signing or tests that have not happened.

## Review Focus

- Read-only package paths and Unicode/space-containing installation paths must work: actual packaged native-worker smoke in Task 3.
- Store framework may not satisfy the pinned runtime’s imported symbols: import/version checks and packaged runtime startup in Tasks 2–3.
- A missing/corrupt/wrong model must stop packaging: model size/hash rejection tests in Task 2.
- Store install/update/removal must coexist with the existing EXE model: explicit lifecycle checklist in Task 3; no automatic migration/deletion.
- User feedback must remain voluntary and private by default: local dialog inspection and listing/privacy review in Tasks 1 and 3.

## Files and responsibilities

- `src/main.rs`: local coverage/support text only; retain existing inference paths.
- Root/launcher `Cargo.toml` and `Cargo.lock`: consistent next preview versions.
- `packaging/store/store-config.json`: exact product identity, Store version and model lock.
- `scripts/stage_store.py`: validate inputs, reuse runtime staging, produce allowed payload, manifest and build report.
- `scripts/create-store-assets.ps1`: generate readable app-name PNG assets with standard Windows drawing tools.
- `scripts/build-store.ps1`: source build, official framework discovery/validation, full model acquisition, SDK packaging and report.
- `.github/workflows/store-package.yml`: manual, read-only, action-SHA-pinned Windows build with short-lived review artifacts.
- `tests/test_stage_store.py`: payload/identity/model/private-file regression checks.
- `STORE_RELEASE.md`, `packaging/store/listing.md`, `packaging/store/privacy.md`: operator instructions, honest listing, voluntary-report privacy and lifecycle evidence.

### Task 1: Store-ready support information and preview source

**Interfaces:** Keep CLI/session worker interfaces unchanged. Coverage control remains command ID 9 and opens only a local information dialog.

- [ ] Update preview versions to `0.3.4-preview` consistently and verify Cargo lock package entries.
- [ ] Rename existing coverage control to “Coverage and support”; retain language coverage text and append public support email, privacy/source URLs and voluntary reporting instructions. No automatic network request or copied user translation text.
- [ ] Prepare exact listing/privacy copy: Free, experimental 19 language options, local generative model, known errors, full package download and offline operation. Retain all component notices.
- [ ] Run existing Rust/integration checks in Windows CI and inspect the actual dialog/UI. Do not write tests that merely mirror static wording.
- [ ] Commit the independently reviewable source/documentation change.

### Task 2: Validated MSIX payload and SDK build

**Interfaces:** `stage_store(app: Path, runtime_archive: Path, model: Path, notices: Path, assets: Path, output: Path, config: dict, framework: dict) -> dict` produces a fresh payload with `AppxManifest.xml` and a hash report. `config` comes from `packaging/store/store-config.json`. `framework` contains the exact official package name, publisher and validated version. Consume the existing `stage_bundle()` runtime checker rather than duplicating ZIP extraction.

- [ ] Add failing standard-library `unittest` cases: exact identity/publisher/architecture/direct entry point; model size/hash mismatch and missing model reject before final output; unexpected extra/private files and symlinks excluded/rejected; existing output not silently overwritten. Use small fixture model data with explicit fixture size/hash to exercise the same validation path.
- [ ] Run `python tests/test_stage_store.py` and confirm the new tests fail for the missing Store staging implementation.
- [ ] Implement `stage_store()` and CLI options for app/runtime/model/notices/assets/output/config/framework. Validate all inputs and write only to a fresh staging directory. Include allowed runtime/notices, model, generated assets, manifest and hash report; no EXE setup launcher or unrelated files.
- [ ] Generate standard required PNG logo sizes using app-name typography and verify their legibility visually. Use a full-trust desktop manifest with the exact identity, x64, `1.0.0.0`, direct `lugha-desk.exe` entry and supported VCLibs dependency. Document the selected supported Windows minimum build.
- [ ] Implement `scripts/build-store.ps1`: locked app tests/build, pinned runtime/model downloads and integrity checks, official framework metadata/import compatibility checks, PNG generation, staging and MakeAppx packaging. Record SDK/compiler/framework versions and source revision. Never suppress validation failures.
- [ ] Run unit tests (all pass), package with the real model, unpack the MSIX with SDK tooling and compare every payload hash and manifest identity. Failure to demonstrate runtime/framework compatibility blocks certification submission.
- [ ] Commit packaging, tests and operator documentation together.

### Task 3: CI artifact, packaged testing and reviewable Store draft

**Interfaces:** Manual CI invokes `scripts/build-store.ps1`; output is `dist/store/` containing unsigned Store-submission package, checksums, payload/build reports, listing and lifecycle checklist. Store supplies the production signature only after certification.

- [ ] Add manual `store-package.yml` using locked Rust, standard-library Python and commit-pinned checkout/setup/upload actions with contents-read permissions. Select a Windows runner with appropriate SDK/framework tooling, record versions and retain review artifacts briefly; no signing secrets or automatic publication.
- [ ] Push the Store branch, run workflow and inspect exact source SHA, completed tests and SDK validation. Download and verify package/artifact digests, payload hashes, bundled model/notice presence and exact Store identity.
- [ ] Run WACK and actual packaged install, direct activation, offline forward/reverse translation, dictionary/bypass checks, cancel/reopen, update, EXE coexistence and uninstall tests in an isolated suitable Windows environment. Report each observed result; do not count unpackaged or mocked tests as packaged execution. Obtain specific approval before any security/trust changes on Ruth’s main PC.
- [ ] If the available environment cannot perform packaged/WACK/lifecycle tests, provide the built artifact, precise remaining checks and the concrete environment blocker; keep submission unsent and continue independent preparation.
- [ ] Complete listing/privacy/support drafts, free pricing, factual age-rating and generative-model disclosure, screenshots from the actual app and certification instructions. Upload only a validated package into the existing reserved draft when ready; do not send for certification yet.
- [ ] Request one fresh whole-branch review, resolve findings and record rulings on deferred issues. Present package/test evidence and the completed submission draft for Ruth’s final review before any Microsoft certification submission.

## Plan self-review

All approved design sections map to the tasks above. Build interfaces use the same argument names throughout. Support text is a low-impact local UI change; meaningful tests focus on packaging boundaries and existing inference behavior. Framework compatibility and real package lifecycle are explicit gates, not assumptions or placeholders. No additional translation features or deployment channels are introduced.
