# Lugha Desk

Offline translation for African languages, built in Rust for Windows x64. “Lugha” means language in Swahili. No Python or R installation is required by end users; R is used for testing. No cloud translation or paid API.

## Download and Windows setup

[Download the free Windows launcher, about 22.5 MB](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.3.0-preview/Lugha-Desk-Launcher-0.3.0-startup-fix-Windows-x64.exe)

[Release notes and checksum](https://github.com/shallom-ruth/lugha-desk/releases/tag/v0.3.0-preview)

1. Open the downloaded launcher on Windows 10 or 11, x64.

2. Choose Download model. This is a one time 1.08 GB download. You can Pause and Resume setup. Allow at least 1.3 GB free disk space.

3. If prompted, use the Microsoft button to install the free [official Visual C++ x64 runtime](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist). Complete Microsoft’s setup yourself and return to Lugha Desk.

4. Choose Open Lugha Desk. Afterwards, use the Lugha Desk desktop or Start Menu shortcut. Translation works offline with the saved model.

No Python, R, Rust installation, account or paid API is needed. Loading progress and Cancel are available. The first short translation took about 10 to 12 seconds on the test laptop, with repeat translations taking about 5 to 7 seconds. Other computers and longer text may take longer.

This is an experimental preview with known translation errors. The executable is unsigned. Keep Windows security protections enabled.

[Previous ZIP download, retained as a fallback](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.2.0/Lugha-Desk-0.2.0-Windows-x64.zip). Extract the entire ZIP before opening its app and keep the model and runtime folders together. Do not run inside the ZIP.

## Current capabilities

The installed-model integration offers English routes for 19 provider-listed African language options. The long-term goal is 120 distinct African languages; that coverage is not implemented.

Base R exercised 27 complete executions across 12 model language options. This establishes technical execution, not correct meaning. The test evidence predates the product-name change; inference settings are unchanged. Known failures include French output on an English→Wolof case, questionable Luganda output, and the Swahili borrowing error. See tests/native-smoke-results-2026-10-28.csv.

Gĩkũyũ, Dholuo, Kikamba and additional missing South African languages still need model integration. Additional language options are experimental and require qualified review. Do not use unreviewed translations for high-stakes decisions.

The corrected launcher passed 13 normal Rust tests and its completed model reopen test. The corrected app passed 15 Rust tests and six base R startup and reopen translation checks. These checks do not certify translation meaning.

## Build from source

Matching launcher and app source, including generated runtime resources, is available in [Lugha Desk 0.3.0 Source](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.3.0-preview/Lugha-Desk-0.3.0-Source.zip). This developer archive is not needed to install the app. GitHub’s automatic source archives do not contain the ready to run launcher.

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

No translation history is intentionally saved. The app uses an ephemeral authenticated 127.0.0.1 model server, no proxy, no redirects and no cloud fallback. Native --offline mode is not an OS firewall sandbox. Closing the app terminates its worker/model process tree.

## License and contribution

Original application source is MIT; see LICENSE. Third-party model/runtime/binary terms remain separate. See CONTRIBUTING.md. No affiliation, endorsement or translation-quality certification is claimed.

