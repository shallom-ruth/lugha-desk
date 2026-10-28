# Component licenses

Checked 28 October 2026. The application source license is confirmed as MIT; component licenses remain distinct.

| Component | Published license | Distribution requirements |
|---|---|---|
| Lugha Desk original Rust application source | MIT | Preserve the LICENSE copyright and permission notice. |
| llama.cpp native engine | MIT | Preserve its copyright and permission notice and applicable bundled dependency notices. |
| QVAC TranslatePsy-AfriSLM-0.8B Q8 model weights | Apache-2.0, expressly declared by Tether Data in the model card | Preserve Apache-2.0 license text, applicable notices/attributions and indicate modifications if any; do not imply endorsement. Current weights are unchanged. |
| LLVM OpenMP included with the native runtime | Apache-2.0 WITH LLVM-exception | Preserve the original LICENSE-LLVM-OpenMP file with the runtime. |
| windows-sys 0.61.2; windows-link 0.2.1; serde_json 1.0.154; serde_core 1.0.229; itoa 1.0.18 | MIT OR Apache-2.0 | Preserve the applicable license and notices. |
| zmij 1.0.23 | MIT | Preserve its copyright and permission notice. |
| memchr 2.8.3 | Unlicense OR MIT | Preserve the applicable license text. |
| Microsoft Visual C++ runtime | Microsoft Software License Terms | Do not assume installed System32 DLLs can be redistributed. Use an applicable official redistributable/prerequisite deployment route and its terms. This component is not relicensed MIT. |

## Model provenance is separate from the model license

The model card expressly states that the model is Apache-2.0 and separately identifies the NLLB teacher and synthetic dataset as CC-BY-NC-4.0, plus other training-data notices. These are different artifacts. Do not relabel the teacher/dataset Apache, or label the released weights CC-BY-NC merely because those artifacts were used. Preserve the publisher's provenance/attribution information. This records the publisher's terms; it is not a guarantee that no third-party claim can ever arise.

## Public Windows package requirements

The current private evaluation ZIP contains Microsoft runtime DLLs copied from the local Windows installation. That private ZIP must not be uploaded unchanged as the public release. A public package should omit those copied Microsoft DLLs and direct users to Microsoft's official free Visual C++ Redistributable as a prerequisite, or use a verified authorized redistribution method. No installation or license acceptance is performed on a user's behalf.

Include the app, native-runtime, model and dependency license/notice files with any authorized packaged distribution. Do not describe the entire bundle as exclusively MIT. There are no application license fees or paid translation APIs in this design. No packaged application release has been published.

## Primary sources

- [Application MIT license](LICENSE)
- [llama.cpp license](https://github.com/ggml-org/llama.cpp/blob/master/LICENSE)
- [Model licensing information](https://huggingface.co/qvac/TranslatePsy-AfriSLM-0.8B-Q8-GGUF#licensing-information)
- [Apache-2.0 text](https://www.apache.org/licenses/LICENSE-2.0)
- [Microsoft Visual C++ redistribution requirements](https://learn.microsoft.com/en-us/cpp/windows/redistributing-visual-cpp-files)
- [Official Microsoft redistributable downloads](https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist)
