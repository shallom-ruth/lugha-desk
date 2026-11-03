---
license: apache-2.0
language:
- en
- af
- am
- ha
- ig
- rw
- ln
- lg
- mg
- ny
- om
- sn
- so
- st
- sw
- tn
- wo
- xh
- yo
- zu
datasets:
- qvac/TranslatePsy-AfriSLM-Synthetic-Mix
metrics:
- comet
- chrf
base_model:
- "qvac/TranslatePsy-AfriSLM-0.8B"
pipeline_tag: text-generation
tags:
- translation
- machine-translation
- african-languages
- multilingual
- conversational
- qwen3.5
- gguf
- llama.cpp
- quantized
- q8_0
- arxiv:2608.18655
---

# TranslatePsy-AfriSLM-0.8B-Q8-GGUF

TranslatePsy-AfriSLM-0.8B-Q8-GGUF is the Q8_0 GGUF quantization of [TranslatePsy-AfriSLM-0.8B](https://huggingface.co/qvac/TranslatePsy-AfriSLM-0.8B), a compact conversational machine-translation model specialized for English and 19 Sub-Saharan African languages. The quantization was created with an importance matrix for efficient local inference.

The model accompanies the EMNLP 2026 paper [*TranslatePsy-AfriSLM: High-Quality Data Scaling For Low-Resource Machine Translation*](https://arxiv.org/abs/2608.18655). The TranslatePsy-AfriSLM family outperforms substantially larger systems, including TranslateGemma-27B and Qwen3.5-122B-A10B, on the African machine-translation benchmarks reported in the paper.

![punching_above_its_weight](https://cdn-uploads.huggingface.co/production/uploads/642b07e6556ab448a07482d9/hnMsUcux3LWPRimK2o9Qr.png)

## Model Details

- **Developed by:** Milan Gritta, Patrik Lambert, Jihye Back, and Amril Nazir
- **Organization:** [Tether AI Research](https://tether.io/)
- **Shared by:** [QVAC](https://huggingface.co/qvac)
- **Model type:** GGUF Q8_0 quantization of a Qwen3.5 decoder-only conversational language model
- **Parameters:** approximately 0.8B
- **Languages:** English, Afrikaans, Amharic, Hausa, Igbo, Kinyarwanda, Lingala, Luganda, Malagasy, Nyanja, Oromo, Shona, Somali, Southern Sotho, Swahili, Tswana, Wolof, Xhosa, Yoruba, and Zulu
- **License:** Apache 2.0
- **Quantized from:** [`qvac/TranslatePsy-AfriSLM-0.8B`](https://huggingface.co/qvac/TranslatePsy-AfriSLM-0.8B)
- **Original base model:** [`Qwen/Qwen3.5-0.8B`](https://huggingface.co/Qwen/Qwen3.5-0.8B)
- **Paper:** [arXiv:2608.18655](https://arxiv.org/abs/2608.18655)
- **Code:** [tether-ai-research/qvac-translatepsy-afri-slm](https://github.com/tether-ai-research/qvac-translatepsy-afri-slm)
- **Dataset:** [`qvac/TranslatePsy-AfriSLM-Synthetic-Mix`](https://huggingface.co/datasets/qvac/TranslatePsy-AfriSLM-Synthetic-Mix)
- **Models and resources:** [TranslatePsy-AfriSLM collection](https://huggingface.co/collections/qvac/translatepsy-afrislm)

## Model Family

The paper evaluates three model sizes:

| Model | Base model | Flores-200 | BOUQuET | Smol |
|---|---|---:|---:|---:|
| TranslatePsy-AfriSLM-0.8B | Qwen3.5-0.8B | 0.5944 | 0.6223 | 0.4973 |
| TranslatePsy-AfriSLM-2B | Qwen3.5-2B | 0.6070 | 0.6322 | 0.5074 |
| TranslatePsy-AfriSLM-4B | Qwen3.5-4B | 0.6143 | 0.6391 | 0.5136 |

Scores are macro-averaged SSA-COMET results over the paper's 19 in-distribution African languages. See the paper for all metrics, translation directions, per-language results, and statistical tests.

### Quantized GGUF Results

Q4_K_M and Q8_0 GGUF quantizations are available for the 0.8B, 2B, and 4B variants:

| Model | Quantization | Flores-200 | BOUQuET | Smol |
|---|---|---:|---:|---:|
| TranslatePsy-AfriSLM-0.8B | Q8_0 | 0.5944 | 0.6207 | 0.4955 |
| TranslatePsy-AfriSLM-0.8B | Q4_K_M | 0.5886 | 0.6157 | 0.4905 |
| TranslatePsy-AfriSLM-2B | Q8_0 | 0.6071 | 0.6310 | 0.5063 |
| TranslatePsy-AfriSLM-2B | Q4_K_M | 0.6054 | 0.6299 | 0.5048 |
| TranslatePsy-AfriSLM-4B | Q8_0 | 0.6142 | 0.6384 | 0.5129 |
| TranslatePsy-AfriSLM-4B | Q4_K_M | 0.6138 | 0.6377 | 0.5121 |

These are macro-averaged SSA-COMET scores over the same 19 in-distribution African languages and both translation directions used in the full-precision table. The GGUF evaluations used a 2,048-token context. Quantized results may differ slightly from the full-precision results.

## Uses

### Direct Use

The model is intended for:

- translation between English and the supported African/European languages;
- conversational and multi-turn translation;
- research on low-resource and African machine translation;
- local or resource-constrained inference, especially with the GGUF variants.

### Out-of-Scope Use

The model was not designed or validated for autonomous use in medical, legal, emergency, immigration, financial, or other high-stakes contexts. It should not be used to infer sensitive characteristics, generate deceptive translations, or replace qualified human translators where errors could cause harm.

Translation quality is not guaranteed for unsupported languages, dialects, specialized domains, code-switching, very long documents, or text requiring extensive cultural context. The model's multimodal capabilities were not evaluated after fine-tuning.

## How to Get Started

This repository contains `TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf` for use with GGUF-compatible runtimes such as [llama.cpp](https://github.com/ggml-org/llama.cpp) and [llama-cpp-python](https://github.com/abetlen/llama-cpp-python).

### llama.cpp

Download the model and start a multi-turn conversation:

```bash
hf download qvac/TranslatePsy-AfriSLM-0.8B-Q8-GGUF \
  TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf \
  --local-dir .

llama-cli \
  --model TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf \
  --ctx-size 2048 \
  --conversation
```

### llama-cpp-python

```bash
python -m pip install --upgrade llama-cpp-python huggingface-hub
```

Use the paper's strict translation prompt for a single translation:

```python
from llama_cpp import Llama

llm = Llama.from_pretrained(
    repo_id="qvac/TranslatePsy-AfriSLM-0.8B-Q8-GGUF",
    filename="TranslatePsy-AfriSLM-0.8B-Q8_0-imat.gguf",
    n_ctx=2048,
    n_gpu_layers=-1,
    verbose=False,
)

source_lang = "English"
target_lang = "Swahili"
source_text = "How are you today?"

messages = [
    {
        "role": "system",
        "content": (
            f"You are a professional {source_lang} to {target_lang} translator. "
            f"Your goal is to accurately convey the meaning and nuances of the "
            f"original {source_lang} text while adhering to {target_lang} grammar, "
            f"vocabulary, and cultural sensitivities. Produce only the "
            f"{target_lang} translation, without any additional explanations "
            "or commentary. "
        ),
    },
    {
        "role": "user",
        "content": (
            f"Please translate the following {source_lang} text into "
            f"{target_lang}: {source_text}.\n\nTranslation:"
        ),
    },
]

response = llm.create_chat_completion(
    messages=messages,
    temperature=0.0,
    max_tokens=256,
)
print(response["choices"][0]["message"]["content"])
```

Set `n_gpu_layers=0` for CPU-only inference. For multi-turn chat, retain the message history and append each assistant response before sending the next user message.

## Training Details

### Quantization

This artifact is an importance-matrix-assisted `Q8_0` quantization of the full-precision TranslatePsy-AfriSLM-0.8B checkpoint. The quantized model was evaluated with a 2,048-token context.

### Training Data

The final training mixture combines:

- **TranslatePsy-AfriSLM Synthetic Mix:** quality-filtered synthetic parallel data for the 19 target African languages;
- **Instruct Mix:** multilingual instruction-following data, approximately half of which is African-language content, used to preserve conversational capabilities;
- **Asia-Europe Mix:** English-parallel data covering 38 Asian and European languages, used to reduce catastrophic forgetting outside the target African distribution.

The publicly released synthetic component contains 215,653,192 bidirectional examples, representing approximately 32.37B training tokens before combination with the auxiliary mixes. See the [dataset card](https://huggingface.co/datasets/qvac/TranslatePsy-AfriSLM-Synthetic-Mix) for its construction and limitations.

### Training Procedure

Models were full-parameter fine-tuned for one epoch. Training examples used the model chat template, and loss was computed only on assistant tokens. Sequences longer than 2,048 tokens were filtered; the remaining examples were packed with best-fit decreasing.

#### Hyperparameters

- peak learning rate: `1.25e-5`
- optimizer: fused AdamW
- learning-rate schedule: linear
- warmup: 1%
- global batch size: 256
- gradient clipping: 1.0
- precision: bfloat16
- gradient checkpointing: enabled
- distributed training: DeepSpeed ZeRO-2

#### Infrastructure

Experiments were run with PyTorch, Hugging Face Transformers, TRL, and DeepSpeed on 32 NVIDIA H100 GPUs.

## Evaluation

The models were evaluated in both English-to-African and African-to-English directions on:

- [FLORES-200](https://huggingface.co/datasets/facebook/flores), `devtest` split;
- [BOUQuET](https://huggingface.co/datasets/facebook/bouquet), `test` split;
- [SMOL](https://huggingface.co/datasets/google/smol), `smolsent` split.

Evaluation covered all 19 target African languages. The reported metrics were COMET-22, SSA-COMET, MetricX-24, and ChrF++. TranslatePsy-AfriSLM-0.8B-Q8-GGUF obtained SSA-COMET scores of **0.5944** on FLORES-200, **0.6207** on BOUQuET, and **0.4955** on SMOL.

Paired bootstrap tests in the paper show that the TranslatePsy-AfriSLM family significantly outperforms much larger general-purpose and translation-specialized baselines on most reported settings. Results are nevertheless based on automatic metrics; expert human evaluation remains necessary.

## Bias, Risks, and Limitations

- Training translations are primarily synthetic and may reproduce errors or biases from NLLB-3.3B and the filtering metrics.
- Automatic MT metrics may not reliably measure absolute quality for every supported language.
- Dialect provenance is unavailable, so standardized written forms may be overrepresented relative to regional dialects and oral traditions.
- Performance varies by language and translation direction; aggregate scores can hide weaker cases.
- Web-derived training data may contain harmful content, factual errors, demographic biases, or personal information.
- The paper does not include expert human evaluation, and the model may hallucinate, omit, or mistranslate content.
- Fine-tuning excluded reasoning data, so the model does not retain the base model's thinking capability.
- Quantization can introduce additional quality loss relative to the full-precision checkpoint; use the reported GGUF results rather than assuming identical performance.

Users should conduct native-speaker evaluation for the intended language, dialect, domain, and risk level. High-impact translations should always receive qualified human review.

## Licensing Information
This model which was fine-tuned as described in the [blog post](https://huggingface.co/blog/qvac/translate-psy-afrislm) is licensed by Tether Data, S.A. de C.V. under the [Apache 2.0 license](https://huggingface.co/datasets/choosealicense/licenses/blob/main/markdown/apache-2.0.md). As described in the blog post, this model is a version of the [Qwen3.5-0.8B model](https://huggingface.co/Qwen/Qwen3.5-0.8B), which is made available under the [Apache 2.0 license](https://huggingface.co/datasets/choosealicense/licenses/blob/main/markdown/apache-2.0.md). As described in the blog post, the [NLLB-200-3.3B model](https://huggingface.co/facebook/nllb-200-3.3B) which is made available under the [CC-BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/legalcode.en) license was used as a teacher model.
The [TranslatePsy-AfriSLM synthetic mix](https://huggingface.co/datasets/qvac/TranslatePsy-AfriSLM-Synthetic-Mix) is made available under the [CC-BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/legalcode.en) license. The [SmolTalk2 dataset](https://huggingface.co/datasets/HuggingFaceTB/smoltalk2) is made available under the [Apache 2.0](https://huggingface.co/datasets/choosealicense/licenses/blob/main/markdown/apache-2.0.md) license. The [Dolci-Instruct dataset](https://huggingface.co/datasets/allenai/Dolci-Instruct-SFT-No-Tools) is made available under the [Open Data Commons Attribution License](https://opendatacommons.org/licenses/by/1-0/) (ODC-By) v1.0 license. The [OPUS-100 dataset](https://huggingface.co/datasets/Helsinki-NLP/opus-100) is available on Hugging Face. As described in the blog post, the TranslatePsy-AfriSLM, SmolTalk2, Dolci-Instruct and OPUS-100 datasets were used as a part of fine-tuning the model.

## Citation

```bibtex
@misc{gritta2026translatepsyafrislmhighqualitydatascaling,
      title={TranslatePsy-AfriSLM: High-Quality Data Scaling For Low-Resource Machine Translation},
      author={Milan Gritta and Patrik Lambert and Jihye Back and Amril Nazir},
      institution={Tether Data, S.A. de C.V. d.b.a. Tether AI Research},
      year={2026},
      eprint={2608.18655},
      archivePrefix={arXiv},
      primaryClass={cs.CL},
      url={https://arxiv.org/abs/2608.18655},
}
```

**APA:** Gritta, M., Lambert, P., Back, J., & Nazir, A. (2026). *TranslatePsy-AfriSLM: High-Quality Data Scaling For Low-Resource Machine Translation*. arXiv preprint arXiv:2608.18655. https://arxiv.org/abs/2608.18655

## Model Card Contact

Questions and feedback can be submitted through the [project repository](https://github.com/tether-ai-research/qvac-translatepsy-afri-slm).
