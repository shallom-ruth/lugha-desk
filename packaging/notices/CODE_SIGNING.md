# Code signing policy

Public preview downloads are unsigned. Windows may show an unknown-publisher warning.

Public preview downloads, including v0.3.3-preview, are unsigned. A paid signing
subscription has not been purchased. Lugha Desk remains free, open source and
offline after model setup. Do not claim SignPath acceptance or endorsement.

The design below describes a possible future signing process if approved.

## People and approval

- Author, committer and reviewer: [Ruth Shallom (@shallom-ruth)](https://github.com/shallom-ruth).
- Release signing approver: Ruth Shallom (@shallom-ruth).
- Changes from outside contributors require maintainer review.
- Every production signing request requires explicit maintainer approval.
- Repository and SignPath users must use multi-factor authentication. The maintainer
  confirmed GitHub MFA is enabled; SignPath MFA must be enabled during onboarding.

## Build and signing boundaries

The Windows unsigned-review workflow builds the Rust application and launcher
from this repository, using locked Rust dependencies, a pinned toolchain,
commit-pinned GitHub Actions and an upstream runtime archive pinned by SHA-256.
It has read-only repository permissions, uses no signing secrets and does not
publish releases. Pull requests cannot request production signatures through it.
Its build artifacts are review builds and are explicitly marked unsigned.

When accepted, SignPath's assigned configuration must enforce the project's
product/version metadata and trusted build origin. Sign only Lugha Desk's own
application and launcher. Do not sign upstream executables or DLLs as Lugha code.
Sign and verify the app before compressing it into the launcher; then sign and
verify the completed launcher. Verify that extraction preserves the signed app
bytes and signature. Do not modify executables after signing. Generate release
checksums only from final signed files.

Production configuration, protected approval and signature validation must be
reviewed and tested before any release is described as signed. Once accepted,
this page and the homepage/download pages will carry the accurate credit:
“Free code signing provided by SignPath.io, certificate by SignPath Foundation”.
That sentence is a planned credit, not the status of current downloads.

## Privacy and installation

See [the privacy findings and limitations](README.md#privacy),
[installation/removal instructions](INSTALLATION.md) and
[component licensing](LICENSING.md). Translation runs against a local model server
on the same computer after the user requests the model download. Model setup
contacts Hugging Face and its download infrastructure. Providers receive normal
connection information; their [privacy policy](https://huggingface.co/privacy)
applies. Opening Microsoft or support/download sites uses those services and
their policies. Do not describe model setup as having no network activity.

## Verifying a future signed release

In Windows PowerShell, inspect each downloaded Lugha executable:

```powershell
Get-AuthenticodeSignature -LiteralPath .\LughaDeskLauncher.exe |
    Format-List Status,StatusMessage,SignerCertificate,TimeStamperCertificate
```

Use the actual downloaded filename. Confirm `Valid`, the announced signing
identity and a trusted timestamp. Existing unsigned releases show `NotSigned`.
An approved Foundation certificate would identify SignPath Foundation as the
certificate holder; metadata attributing the app to Ruth Shallom is separate.
A trusted signature establishes a publisher identity; it cannot promise that
SmartScreen reputation warnings disappear immediately.
