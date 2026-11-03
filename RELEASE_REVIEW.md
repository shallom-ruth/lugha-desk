# Packaged release checklist

Component licenses are documented in LICENSING.md. Original application source is MIT. Experimental packaged downloads are published on GitHub Releases; public preview downloads, including v0.3.3-preview, are unsigned. SignPath Foundation declined the application at this time on 3 November 2026 because more independent public recognition is needed. CI artifacts are unsigned review builds; publishing an unsigned experimental preview requires explicit maintainer authorization. The checks below apply before describing any release as signed.

- Exclude the Microsoft runtime DLLs copied from System32 in the private evaluation bundle; use the official Microsoft free redistributable/prerequisite route or another verified authorized method.
- Include the Apache-2.0 model license and publisher attribution/provenance notices; keep teacher/dataset terms distinct.
- Preserve MIT and other native/runtime/dependency notices.
- Match assets to tested source and document actual language coverage and known meaning errors.
- Obtain the requested separate approval before uploading a packaged application release.
- Never commit weights, binaries, private logs, credentials or institutional details to source history.

## Before a signed release

- Obtain SignPath acceptance and verify the configured build-origin and executable metadata restrictions. See [Code signing policy](CODE_SIGNING.md).
- Confirm SignPath MFA and protected manual release approval; no production signing from untrusted PRs.
- Sign/verify the fresh app before regenerating launcher embedded resources; then build and sign/verify the launcher. Preserve upstream runtime binaries unchanged under their original identities.
- Verify both Authenticode chains and trusted timestamps; verify extracted app bytes still match the signed app.
- Pass app and launcher Rust tests, staging integrity tests, and clean-Windows installation/reopen/removal checks.
- Generate final checksums after signing; link the code signing policy from homepage and release/download pages. Claim SignPath credit only once accepted and accurate.
- Never replace the currently working unsigned release with an untested build.
