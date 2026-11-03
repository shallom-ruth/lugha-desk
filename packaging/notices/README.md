# Lugha Desk

Developed by Ruth Shallom.

[Project website: download, tests and limitations](https://shallom-ruth.github.io/lugha-desk.html)

Lugha Desk is free, open-source Windows software developed by Ruth Shallom for offline text translation involving African languages. It currently offers 19 experimental language options; accuracy is still being evaluated. Built in Rust. “Lugha” means language in Swahili. No Python or R installation is required by end users; R is used for testing. No cloud translation or paid API.

## Code signing policy

See [CODE_SIGNING.md](CODE_SIGNING.md) for maintainer roles, build provenance,
privacy links and the proposed SignPath signing process. Public preview downloads remain unsigned. Windows may show an unknown-publisher warning. See [installation and removal](INSTALLATION.md) for setup changes.

The `Windows unsigned review build` GitHub Actions workflow builds both the app
and the recovered launcher from this repository. Review artifacts are unsigned
and are not automatically published as releases.

To build both on Windows with the Visual C++ build tools and Windows SDK installed:

```powershell
./scripts/build-windows.ps1
```

The pinned Rust toolchain and Python 3.13 are developer prerequisites. The build
downloads and verifies the pinned runtime, generates launcher resources and puts
review artifacts in `dist/unsigned-<version>/`. Python and Rust are not needed by
end users. The launcher packaging script does not embed model weights.

## Download and Windows setup

[Download the free Windows launcher, about 21.6 MB](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.3.3-preview/Lugha-Desk-Launcher-Windows-x64.exe)

[Release notes and checksum](https://github.com/shallom-ruth/lugha-desk/releases/tag/v0.3.3-preview)

1. Close Lugha Desk and any setup window, then open the new launcher on Windows 10 or 11, x64. A complete existing model is verified and reused while the app is updated.

2. Choose Download model. This is a one time 1.08 GB download. You can Pause and Resume setup. Allow at least 1.3 GB free disk space.

3. If prompted, use the Microsoft button to install the free [official Visual C++ x64 runtime](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist). Complete Microsoft’s setup yourself and return to Lugha Desk.

4. Choose Open Lugha Desk. Afterwards, use the Lugha Desk desktop or Start Menu shortcut. Translation works offline with the saved model.

No Python, R, Rust installation, account or paid API is needed. Loading progress and Cancel are available. The first short translation took about 10 to 12 seconds on the test laptop, with repeat translations taking about 5 to 7 seconds. Other computers and longer text may take longer.

This is an experimental preview with known translation errors. The executable is unsigned. Keep Windows security protections enabled.

[Previous ZIP download, retained as a fallback](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.2.0/Lugha-Desk-0.2.0-Windows-x64.zip). Extract the entire ZIP before opening its app and keep the model and runtime folders together. Do not run inside the ZIP.

Greeting dictionary

The v0.3.3-preview release extends the six earlier greeting entries with four phrase pairs for Wolof, Luganda and Yoruba. Complete standalone greetings use a small reference-checked dictionary; longer sentences continue through the existing offline model. New entries await fluent-speaker review and are included in the v0.3.3-preview download above. Lingala entries are excluded pending review. No model retraining was performed, and general translation errors remain possible. See [GREETING_DICTIONARY.md](GREETING_DICTIONARY.md) for exact entries, sources, matching rules and tests.

Previous launcher: https://github.com/shallom-ruth/lugha-desk/releases/tag/v0.3.0-preview

Borrowing and lending dictionary

This candidate adds three user-reviewed sentence pairs, six directional entries. It does not retrain or replace the model.

I borrowed a ladder from Rehema.
Niliazima ngazi kutoka kwa Rehema.

I lent a ladder to Rehema.
Nilimwazimisha Rehema ngazi.

Rehema has returned the ladder.
Rehema amerudisha ngazi.

Each pair works in both directions between English and Swahili. Matching covers the whole message and preserves case. Surrounding or repeated whitespace and one optional final full stop are tolerated. Different names, negation, questions, quantities and longer sentences are not dictionary matches. No automatic name substitution or individual-word replacement occurs.

Matched phrases use the reviewed dictionary without starting the model. Unmatched text uses the existing offline model and can still contain meaning errors, including borrowing and lending errors. These six entries do not establish general translation accuracy. Existing reference-checked greetings remain; provisional Lingala greetings remain excluded.

Review basis: Ruth approved G02-01, G02-02 and G02-04 in this project. This is user-reviewed correction data, not an independently validated language benchmark. No development examples or unreviewed phrases are included.

Previous greeting launcher: https://github.com/shallom-ruth/lugha-desk/releases/tag/v0.3.1-preview

## Current capabilities

The installed-model integration offers English routes for 19 provider-listed African language options. The long-term goal is 120 distinct African languages; that coverage is not implemented.

Base R exercised 27 complete executions across 12 model language options. This establishes technical execution, not correct meaning. The test evidence predates the product-name change; inference settings are unchanged. Known failures include French output on an English→Wolof case, questionable Luganda output, and the Swahili borrowing error. See tests/native-smoke-results-2026-10-28.csv.

Gĩkũyũ, Dholuo, Kikamba and additional missing South African languages still need model integration. Additional language options are experimental and require qualified review. Do not use unreviewed translations for high-stakes decisions.

The corrected launcher passed 13 normal Rust tests and its completed model reopen test. The corrected app passed 15 Rust tests and six base R startup and reopen translation checks. These checks do not certify translation meaning.

## Build from source

Matching launcher and app source, including generated runtime resources, is available in [v0.3.3 matching source archive](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.3.3-preview/Lugha-Desk-0.3.3-preview-Source.zip). This developer archive is not needed to install the app. GitHub’s automatic source archives do not contain the ready to run launcher.

Use a Rust Windows x64 toolchain and its linker prerequisites:

```text
cargo test --locked
cargo build --release --locked
```

The executable is target/release/lugha-desk.exe. For offline translation, place model/ and runtime/ beside it as described below. No model weights or proprietary DLLs are stored in this repository.

## Model and native runtime

Model: [QVAC TranslatePsy-AfriSLM-0.8B Q8](https://huggingface.co/qvac/TranslatePsy-AfriSLM-0.8B-Q8-GGUF), revision 55220bcc74bed219ad9db9ab378f6aae31850974. Place TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf in model/. Expected SHA-256 is checked in src/native.rs.
Runtime: [llama.cpp b11932 Windows x64 CPU release](https://github.com/ggml-org/llama.cpp/releases/tag/b11932). Keep llama-server.exe and its required runtime DLLs in runtime/. Preserve vendor notices and follow applicable runtime prerequisites.
The model publisher declares Apache-2.0 for the weights. Teacher and dataset terms are separate. See LICENSING.md for component licenses and packaging requirements.

## Privacy

Lugha Desk translates text on your computer after the model has been downloaded. The reviewed v0.3.2 app code sends translation requests to a local model server on the same computer, with no online translation fallback. We found no analytics, account registration or automatic translation-history saving in that code.

Setup needs internet access to download the model from Hugging Face. Download providers can receive normal connection information, such as your IP address. Opening download or support websites also uses your browser and those websites’ services.

During a translation test on 2 November 2026, the connection-monitor screenshots showed only local connections for the monitored app processes. This was a limited TCP check, not a complete security audit: brief connections, other network protocols and third-party component behaviour were not fully ruled out. The earlier packet capture was inconclusive. Translation text exists temporarily in memory, and Windows may manage memory or crash data separately.

These findings apply to the reviewed v0.3.2 preview and do not guarantee every future version or certify the app as malware-free.

## License and contribution

Original application source is MIT; see LICENSE. Third-party model/runtime/binary terms remain separate. See CONTRIBUTING.md. No affiliation, endorsement or translation-quality certification is claimed.


