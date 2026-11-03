# TranslatePsy-AfriSLM: High-Quality Data Scaling for Low-Resource Machine Translation

📄 [Paper (arXiv)](https://arxiv.org/abs/2608.18655) · 💻 [GitHub](https://github.com/tether-ai-research/qvac-translatepsy-afri-slm) · 🤗 [Hugging Face Collection](https://huggingface.co/collections/qvac/translatepsy-afrislm)

The rapid progress in Artificial Intelligence has largely bypassed African languages, creating a digital divide that limits AI adoption for over a billion people across the continent [1], with **Machine Translation (MT)** being a key resource for communication, education and business. To help bridge this gap, we introduce **TranslatePsy-AfriSLM**: an open-source suite of machine translation resources covering 19 Sub-Saharan African languages. In our **forthcoming EMNLP 2026 paper**, we demonstrate how our rigorous quality-estimation filtering and synthetic data generation allow SLMs with **as few as 0.8B parameters** to rival, and often outperform, frontier open-source LLMs and specialised translation models on African languages.


![plot](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/B6PCFYsSf4ngFUzgwfSCg.png)
<p align="center">
  Figure 1: SSA-COMET on Flores-200, BOUQuET, and Smol. Even our smallest model, TranslatePsy-AfriSLM-0.8B, matches or outperforms general-purpose, dedicated, and African-specialised systems up to 152× its size (e.g. Qwen3.5-122B-A10B, TranslateGemma-27B) on all three benchmarks.
</p>

## 🚀 Key Highlights

- **SLMs Beat Frontier Behemoths:** Our TranslatePsy-AfriSLM-0.8B outperforms much larger systems, including Qwen3.5-122B-A10B, TranslateGemma-27B, and NLLB-3.3B across major African machine translation benchmarks.
- **19 African Languages:** TranslatePsy-AfriSLM covers 19 African languages including languages that remain relatively underrepresented in existing AI machine translation models: Afrikaans, Amharic, Hausa, Igbo, Kinyarwanda, Lingala, Luganda, Malagasy, Nyanja, Oromo, Shona, Somali, Southern Sotho, Swahili, Tswana, Wolof, Xhosa, Yoruba, and Zulu.
- **Up to 96% Fewer Training Tokens:** Our unified quality estimation (QE) filtering filters noisy and redundant training data, reducing the open-source training budget from 44.93B to 1.76B tokens, a 96% reduction without degrading translation performance.
- **Synthetic Data Superiority:** Our high-quality synthetic parallel data dominates open-source corpora across the quality-efficiency Pareto frontier showing that data quality and efficiency matters more than simply scaling data volume.
- **Conversational Translation:** TranslatePsy-AfriSLM retains multi-turn translation, language identification, and cross-lingual conversational capabilities, going beyond rigid sentence-level translation systems.
- **Run on a laptop or smartphone:** We additionally share **GGUF models** for **llama.cpp**, which are quantized models for on-device deployment, providing further efficiency and convenience. No need for expensive GPUs, internet access or advanced hardware!
- **Open-Source Release**: Tether Data, S.A. de C.V. (Tether AI Research, we, us, our) releases the **model weights, inference code and data-processing scripts** under the Apache 2.0 license for research and educational purposes. The **synthetic training data** is released under CC-BY-NC 4.0 (Creative Commons Attribution–NonCommercial 4.0) for research and educational purposes.

**Copyright Complaints**: We will take appropriate actions in response to notice of copyright infringement. If you believe your work has been used or copied in a manner that infringes upon your intellectual property rights, please email data-apps@tether.io identifying and describing both the copyrighted work and alleged infringing content to file a notice of infringement.



---

## 🌍 Motivation: Why African MT Is a Data Problem

Machine translation is critical for communication, education, and cross-border collaboration across Africa, yet even large open-source LLMs remain relatively weak in many African languages. Adapting smaller models is promising, but the main bottleneck is high-quality training data at sufficient scale.

Existing African MT resources face a persistent trade-off between scale, coverage, and quality:

- **Large but noisy corpora:** OPUS, MALA, WMT22, and Fine Translations provide millions of parallel sentences, but contain substantial duplication, imbalance, and noisy translations.
- **Curated datasets:** AfriNLLB offers a cleaner supervision, but covers fewer languages and remains relatively limited in scale.
- **Human-quality datasets:** SMOL, AfriDOC-MT, MMT-Africa, and LAFAND-MT provide strong training signals per example, but are too small to drive large-scale SLM adaptation on their own.

This leads to our central question: How can we scale our training data budget while maintaining high quality?

## 🛠️ Methodology: Building a High-Quality Training Pipeline

TranslatePsy-AfriSLM is built around a simple idea: scale high-quality training signals rather than just raw volume.  Our data curation pipeline starts from large-scale open-source and synthetic parallel data, then progressively cleans, scores, and filters it to retain the most useful training signal for African language machine translation.


![data_pipeline](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/M0zneG4_UaaMD7aUnBePx.png)
<p align="center">
  Figure 2: The TranslatePsy-AfriSLM data pipeline. Raw parallel and monolingual sources are cleaned, expanded with NLLB-3.3B synthetic generation, and filtered by a Unified QE module before data-quantity selection produces the final training mixtures.
</p>

### 1. Data Sourcing & Preprocessing

We build two large parallel-data pools:

- **Open-source data:** We collect **427M raw sentence pairs across 19 African languages** from WMT22 [1], MALA [11], OPUS [10], and Fine Translations [12]. *NOTE: this data mixture was used for research purposes only and will not be publicly released, however, we are releasing our [synthetic data mixture](https://huggingface.co/datasets/qvac/TranslatePsy-AfriSLM-Synthetic-Mix) listed below (our most performant training dataset).*
- **Synthetic data:** starting from MADLAD-400 [13] monolingual text, we generate English ↔ African language parallel data using **NLLB-3.3B** via CTranslate2 [9].

**Cleaning & Decontamination:** Both undergo language-ID filtering, document and sentence cleaning, exact/fuzzy deduplication, and strict decontamination against all evaluation sets.

### 2. Unified Quality Estimation (QE)

Quality estimators are models that have been trained to evaluate the quality of a translation. We found that no single QE estimator is consistently optimal across evaluation metrics. In our ablations, data selected by each estimator tends to perform particularly well under its corresponding evaluation metric. However, in order to obtain a more robust filtering signal, we combine **AfriCOMET** [8], **SSA-COMET** [4], and **MetricX-24** [3] into a unified score.

Since these estimators differ in both scale and polarity, we calibrate their raw scores against approximately 352K human-translated sentence pairs using a robust z-score based on the median and median absolute deviation (MAD):

$$
\mathrm{MAD}_{d,m}
=
\operatorname{median}\left(
\left|x_i - \tilde{x}_{d,m}\right|
\right)
$$

$$
z_{i,d,m}
=
s_m \cdot
\frac{
0.6745 \left(x_{i,d,m} - \tilde{x}_{d,m}\right)
}{
\mathrm{MAD}_{d,m}
}
$$

$$
\overline{z}_{i,d}
=
\frac{1}{3}
\sum_{m=1}^{3}
z_{i,d,m}
$$

where \\(s_m \in \{+1, -1\}\\) flips lower-is-better metrics so that higher normalized scores always indicate better estimated translation quality.

**Scoring direction matters.** QE filtering works best when scoring is aligned with the final training direction (X → Y). Reversing the direction (Y → X) leads to average downstream degradation of 12.0% on MetricX and 3.1% on SSA-COMET. This means that even for back-translated data, QE should be computed in the same direction as the training example.

### 3. Data Quantity Selection Strategies

Once each sentence pair receives a unified QE score, we study how to convert quality estimates into an effective training mixture. We compare three complementary strategies:

1. **Thresholding:** Keep only sentence pairs above a minimum (\overline{z})-score.
2. **TopN Capping:** Limit the number of examples per language pair to prevent higher-resource languages from dominating the mixture.
3. **Bidirectional Expansion:** Expand selected pairs into both translation directions while keeping QE scoring aligned with each final training direction.

These strategies allow us to study the effects of quality, language balance, and translation direction. We sweep them across open-source, synthetic, and combined data pools; the resulting quality-quantity trade-offs determine our final TranslatePsy-AfriSLM Synthetic Mix, discussed in the Results section.

### 4. Final Training Mix for SFT

Our scaling experiments select filtered synthetic data with thresholding and bidirectional expansion as the core TranslatePsy-AfriSLM Synthetic Mix. The final configuration uses a unified QE threshold of (\overline{z} \ge 0.68), contains 32.37B training tokens, and reaches 0.632 SSA-COMET on BOUQuET. We then combine this our MT dataset with two auxiliary mixtures that serve complementary roles:

- **Instruct Mix (4.6M samples):** approximately half consists of African-language instruction data (~2.3M examples, which are concatenated from multiple open-source African instruct datasets), complemented by ~2.3M general instruction-following examples (concatenated from two open-source datasets, see paper for full details). This is to ensure the models retain their instruction-following capabilities.
- **Asia-Europe Mix (24M samples across 38 languages):** High-resource parallel pairs included specifically to prevent catastrophic forgetting on non-African languages using a subset of OPUS-100 data. This is to ensure that major European and Asian languages are well supported. You can reproduce these datasets using our Github code.

Using this final mixture, we fine-tune **Qwen3.5-0.8B, 2B, and 4B** for one epoch with SFT, computing loss only on assistant target tokens. Training examples follow the Qwen3.5 chat format, with sequences capped at 2,048 tokens and packed for efficient training.

---

## 📊 Evaluation: Datasets & Metrics

We evaluate TranslatePsy-AfriSLM on **19 in-distribution African languages (Africa-IID)** and **8 held-out African languages (Africa-OOD)** across three complementary MT benchmarks.

**🌐 Evaluation languages (Africa-IID & Africa-OOD)**

| **Group** | **Languages** |
| --- | --- |
| **Africa-IID (19)** | Afrikaans (afr), Amharic (amh), Hausa (hau), Igbo (ibo), Kinyarwanda (kin), Lingala (lin), Luganda (lug), Malagasy (mlg), Nyanja (nya), Oromo (orm), Shona (sna), Somali (som), Southern Sotho (sot), Swahili (swa), Tswana (tsn), Wolof (wol), Xhosa (xho), Yoruba (yor), Zulu (zul) |
| **Africa-OOD (8)** | Sudanese Arabic (apd), Akan (aka), Tamazight (ber), Kituba (ktu), Bambara (bam), Sepedi (nso), Mooré (mos), Nigerian Pidgin (pcm) |

**📏 Benchmarks & metrics**

| **Component** | **Evaluation Setup** |
| --- | --- |
| **Benchmarks** | **Flores-200** (1,012 sentences), **BOUQuET** (854), **Smol** (863) |
| **Metrics** | **SSA-COMET ↑**, **COMET-22 ↑**, **MetricX-24 ↓**, **ChrF++ ↑, spBLEU ↑** |

The three benchmarks cover all 19 Africa-IID languages, while Africa-OOD coverage varies by benchmark. We use multiple neural and lexical metrics to evaluate translation quality from complementary perspectives (see table above).


---

## 🏆 Results & Insights

### 1. Data Scaling & Token Efficiency

Quality filtering substantially improves token efficiency. For open-source data, filtering reduces the training budget from 44.93B to 1.76B tokens, a 96% reduction, while maintaining comparable SSA-COMET performance (see Figure 3 below).

Synthetic data scales even more effectively: filtered synthetic mixtures outperform open-source mixtures across nearly all training-token budgets. At larger scales, simply adding lower-quality open-source data provides little benefit and can even dilute the training signal.

Based on these results, we select a filtered synthetic mixture with bidirectional expansion as our final **TranslatePsy-AfriSLM training data,** using 32.37B tokens and reaching 0.632 SSA-COMET.


![pareto_frontier_ssa_vs_tokens__bouquet](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/xewpUp-FkeZEbTszH1uJ5.png)
<p align="center">
  Figure 3: Quality–efficiency Pareto frontier — SSA-COMET vs. training tokens (log scale) on BOUQuET. Marker shape encodes the selection strategy and fill shade the QE threshold. Filtered synthetic (green) and synthetic + open-source (blue) mixtures dominate raw open-source data (red); the final TranslatePsy-AfriSLM sits at the top-right, far above the unfiltered open-source baseline.
</p>

### 2. Top Benchmark Performance

Fine-tuned on our final data mixture, TranslatePsy-AfriSLM demonstrates that model scale alone is not sufficient for strong African machine translation. Of the 19 Africa-IID languages, even our **smallest 0.8B model outperforms Qwen3.5-122B-A10B and TranslateGemma-27B** across Flores-200, BOUQuET, and Smol. It also matches NLLB-3.3B on Flores-200 and surpasses it on BOUQuET and Smol, despite using roughly one quarter of the parameters.

Scaling TranslatePsy-AfriSLM from 0.8B to 2B and 4B yields further consistent improvements, with the 4B model achieving the strongest SSA-COMET results across all three benchmarks among the evaluated systems. These results suggest that, for low-resource African MT, **carefully curated post-training data can matter more than simply increasing model size**.


![plot_family](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/uOSy0o_NJUtIjA1sjeKIq.png)
<p align="center">
  Figure 4: The full TranslatePsy-AfriSLM family (0.8B, 2B, 4B) on SSA-COMET across Flores-200, BOUQuET, and Smol. Every size leads at its scale, and the 4B model achieves the strongest SSA-COMET overall, outperforming much larger general-purpose and specialised systems.
</p>

### 3. Generalization to Unseen African Languages

TranslatePsy-AfriSLM generalizes beyond the 19 languages seen during fine-tuning. On 8 held-out African languages, **TranslatePsy-AfriSLM-2B improves over the Qwen3.5-2B backbone on all languages**, with particularly strong gains for Sepedi, Bambara, and Akan.

These results suggest meaningful cross-lingual transfer beyond the training distribution (SSA-COMET scores), although gains are less uniform across other evaluation metrics.

![plot_africa_ood](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/etQAkJnOJVNYU0EwXVfSD.png)
<p align="center">
  Figure 5: Transfer to 8 held-out African languages never seen in fine-tuning. TranslatePsy-AfriSLM-2B improves SSA-COMET over its Qwen3.5-2B backbone on every language (both directions, averaged over Flores-200, BOUQuET, and Smol) — an average gain of +0.127 (+46%). Δ shows the per-language improvement.
</p>


### 4. Zero-Shot African-to-African Language Translation

Our training data contains only English ↔ African language pairs, meaning the models **never observe direct** African-to-African pairs during training. Despite this, TranslatePsy-AfriSLM-2B achieves the **best aggregate performance across all four evaluation metrics** on 20 zero-shot translation directions such as Swahili, Hausa, Yoruba, Zulu, and Amharic. This suggests that the improvements learned from English-centric post-training transfer to entirely unseen African-to-African translation directions.

![plot_afr2afr](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/sFGMHiPNktB8EJI1TNvzv.png)
<p align="center">
  Figure 6: Zero-shot African-to-African translation on 20 xx→yy directions (BOUQuET) among Swahili, Hausa, Yoruba, Zulu, and Amharic — none seen in training. Trained only on English↔African pairs, TranslatePsy-AfriSLM-2B still tops all four metrics against systems up to 61× its size. The MetricX-24 axis is reflected so taller is always better.
</p>

### 5. Mitigating Catastrophic Forgetting

Fine-tuning without the **Asia-Europe Mix** substantially degrades translation performance on non-African languages, with MetricX showing an **86.0% drop** across the evaluated Asian and European languages (Figure 5). Adding the mix reduces this degradation to **10.3%**, while preserving nearly identical gains on Africa-IID languages. We therefore include the Asia-Europe Mix to **preserve broader multilingual capabilities**.


![image](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/x0wEXOcNVqMwvDWCBcTGH.png)
<p align="center">
  Figure 7: Asia–Europe Mix ablation. Percent change vs. the Qwen3.5-2B baseline on BOUQuET for Asian, European, and Africa-IID languages (MetricX sign-flipped so higher is better everywhere). A light Asia–Europe mix (black) recovers most of the non-African forgetting seen without it (grey), averaging −39.0% → −4.1% across metrics, while African gains stay intact. Δ = mix − no-mix, in points.
</p>

### 6. Evaluation Robustness

Because our unified QE filter overlaps with several of the neural metrics that also serve as evaluation metrics, our reported gains could in principle reflect metric-specific bias rather than genuine translation quality. We address this concern from two independent angles: correlation with metrics excluded from filtering, and an independent LLM-as-a-judge evaluation.

**Metric circularity.** To test whether our rankings are an artifact of the filtering family, we compare our neural metrics (COMET-22, SSA-COMET, MetricX-24) against two lexical metrics that are never used for QE filtering: chrF++ and spBLEU. Across all system rankings, the neural and surface-level metrics move in tight alignment — every pairwise Spearman correlation is at least 0.95 in magnitude, and all are highly significant (p ≪ 0.001). Because the rankings hold under metrics outside the filtering family, our performance gains are unlikely to be an artifact of metric-specific optimization.

| Spearman ρ | COMET-22 | chrF++ | SSA-COMET | MetricX ↓ | spBLEU |
| --- | :---: | :---: | :---: | :---: | :---: |
| **COMET-22** | 1 | 0.984 | 0.967 | −0.978 | 0.971 |
| **chrF++** | — | 1 | 0.958 | −0.968 | 0.989 |
| **SSA-COMET** | — | — | 1 | −0.985 | 0.952 |
| **MetricX ↓** | — | — | — | 1 | −0.970 |
| **spBLEU** | — | — | — | — | 1 |

*Table 1: Spearman rank correlation (ρ) between neural metrics (COMET-22, SSA-COMET, MetricX) and lexical metrics excluded from QE filtering (chrF++, spBLEU) across system rankings. All correlations are strong (|ρ| ≥ 0.95) and highly significant (p ≪ 0.001).*

**Independent LLM-as-a-judge.** As a second check, we conducted an LLM-as-a-judge evaluation comparing TranslatePsy-AfriSLM-2B against NLLB-3.3B and TranslateGemma-27B. We used two frontier models, GPT-5.5 and Claude Opus 4.8, as independent judges to provide a semantic assessment of translation quality. For each language pair, we randomly sampled 100 sentences from the BOUQuET test set and asked each judge to perform pairwise comparisons. Aggregating how often each system was ranked first, second, or third, the resulting ordering agrees with our automatic-metric rankings (see Figure 8), giving further evidence that the observed gains persist beyond the metrics used for data filtering.

![plot_llm_judge](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/HVpWPKM15nxPzzi3tC5N2.png)
*Figure 8: LLM-as-a-judge rankings on BOUQuET (GPT-5.5 and Claude Opus 4.8 as judges). TranslatePsy-AfriSLM-2B is ranked #1 most often in both eng→xx and xx→eng, matching the automatic-metric ordering.*


### 7. Conversational MT

Thanks to our Instruct mixes, TranslatePsy-AfriSLM models are not rigid input-output translators. They retain multi-turn translation, language identification, and cross-lingual conversational capabilities.

![TranslatePsy-AfriSLM_chat-demo](https://cdn-uploads.huggingface.co/production/uploads/61a623353aff6ad7c083e882/5aKjfnTXtGaYYL8aV5YK7.png)
<p align="center">
  Figure 9: Multi-turn chat with TranslatePsy-AfriSLM-4B — sentence translation, language identification, and cross-lingual conversation across Lingala, Swahili, German, Amharic, Igbo, Chinese, Hausa, Somali, and Zulu in a single session.
</p>

## 🔮 Limitations & Future Work

While TranslatePsy-AfriSLM demonstrates the power of data scaling, several key challenges remain for future research:

1. **Automated QE vs. Human Judgment:** Reference-free QE models such as AfriCOMET, SSA-COMET and Metric-X share underlying model families with evaluation metrics. While synthetic data scores as high as human-translated pairs under automated metrics, absolute quality gaps compared to European languages still exist.
2. **Compound Error Propagation:** Large-scale automated curation pipelines carry the risk of compounding errors across data scraping, teacher model generation, QE filtering, and automated scoring. This warrants further investigation in future works.
3. **Human Evaluation Campaigns:** Future work must incorporate systematic human evaluation campaigns involving expert native linguists to validate fine-grained errors, dialectal nuances, and cultural context preservation.

## 📚 References

- **[1] Adelani et al. (2022b):** *Findings of the WMT'22 shared task on large-scale machine translation evaluation for African languages.* WMT.
- **[2] Caswell et al. (2025):** *SMOL: Professionally translated parallel data for 115 under-represented languages.*
- **[3] Juraska et al. (2024):** *MetricX-24: The Google submission to the WMT 2024 metrics shared task.* WMT.
- **[4] Li et al. (2025):** *SSA-COMET: Do LLMs outperform learned metrics in evaluating MT for under-resourced African languages?* EMNLP.
- **[5] Moslem et al. (2026):** *AfriNLLB: Efficient translation models for African languages.*
- **[6] NLLB Team et al. (2022):** *No Language Left Behind: Scaling human-centered machine translation.* arXiv:2207.04672.
- **[7] Qwen Team (2026):** *Qwen3.5: Accelerating productivity with native multimodal agents.*
- **[8] Wang et al. (2024):** *AfriMTE and AfriCOMET: Enhancing COMET to embrace under-resourced African languages.* NAACL.
- **[9] Klein et al. (2020):** *CTranslate2: Fast inference engine for Transformer models.* GitHub, OpenNMT.
- **[10] Tiedemann (2012):** *Parallel data, tools and interfaces in OPUS.* LREC.
- **[11] Ji et al. (2024):** *EMMA-500: Enhancing massively multilingual adaptation of large language models (MALA corpus).* arXiv:2409.17892.
- **[12] Penedo et al. (2026):** *FineTranslations.* Hugging Face Datasets.
- **[13] Kudugunta et al. (2023):** *MADLAD-400: A multilingual and document-level large audited dataset.*


## Licensing Information
This model, which was fine-tuned as described in the [blog post](https://huggingface.co/blog/qvac/translate-psy-afrislm) is licensed by Tether Data, S.A. de C.V. under the [Apache 2.0 license](https://huggingface.co/datasets/choosealicense/licenses/blob/main/markdown/apache-2.0.md). As described in the blog post, this model is a version of the [Qwen3.5-0.8B model](https://huggingface.co/Qwen/Qwen3.5-0.8B), which is made available under the [Apache 2.0 license](https://huggingface.co/datasets/choosealicense/licenses/blob/main/markdown/apache-2.0.md). As described in the blog post, the [NLLB-200-3.3B model](https://huggingface.co/facebook/nllb-200-3.3B) which is made available under the [CC-BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/legalcode.en) license was used as a teacher model.
The [TranslatePsy-AfriSLM synthetic mix](https://huggingface.co/datasets/qvac/TranslatePsy-AfriSLM-Synthetic-Mix) is made available under the [CC-BY-NC 4.0](https://creativecommons.org/licenses/by-nc/4.0/legalcode.en) license. The [SmolTalk2 dataset](https://huggingface.co/datasets/HuggingFaceTB/smoltalk2) is made available under the [Apache 2.0](https://huggingface.co/datasets/choosealicense/licenses/blob/main/markdown/apache-2.0.md) license. The [Dolci-Instruct dataset](https://huggingface.co/datasets/allenai/Dolci-Instruct-SFT-No-Tools) is made available under the [Open Data Commons Attribution License](https://opendatacommons.org/licenses/by/1-0/) (ODC-By) v1.0 license. The [OPUS-100 dataset](https://huggingface.co/datasets/Helsinki-NLP/opus-100) is available on Hugging Face. As described in the blog post, the TranslatePsy-AfriSLM, SmolTalk2, Dolci-Instruct and OPUS-100 datasets were used as a part of fine-tuning the model.
Licenses are reproduced below.

Apache
Version 2.0, January 2004
http://www.apache.org/licenses/
 
TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION
Definitions.
"License" shall mean the terms and conditions for use, reproduction, and distribution as defined by Sections 1 through 9 of this document.
"Licensor" shall mean the copyright owner or entity authorized by the copyright owner that is granting the License.
"Legal Entity" shall mean the union of the acting entity and all other entities that control, are controlled by, or are under common control with that entity. For the purposes of this definition, "control" means (i) the power, direct or indirect, to cause the direction or management of such entity, whether by contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the outstanding shares, or (iii) beneficial ownership of such entity.
"You" (or "Your") shall mean an individual or Legal Entity exercising permissions granted by this License.
"Source" form shall mean the preferred form for making modifications, including but not limited to software source code, documentation source, and configuration files.
"Object" form shall mean any form resulting from mechanical transformation or translation of a Source form, including but not limited to compiled object code, generated documentation, and conversions to other media types.
"Work" shall mean the work of authorship, whether in Source or Object form, made available under the License, as indicated by a copyright notice that is included in or attached to the work (an example is provided in the Appendix below).
"Derivative Works" shall mean any work, whether in Source or Object form, that is based on (or derived from) the Work and for which the editorial revisions, annotations, elaborations, or other modifications represent, as a whole, an original work of authorship. For the purposes of this License, Derivative Works shall not include works that remain separable from, or merely link (or bind by name) to the interfaces of, the Work and Derivative Works thereof.
"Contribution" shall mean any work of authorship, including the original version of the Work and any modifications or additions to that Work or Derivative Works thereof, that is intentionally submitted to Licensor for inclusion in the Work by the copyright owner or by an individual or Legal Entity authorized to submit on behalf of the copyright owner. For the purposes of this definition, "submitted" means any form of electronic, verbal, or written communication sent to the Licensor or its representatives, including but not limited to communication on electronic mailing lists, source code control systems, and issue tracking systems that are managed by, or on behalf of, the Licensor for the purpose of discussing and improving the Work, but excluding communication that is conspicuously marked or otherwise designated in writing by the copyright owner as "Not a Contribution."
"Contributor" shall mean Licensor and any individual or Legal Entity on behalf of whom a Contribution has been received by Licensor and subsequently incorporated within the Work.
Grant of Copyright License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable copyright license to reproduce, prepare Derivative Works of, publicly display, publicly perform, sublicense, and distribute the Work and such Derivative Works in Source or Object form.
Grant of Patent License. Subject to the terms and conditions of this License, each Contributor hereby grants to You a perpetual, worldwide, non-exclusive, no-charge, royalty-free, irrevocable (except as stated in this section) patent license to make, have made, use, offer to sell, sell, import, and otherwise transfer the Work, where such license applies only to those patent claims licensable by such Contributor that are necessarily infringed by their Contribution(s) alone or by combination of their Contribution(s) with the Work to which such Contribution(s) was submitted. If You institute patent litigation against any entity (including a cross-claim or counterclaim in a lawsuit) alleging that the Work or a Contribution incorporated within the Work constitutes direct or contributory patent infringement, then any patent licenses granted to You under this License for that Work shall terminate as of the date such litigation is filed.
Redistribution. You may reproduce and distribute copies of the Work or Derivative Works thereof in any medium, with or without modifications, and in Source or Object form, provided that You meet the following conditions:
(a)   You must give any other recipients of the Work or Derivative Works a copy of this License; and
(b)   You must cause any modified files to carry prominent notices stating that You changed the files; and
(c)   You must retain, in the Source form of any Derivative Works that You distribute, all copyright, patent, trademark, and attribution notices from the Source form of the Work, excluding those notices that do not pertain to any part of the Derivative Works; and
(d)   If the Work includes a "NOTICE" text file as part of its distribution, then any Derivative Works that You distribute must include a readable copy of the attribution notices contained within such NOTICE file, excluding those notices that do not pertain to any part of the Derivative Works, in at least one of the following places: within a NOTICE text file distributed as part of the Derivative Works; within the Source form or documentation, if provided along with the Derivative Works; or, within a display generated by the Derivative Works, if and wherever such third-party notices normally appear. The contents of the NOTICE file are for informational purposes only and do not modify the License. You may add Your own attribution notices within Derivative Works that You distribute, alongside or as an addendum to the NOTICE text from the Work, provided that such additional attribution notices cannot be construed as modifying the License.
You may add Your own copyright statement to Your modifications and may provide additional or different license terms and conditions for use, reproduction, or distribution of Your modifications, or for any such Derivative Works as a whole, provided Your use, reproduction, and distribution of the Work otherwise complies with the conditions stated in this License.
Submission of Contributions. Unless You explicitly state otherwise, any Contribution intentionally submitted for inclusion in the Work by You to the Licensor shall be under the terms and conditions of this License, without any additional terms or conditions. Notwithstanding the above, nothing herein shall supersede or modify the terms of any separate license agreement you may have executed with Licensor regarding such Contributions.
Trademarks. This License does not grant permission to use the trade names, trademarks, service marks, or product names of the Licensor, except as required for reasonable and customary use in describing the origin of the Work and reproducing the content of the NOTICE file.
Disclaimer of Warranty. Unless required by applicable law or agreed to in writing, Licensor provides the Work (and each Contributor provides its Contributions) on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied, including, without limitation, any warranties or conditions of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or FITNESS FOR A PARTICULAR PURPOSE. You are solely responsible for determining the appropriateness of using or redistributing the Work and assume any risks associated with Your exercise of permissions under this License.
Limitation of Liability. In no event and under no legal theory, whether in tort (including negligence), contract, or otherwise, unless required by applicable law (such as deliberate and grossly negligent acts) or agreed to in writing, shall any Contributor be liable to You for damages, including any direct, indirect, special, incidental, or consequential damages of any character arising as a result of this License or out of the use or inability to use the Work (including but not limited to damages for loss of goodwill, work stoppage, computer failure or malfunction, or any and all other commercial damages or losses), even if such Contributor has been advised of the possibility of such damages.
Accepting Warranty or Additional Liability. While redistributing the Work or Derivative Works thereof, You may choose to offer, and charge a fee for, acceptance of support, warranty, indemnity, or other liability obligations and/or rights consistent with this License. However, in accepting such obligations, You may act only on Your own behalf and on Your sole responsibility, not on behalf of any other Contributor, and only if You agree to indemnify, defend, and hold each Contributor harmless for any liability incurred by, or claims asserted against, such Contributor by reason of your accepting any such warranty or additional liability.
END OF TERMS AND CONDITIONS
 
CC BY NC 4.0
Attribution-NonCommercial 4.0 International
https://creativecommons.org/licenses/by-nc/4.0/legalcode.en
By exercising the Licensed Rights (defined below), You accept and agree to be bound by the terms and conditions of this Creative Commons Attribution-NonCommercial 4.0 International Public License ("Public License"). To the extent this Public License may be interpreted as a contract, You are granted the Licensed Rights in consideration of Your acceptance of these terms and conditions, and the Licensor grants You such rights in consideration of benefits the Licensor receives from making the Licensed Material available under these terms and conditions.
Section 1 – Definitions.
Adapted Material means material subject to Copyright and Similar Rights that is derived from or based upon the Licensed Material and in which the Licensed Material is translated, altered, arranged, transformed, or otherwise modified in a manner requiring permission under the Copyright and Similar Rights held by the Licensor. For purposes of this Public License, where the Licensed Material is a musical work, performance, or sound recording, Adapted Material is always produced where the Licensed Material is synched in timed relation with a moving image.
Adapter's License means the license You apply to Your Copyright and Similar Rights in Your contributions to Adapted Material in accordance with the terms and conditions of this Public License.
Copyright and Similar Rights means copyright and/or similar rights closely related to copyright including, without limitation, performance, broadcast, sound recording, and Sui Generis Database Rights, without regard to how the rights are labeled or categorized. For purposes of this Public License, the rights specified in Section 2(b)(1)-(2) are not Copyright and Similar Rights.
Effective Technological Measures means those measures that, in the absence of proper authority, may not be circumvented under laws fulfilling obligations under Article 11 of the WIPO Copyright Treaty adopted on December 20, 1996, and/or similar international agreements.
Exceptions and Limitations means fair use, fair dealing, and/or any other exception or limitation to Copyright and Similar Rights that applies to Your use of the Licensed Material.
Licensed Material means the artistic or literary work, database, or other material to which the Licensor applied this Public License.
Licensed Rights means the rights granted to You subject to the terms and conditions of this Public License, which are limited to all Copyright and Similar Rights that apply to Your use of the Licensed Material and that the Licensor has authority to license.
Licensor means the individual(s) or entity(ies) granting rights under this Public License.
NonCommercial means not primarily intended for or directed towards commercial advantage or monetary compensation. For purposes of this Public License, the exchange of the Licensed Material for other material subject to Copyright and Similar Rights by digital file-sharing or similar means is NonCommercial provided there is no payment of monetary compensation in connection with the exchange.
Share means to provide material to the public by any means or process that requires permission under the Licensed Rights, such as reproduction, public display, public performance, distribution, dissemination, communication, or importation, and to make material available to the public including in ways that members of the public may access the material from a place and at a time individually chosen by them.
Sui Generis Database Rights means rights other than copyright resulting from Directive 96/9/EC of the European Parliament and of the Council of 11 March 1996 on the legal protection of databases, as amended and/or succeeded, as well as other essentially equivalent rights anywhere in the world.
You means the individual or entity exercising the Licensed Rights under this Public License. Your has a corresponding meaning.
Section 2 – Scope.
License grant.
Subject to the terms and conditions of this Public License, the Licensor hereby grants You a worldwide, royalty-free, non-sublicensable, non-exclusive, irrevocable license to exercise the Licensed Rights in the Licensed Material to:
reproduce and Share the Licensed Material, in whole or in part, for NonCommercial purposes only; and
produce, reproduce, and Share Adapted Material for NonCommercial purposes only.
Exceptions and Limitations. For the avoidance of doubt, where Exceptions and Limitations apply to Your use, this Public License does not apply, and You do not need to comply with its terms and conditions.
Term. The term of this Public License is specified in Section 6(a).
Media and formats; technical modifications allowed. The Licensor authorizes You to exercise the Licensed Rights in all media and formats whether now known or hereafter created, and to make technical modifications necessary to do so. The Licensor waives and/or agrees not to assert any right or authority to forbid You from making technical modifications necessary to exercise the Licensed Rights, including technical modifications necessary to circumvent Effective Technological Measures. For purposes of this Public License, simply making modifications authorized by this Section 2(a)(4) never produces Adapted Material.
Downstream recipients.
Offer from the Licensor – Licensed Material. Every recipient of the Licensed Material automatically receives an offer from the Licensor to exercise the Licensed Rights under the terms and conditions of this Public License.
No downstream restrictions. You may not offer or impose any additional or different terms or conditions on, or apply any Effective Technological Measures to, the Licensed Material if doing so restricts exercise of the Licensed Rights by any recipient of the Licensed Material.
No endorsement. Nothing in this Public License constitutes or may be construed as permission to assert or imply that You are, or that Your use of the Licensed Material is, connected with, or sponsored, endorsed, or granted official status by, the Licensor or others designated to receive attribution as provided in Section 3(a)(1)(A)(i).
Other rights.
Moral rights, such as the right of integrity, are not licensed under this Public License, nor are publicity, privacy, and/or other similar personality rights; however, to the extent possible, the Licensor waives and/or agrees not to assert any such rights held by the Licensor to the limited extent necessary to allow You to exercise the Licensed Rights, but not otherwise.
Patent and trademark rights are not licensed under this Public License.
To the extent possible, the Licensor waives any right to collect royalties from You for the exercise of the Licensed Rights, whether directly or through a collecting society under any voluntary or waivable statutory or compulsory licensing scheme. In all other cases the Licensor expressly reserves any right to collect such royalties, including when the Licensed Material is used other than for NonCommercial purposes.
Section 3 – License Conditions.
Your exercise of the Licensed Rights is expressly made subject to the following conditions.
Attribution.
If You Share the Licensed Material (including in modified form), You must:
retain the following if it is supplied by the Licensor with the Licensed Material:
identification of the creator(s) of the Licensed Material and any others designated to receive attribution, in any reasonable manner requested by the Licensor (including by pseudonym if designated);
a copyright notice;
a notice that refers to this Public License;
a notice that refers to the disclaimer of warranties;
a URI or hyperlink to the Licensed Material to the extent reasonably practicable;
indicate if You modified the Licensed Material and retain an indication of any previous modifications; and
indicate the Licensed Material is licensed under this Public License, and include the text of, or the URI or hyperlink to, this Public License.
You may satisfy the conditions in Section 3(a)(1) in any reasonable manner based on the medium, means, and context in which You Share the Licensed Material. For example, it may be reasonable to satisfy the conditions by providing a URI or hyperlink to a resource that includes the required information.
If requested by the Licensor, You must remove any of the information required by Section 3(a)(1)(A) to the extent reasonably practicable.
If You Share Adapted Material You produce, the Adapter's License You apply must not prevent recipients of the Adapted Material from complying with this Public License.
Section 4 – Sui Generis Database Rights.
Where the Licensed Rights include Sui Generis Database Rights that apply to Your use of the Licensed Material:
for the avoidance of doubt, Section 2(a)(1) grants You the right to extract, reuse, reproduce, and Share all or a substantial portion of the contents of the database for NonCommercial purposes only;
if You include all or a substantial portion of the database contents in a database in which You have Sui Generis Database Rights, then the database in which You have Sui Generis Database Rights (but not its individual contents) is Adapted Material; and
You must comply with the conditions in Section 3(a) if You Share all or a substantial portion of the contents of the database.
For the avoidance of doubt, this Section 4 supplements and does not replace Your obligations under this Public License where the Licensed Rights include other Copyright and Similar Rights.
Section 5 – Disclaimer of Warranties and Limitation of Liability.
Unless otherwise separately undertaken by the Licensor, to the extent possible, the Licensor offers the Licensed Material as-is and as-available, and makes no representations or warranties of any kind concerning the Licensed Material, whether express, implied, statutory, or other. This includes, without limitation, warranties of title, merchantability, fitness for a particular purpose, non-infringement, absence of latent or other defects, accuracy, or the presence or absence of errors, whether or not known or discoverable. Where disclaimers of warranties are not allowed in full or in part, this disclaimer may not apply to You.
To the extent possible, in no event will the Licensor be liable to You on any legal theory (including, without limitation, negligence) or otherwise for any direct, special, indirect, incidental, consequential, punitive, exemplary, or other losses, costs, expenses, or damages arising out of this Public License or use of the Licensed Material, even if the Licensor has been advised of the possibility of such losses, costs, expenses, or damages. Where a limitation of liability is not allowed in full or in part, this limitation may not apply to You.
The disclaimer of warranties and limitation of liability provided above shall be interpreted in a manner that, to the extent possible, most closely approximates an absolute disclaimer and waiver of all liability.
Section 6 – Term and Termination.
This Public License applies for the term of the Copyright and Similar Rights licensed here. However, if You fail to comply with this Public License, then Your rights under this Public License terminate automatically.
Where Your right to use the Licensed Material has terminated under Section 6(a), it reinstates:
automatically as of the date the violation is cured, provided it is cured within 30 days of Your discovery of the violation; or
upon express reinstatement by the Licensor.
For the avoidance of doubt, this Section 6(b) does not affect any right the Licensor may have to seek remedies for Your violations of this Public License.
For the avoidance of doubt, the Licensor may also offer the Licensed Material under separate terms or conditions or stop distributing the Licensed Material at any time; however, doing so will not terminate this Public License.
Sections 1, 5, 6, 7, and 8 survive termination of this Public License.
Section 7 – Other Terms and Conditions.
The Licensor shall not be bound by any additional or different terms or conditions communicated by You unless expressly agreed.
Any arrangements, understandings, or agreements regarding the Licensed Material not stated herein are separate from and independent of the terms and conditions of this Public License.
Section 8 – Interpretation.
For the avoidance of doubt, this Public License does not, and shall not be interpreted to, reduce, limit, restrict, or impose conditions on any use of the Licensed Material that could lawfully be made without permission under this Public License.
To the extent possible, if any provision of this Public License is deemed unenforceable, it shall be automatically reformed to the minimum extent necessary to make it enforceable. If the provision cannot be reformed, it shall be severed from this Public License without affecting the enforceability of the remaining terms and conditions.
No term or condition of this Public License will be waived and no failure to comply consented to unless expressly agreed to by the Licensor.
Nothing in this Public License constitutes or may be interpreted as a limitation upon, or waiver of, any privileges and immunities that apply to the Licensor or You, including from the legal processes of any jurisdiction or authority.
 
ODC-BY
Open Data Commons Attribution License (ODC-By) v1.0
https://opendatacommons.org/licenses/by/1-0/
 
Preamble
The Open Data Commons Attribution License is a license agreement intended to allow users to freely share, modify, and use this Database subject only to the attribution requirements set out in Section 4.
Databases can contain a wide variety of types of content (images, audiovisual material, and sounds all in the same database, for example), and so this license only governs the rights over the Database, and not the contents of the Database individually. Licensors may therefore wish to use this license together with another license for the contents.
Sometimes the contents of a database, or the database itself, can be covered by other rights not addressed here (such as private contracts, trademark over the name, or privacy rights / data protection rights over information in the contents), and so you are advised that you may have to consult other documents or clear other rights before doing activities not covered by this License.
The Licensor (as defined below)
and
You (as defined below)
agree as follows:
1.0 Definitions of Capitalised Words
“Collective Database” – Means this Database in unmodified form as part of a collection of independent databases in themselves that together are assembled into a collective whole. A work that constitutes a Collective Database will not be considered a Derivative Database.
“Convey” – As a verb, means Using the Database, a Derivative Database, or the Database as part of a Collective Database in any way that enables a Person to make or receive copies of the Database or a Derivative Database. Conveying does not include interaction with a user through a computer network, or creating and Using a Produced Work, where no transfer of a copy of the Database or a Derivative Database occurs.
“Contents” – The contents of this Database, which includes the information, independent works, or other material collected into the Database. For example, the contents of the Database could be factual data or works such as images, audiovisual material, text, or sounds.
“Database” – A collection of material (the Contents) arranged in a systematic or methodical way and individually accessible by electronic or other means offered under the terms of this License.
“Database Directive” – Means Directive 96/9/EC of the European Parliament and of the Council of 11 March 1996 on the legal protection of databases, as amended or succeeded.
“Database Right” – Means rights resulting from the Chapter III (“sui generis”) rights in the Database Directive (as amended and as transposed by member states), which includes the Extraction and Re-utilisation of the whole or a Substantial part of the Contents, as well as any similar rights available in the relevant jurisdiction under Section 10.4.
“Derivative Database” – Means a database based upon the Database, and includes any translation, adaptation, arrangement, modification, or any other alteration of the Database or of a Substantial part of the Contents. This includes, but is not limited to, Extracting or Re-utilising the whole or a Substantial part of the Contents in a new Database.
“Extraction” – Means the permanent or temporary transfer of all or a Substantial part of the Contents to another medium by any means or in any form.
“License” – Means this license agreement and is both a license of rights such as copyright and Database Rights and an agreement in contract.
“Licensor” – Means the Person that offers the Database under the terms of this License.
“Person” – Means a natural or legal person or a body of persons corporate or incorporate.
“Produced Work” – a work (such as an image, audiovisual material, text, or sounds) resulting from using the whole or a Substantial part of the Contents (via a search or other query) from this Database, a Derivative Database, or this Database as part of a Collective Database.
“Publicly” – means to Persons other than You or under Your control by either more than 50% ownership or by the power to direct their activities (such as contracting with an independent consultant).
“Re-utilisation” – means any form of making available to the public all or a Substantial part of the Contents by the distribution of copies, by renting, by online or other forms of transmission.
“Substantial” – Means substantial in terms of quantity or quality or a combination of both. The repeated and systematic Extraction or Re-utilisation of insubstantial parts of the Contents may amount to the Extraction or Re-utilisation of a Substantial part of the Contents.
“Use” – As a verb, means doing any act that is restricted by copyright or Database Rights whether in the original medium or any other; and includes without limitation distributing, copying, publicly performing, publicly displaying, and preparing derivative works of the Database, as well as modifying the Database as may be technically necessary to use it in a different mode or format.
“You” – Means a Person exercising rights under this License who has not previously violated the terms of this License with respect to the Database, or who has received express permission from the Licensor to exercise rights under this License despite a previous violation.
Words in the singular include the plural and vice versa.
2.0 What this License covers
2.1. Legal effect of this document. This License is:
a. A license of applicable copyright and neighbouring rights;
b. A license of the Database Right; and
c. An agreement in contract between You and the Licensor.
2.2 Legal rights covered. This License covers the legal rights in the Database, including:
a. Copyright. Any copyright or neighbouring rights in the Database. The copyright licensed includes any individual elements of the Database, but does not cover the copyright over the Contents independent of this Database. See Section 2.4 for details. Copyright law varies between jurisdictions, but is likely to cover: the Database model or schema, which is the structure, arrangement, and organisation of the Database, and can also include the Database tables and table indexes; the data entry and output sheets; and the Field names of Contents stored in the Database;
b. Database Rights. Database Rights only extend to the Extraction and Re-utilisation of the whole or a Substantial part of the Contents. Database Rights can apply even when there is no copyright over the Database. Database Rights can also apply when the Contents are removed from the Database and are selected and arranged in a way that would not infringe any applicable copyright; and
c. Contract. This is an agreement between You and the Licensor for access to the Database. In return you agree to certain conditions of use on this access as outlined in this License.
2.3 Rights not covered.
a. This License does not apply to computer programs used in the making or operation of the Database;
b. This License does not cover any patents over the Contents or the Database; and
c. This License does not cover any trademarks associated with the Database.
2.4 Relationship to Contents in the Database. The individual items of the Contents contained in this Database may be covered by other rights, including copyright, patent, data protection, privacy, or personality rights, and this License does not cover any rights (other than Database Rights or in contract) in individual Contents contained in the Database.
For example, if used on a Database of images (the Contents), this License would not apply to copyright over individual images, which could have their own separate licenses, or one single license covering all of the rights over the images.
3.0 Rights granted
3.1 Subject to the terms and conditions of this License, the Licensor grants to You a worldwide, royalty-free, non-exclusive, terminable (but only under Section 9) license to Use the Database for the duration of any applicable copyright and Database Rights. These rights explicitly include commercial use, and do not exclude any field of endeavour. To the extent possible in the relevant jurisdiction, these rights may be exercised in all media and formats whether now known or created in the future.
The rights granted cover, for example:
a. Extraction and Re-utilisation of the whole or a Substantial part of the Contents;
b. Creation of Derivative Databases;
c. Creation of Collective Databases;
d. Creation of temporary or permanent reproductions by any means and in any form, in whole or in part, including of any Derivative Databases or as a part of Collective Databases; and
e. Distribution, communication, display, lending, making available, or performance to the public by any means and in any form, in whole or in part, including of any Derivative Database or as a part of Collective Databases.
3.2 Compulsory license schemes. For the avoidance of doubt:
a. Non-waivable compulsory license schemes. In those jurisdictions in which the right to collect royalties through any statutory or compulsory licensing scheme cannot be waived, the Licensor reserves the exclusive right to collect such royalties for any exercise by You of the rights granted under this License;
b. Waivable compulsory license schemes. In those jurisdictions in which the right to collect royalties through any statutory or compulsory licensing scheme can be waived, the Licensor waives the exclusive right to collect such royalties for any exercise by You of the rights granted under this License; and,
c. Voluntary license schemes. The Licensor waives the right to collect royalties, whether individually or, in the event that the Licensor is a member of a collecting society that administers voluntary licensing schemes, via that society, from any exercise by You of the rights granted under this License.
3.3 The right to release the Database under different terms, or to stop distributing or making available the Database, is reserved. Note that this Database may be multiple-licensed, and so You may have the choice of using alternative licenses for this Database. Subject to Section 10.4, all other rights not expressly granted by Licensor are reserved.
4.0 Conditions of Use
4.1 The rights granted in Section 3 above are expressly made subject to Your complying with the following conditions of use. These are important conditions of this License, and if You fail to follow them, You will be in material breach of its terms.
4.2 Notices. If You Publicly Convey this Database, any Derivative Database, or the Database as part of a Collective Database, then You must:
a. Do so only under the terms of this License;
b. Include a copy of this License or its Uniform Resource Identifier (URI) with the Database or Derivative Database, including both in the Database or Derivative Database and in any relevant documentation;
c. Keep intact any copyright or Database Right notices and notices that refer to this License; and
d. If it is not possible to put the required notices in a particular file due to its structure, then You must include the notices in a location (such as a relevant directory) where users would be likely to look for it.
4.3 Notice for using output (Contents). Creating and Using a Produced Work does not require the notice in Section 4.2. However, if you Publicly Use a Produced Work, You must include a notice associated with the Produced Work reasonably calculated to make any Person that uses, views, accesses, interacts with, or is otherwise exposed to the Produced Work aware that Content was obtained from the Database, Derivative Database, or the Database as part of a Collective Database, and that it is available under this License.
a. Example notice. The following text will satisfy notice under Section 4.3:
Contains information from DATABASE NAME which is made available
under the ODC Attribution License.
DATABASE NAME should be replaced with the name of the Database and a hyperlink to the location of the Database. “ODC Attribution License” should contain a hyperlink to the URI of the text of this License. If hyperlinks are not possible, You should include the plain text of the required URI’s with the above notice.
4.4 Licensing of others. You may not sublicense the Database. Each time You communicate the Database, the whole or Substantial part of the Contents, or any Derivative Database to anyone else in any way, the Licensor offers to the recipient a license to the Database on the same terms and conditions as this License. You are not responsible for enforcing compliance by third parties with this License, but You may enforce any rights that You have over a Derivative Database. You are solely responsible for any modifications of a Derivative Database made by You or another Person at Your direction. You may not impose any further restrictions on the exercise of the rights granted or affirmed under this License.
5.0 Moral rights
5.1 Moral rights. This section covers moral rights, including any rights to be identified as the author of the Database or to object to treatment that would otherwise prejudice the author’s honour and reputation, or any other derogatory treatment:
a. For jurisdictions allowing waiver of moral rights, Licensor waives all moral rights that Licensor may have in the Database to the fullest extent possible by the law of the relevant jurisdiction under Section 10.4;
b. If waiver of moral rights under Section 5.1 a in the relevant jurisdiction is not possible, Licensor agrees not to assert any moral rights over the Database and waives all claims in moral rights to the fullest extent possible by the law of the relevant jurisdiction under Section 10.4; and
c. For jurisdictions not allowing waiver or an agreement not to assert moral rights under Section 5.1 a and b, the author may retain their moral rights over certain aspects of the Database.
Please note that some jurisdictions do not allow for the waiver of moral rights, and so moral rights may still subsist over the Database in some jurisdictions.
6.0 Fair dealing, Database exceptions, and other rights not affected
6.1 This License does not affect any rights that You or anyone else may independently have under any applicable law to make any use of this Database, including without limitation:
a. Exceptions to the Database Right including: Extraction of Contents from non-electronic Databases for private purposes, Extraction for purposes of illustration for teaching or scientific research, and Extraction or Re-utilisation for public security or an administrative or judicial procedure.
b. Fair dealing, fair use, or any other legally recognised limitation or exception to infringement of copyright or other applicable laws.
6.2 This License does not affect any rights of lawful users to Extract and Re-utilise insubstantial parts of the Contents, evaluated quantitatively or qualitatively, for any purposes whatsoever, including creating a Derivative Database (subject to other rights over the Contents, see Section 2.4). The repeated and systematic Extraction or Re-utilisation of insubstantial parts of the Contents may however amount to the Extraction or Re-utilisation of a Substantial part of the Contents.
7.0 Warranties and Disclaimer
7.1 The Database is licensed by the Licensor “as is” and without any warranty of any kind, either express, implied, or arising by statute, custom, course of dealing, or trade usage. Licensor specifically disclaims any and all implied warranties or conditions of title, non-infringement, accuracy or completeness, the presence or absence of errors, fitness for a particular purpose, merchantability, or otherwise. Some jurisdictions do not allow the exclusion of implied warranties, so this exclusion may not apply to You.
8.0 Limitation of liability
8.1 Subject to any liability that may not be excluded or limited by law, the Licensor is not liable for, and expressly excludes, all liability for loss or damage however and whenever caused to anyone by any use under this License, whether by You or by anyone else, and whether caused by any fault on the part of the Licensor or not. This exclusion of liability includes, but is not limited to, any special, incidental, consequential, punitive, or exemplary damages such as loss of revenue, data, anticipated profits, and lost business. This exclusion applies even if the Licensor has been advised of the possibility of such damages.
8.2 If liability may not be excluded by law, it is limited to actual and direct financial loss to the extent it is caused by proved negligence on the part of the Licensor.
9.0 Termination of Your rights under this License
9.1 Any breach by You of the terms and conditions of this License automatically terminates this License with immediate effect and without notice to You. For the avoidance of doubt, Persons who have received the Database, the whole or a Substantial part of the Contents, Derivative Databases, or the Database as part of a Collective Database from You under this License will not have their licenses terminated provided their use is in full compliance with this License or a license granted under Section 4.8 of this License. Sections 1, 2, 7, 8, 9 and 10 will survive any termination of this License.
9.2 If You are not in breach of the terms of this License, the Licensor will not terminate Your rights under it.
9.3 Unless terminated under Section 9.1, this License is granted to You for the duration of applicable rights in the Database.
9.4 Reinstatement of rights. If you cease any breach of the terms and conditions of this License, then your full rights under this License will be reinstated:
a. Provisionally and subject to permanent termination until the 60th day after cessation of breach;
b. Permanently on the 60th day after cessation of breach unless otherwise reasonably notified by the Licensor; or
c. Permanently if reasonably notified by the Licensor of the violation, this is the first time You have received notice of violation of this License from the Licensor, and You cure the violation prior to 30 days after your receipt of the notice.
9.5 Notwithstanding the above, Licensor reserves the right to release the Database under different license terms or to stop distributing or making available the Database. Releasing the Database under different license terms or stopping the distribution of the Database will not withdraw this License (or any other license that has been, or is required to be, granted under the terms of this License), and this License will continue in full force and effect unless terminated as stated above.
10.0 General
10.1 If any provision of this License is held to be invalid or unenforceable, that must not affect the validity or enforceability of the remainder of the terms and conditions of this License and each remaining provision of this License shall be valid and enforced to the fullest extent permitted by law.
10.2 This License is the entire agreement between the parties with respect to the rights granted here over the Database. It replaces any earlier understandings, agreements or representations with respect to the Database.
10.3 If You are in breach of the terms of this License, You will not be entitled to rely on the terms of this License or to complain of any breach by the Licensor.
10.4 Choice of law. This License takes effect in and will be governed by the laws of the relevant jurisdiction in which the License terms are sought to be enforced. If the standard suite of rights granted under applicable copyright law and Database Rights in the relevant jurisdiction includes additional rights not granted under this License, these additional rights are granted in this License in order to meet the terms of this License.
