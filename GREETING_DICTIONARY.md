Greeting dictionary

This candidate adds six directional entries for complete, standalone greetings:

English to Kinyarwanda: Good morning. -> Mwaramutse.
Kinyarwanda to English: Mwaramutse. -> Good morning.
English to Amharic: Good morning. -> እንደምን አደሩ?
Amharic to English: እንደምን አደሩ? -> Good morning.
English to Malagasy: How are you? -> Manao ahoana?
Malagasy to English: Manao ahoana? -> How are you?

The Amharic entry is a respectful morning greeting. These are reference-checked phrase equivalents, not universal word-for-word translations or independent language certification. Ruth Shallom tested the greetings in the local app and reported that they work. Fluent-speaker validation across varieties remains pending. Lingala dictionary entries are excluded pending further review.

Exact standalone matches use this built-in dictionary without loading the translation model. Matching allows capitalization differences, surrounding whitespace, repeated spaces or tabs, and final full stops, question marks or exclamation marks. Longer sentences, other greetings and other language routes use the existing offline model. There is no word replacement inside sentences. Text containing line breaks bypasses the dictionary.

This is an application-level correction. The model has not been retrained. General meaning errors, including borrowing and lending confusion, remain possible. The dictionary is deliberately small and does not establish accurate translation for all supported language options.

Sources

Kinyarwanda: Peace Corps Rwanda language lessons
https://files.peacecorps.gov/multimedia/audio/languagelessons/rwanda/RW_Kinyarwanda_Language_Lessons.pdf

Amharic: Peace Corps Ethiopia Amharic Language Manual, morning greeting dialogue (2015; hosted copy)
https://www.livelingua.com/peace-corps/Amharic/Amharic%20Peace%20Corps%20Language%20Manual-2015.pdf

Malagasy: Malagasy Dictionary and Encyclopedia, manao ahoana
https://motmalgache.org/bins/teny1/manaoahoana?w=manaoahoana

Setup

Open the launcher on Windows 10 or 11 x64. Setup remains free. The official Microsoft Visual C++ x64 runtime is required if not already installed:
https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist

A completed setup with the same verified model reuses it. First-time setup requires the existing one-time 1.08 GB model download; translation then works offline. This candidate does not bundle model weights or introduce a cloud translation service. The launcher remains unsigned. Keep Windows security protections enabled.

Verification

Local Rust app: 16 tests passed; one model-dependent test skipped.
R integration: 43 checks passed, covering dictionary matching, Unicode, bypasses and the desktop translation entry point.
Connected-model R check: greeting, longer sentence, greeting completed in one session; model loaded once and was used only for the longer sentence. These are behavior checks, not broad translation quality certification.
