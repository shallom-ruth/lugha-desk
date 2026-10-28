# Packaged release checklist

Component licenses are documented in LICENSING.md. The source is published under MIT. No packaged application download is currently published.

- Exclude the Microsoft runtime DLLs copied from System32 in the private evaluation bundle; use the official Microsoft free redistributable/prerequisite route or another verified authorized method.
- Include the Apache-2.0 model license and publisher attribution/provenance notices; keep teacher/dataset terms distinct.
- Preserve MIT and other native/runtime/dependency notices.
- Match assets to tested source and document actual language coverage and known meaning errors.
- Obtain the requested separate approval before uploading a packaged application release.
- Never commit weights, binaries, private logs, credentials or institutional details to source history.
