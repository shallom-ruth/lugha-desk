# Launcher build provenance

The launcher source and `pack_assets.py` were recovered from the published
v0.3.2-preview `Lugha-Desk-Phrase-Candidate-Source.zip` source asset.
All six app Rust source files in that archive match repository main at
ddcb419fb1c03e83f4869fbc89d6f933e1fe3b3b byte-for-byte.

`notices/` preserves the release's license/model-provenance text resources.
Recovery decompressed the original resources using Windows cabinet.dll and
verified each against the SHA-256 in the original launcher `src/assets.rs`.
The original compressed app/runtime binaries are not source inputs here.
The relative references in THIRD_PARTY_NOTICES.md were adjusted for this layout.
APP_LICENSING.md and INSTALLATION.md record the current packaging documentation.

The build uses the app produced from this checkout and the SHA-256-pinned
llama.cpp b11932 archive in scripts/runtime-lock.json. Only the 22 required
runtime filenames are staged. Remaining upstream command-line tools are omitted.
Runtime bytes are not recompiled, rewritten or signed as Lugha code.

The model is not embedded or fetched by CI. Its user-requested download remains
pinned in launcher/src/setup.rs and validated by the launcher before use.
Generated assets.rs and compressed assets are build outputs, not checked-in
source. No paid signing or translation service is configured by this change.
