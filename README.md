# Lugha Desk

Offline translation for African languages, built in Rust for Windows x64. “Lugha” means language in Swahili. No Python or R installation is required by end users; R is used for testing. No cloud translation or paid API.

## Download and Windows setup

**[Download Lugha Desk 0.2.0 for Windows x64 — free ZIP (1.10 GB)](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.2.0/Lugha-Desk-0.2.0-Windows-x64.zip)**

[Release notes](https://github.com/shallom-ruth/lugha-desk/releases/tag/v0.2.0) · [SHA-256 checksum](https://github.com/shallom-ruth/lugha-desk/releases/download/v0.2.0/SHA256SUMS.txt)

1. Use Windows 10 or 11, x64. Download the ZIP above; it includes the model and native translation engine.
2. If needed, install the **[official Microsoft Visual C++ v14 Redistributable (x64)](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)** from Microsoft. Review its terms yourself. Microsoft runtime DLLs and its installer are not bundled.
3. Right-click the ZIP and select **Extract All**. Keep the entire extracted folder together, including `model/` and `runtime/`. Do not run the app inside the ZIP.
4. Open **`lugha-desk.exe`**. Translation runs offline after prerequisite setup; no Python, R, Rust, account or paid API is needed.

This is an **experimental pre-release** with known translation errors (see below). The executable is unsigned; keep Windows security protections enabled. GitHub’s separate “Source code” archives are for developers and do not include the ready-to-run model pack.

## Current capabilities

The installed-model integration offers English routes for 19 provider-listed African language options. The long-term goal is 120 distinct African languages; that coverage is not implemented.

Base R exercised 27 complete executions across 12 model language options. This establishes technical execution, not correct meaning. The test evidence predates the product-name change; inference settings are unchanged. Known failures include French output on an English→Wolof case, questionable Luganda output, and the Swahili borrowing error. See tests/native-smoke-results-2026-10-28.csv.

Gĩkũyũ, Dholuo, Kikamba and additional missing South African languages still need model integration. Additional language options are experimental and require qualified review. Do not use unreviewed translations for high-stakes decisions.

## Build from source

Use a Rust Windows x64 toolchain and its linker prerequisites:

```text
cargo test --locked
cargo build --release --locked
```

The executable is target/release/lugha-desk.exe. For offline translation, place model/ and runtime/ beside it as described below. No model weights or proprietary DLLs are stored in this repository.

## Model and native runtime

- Model: [QVAC TranslatePsy-AfriSLM-0.8B Q8](https://huggingface.co/qvac/TranslatePsy-AfriSLM-0.8B-Q8-GGUF), revision 55220bcc74bed219ad9db9ab378f6aae31850974. Place TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf in model/. Expected SHA-256 is checked in src/native.rs.
- Runtime: [llama.cpp b11932 Windows x64 CPU release](https://github.com/ggml-org/llama.cpp/releases/tag/b11932). Keep llama-server.exe and its required runtime DLLs in runtime/. Preserve vendor notices and follow applicable runtime prerequisites.
- The model publisher declares Apache-2.0 for the weights. Teacher and dataset terms are separate. See LICENSING.md for component licenses and packaging requirements.

## Privacy

No translation history is intentionally saved. The app uses an ephemeral authenticated 127.0.0.1 model server, no proxy, no redirects and no cloud fallback. Native --offline mode is not an OS firewall sandbox. Closing the app terminates its worker/model process tree.

## License and contribution

Original application source is MIT; see LICENSE. Third-party model/runtime/binary terms remain separate. See CONTRIBUTING.md. No affiliation, endorsement or translation-quality certification is claimed.

