Greeting dictionary

The dictionary covers complete, standalone greetings. The current development
candidate adds four phrase pairs (eight directions, with a Wolof spacing alias)
to the original six directional entries:

English to Kinyarwanda: Good morning. -> Mwaramutse.
Kinyarwanda to English: Mwaramutse. -> Good morning.
English to Amharic: Good morning. -> እንደምን አደሩ?
Amharic to English: እንደምን አደሩ? -> Good morning.
English to Malagasy: How are you? -> Manao ahoana?
Malagasy to English: Manao ahoana? -> How are you?

English to Wolof: Good morning. -> Jamm nga fanaan.
Wolof to English: Jamm nga fanaan. -> Good morning.
English to Wolof: How are you? -> Nanga def?
Wolof to English: Nanga def? / Na nga def? -> How are you?
English to Luganda: Good morning. -> Wasuze otya?
Luganda to English: Wasuze otya? -> Good morning.
English to Yoruba: Good morning. -> Ẹ káàárọ̀.
Yoruba to English: Ẹ káàárọ̀. -> Good morning.

The Wolof morning form comes from Peace Corps Mauritania lessons; other
varieties and situations use different greetings. The Luganda form addresses
one person and literally asks about their night; here it is translated as an
idiomatic morning greeting. Gendered titles are not inferred. The Yoruba form
is respectful. Tone marks are retained; accent-free spellings and Unicode
normalization variants are not inferred. The new entries have not yet been
tested by Ruth or independently reviewed by fluent speakers.

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

Wolof: Peace Corps Mauritania, Lesson 3, printed page 6 (PDF page 7)
https://fsi-languages.yojik.eu/languages/PeaceCorps/Wolof/MR_Wolof_Language_Lessons.pdf

Wolof spacing variant: WHO 2024 EIOS meeting booklet, printed page 39 (PDF page 45)
https://cdn.who.int/media/docs/default-source/eios/2024-eios-gtm-programme-booklet_english.pdf?sfvrsn=b30b4778_5

Luganda: Luganda/Lusoga/Lugwere Language Commission, Lesson 1, morning greetings
https://lugandalusogalugwerecommission.com/381335032/381335040/381335040_3014782

Yoruba: University of Texas at Austin, Yorùbá Yé Mi, Chapter 1, printed pages 26–27
https://coerll.utexas.edu/yemi/pdfs/yy_ch1.pdf

Setup

Open the launcher on Windows 10 or 11 x64. Setup remains free. The official Microsoft Visual C++ x64 runtime is required if not already installed:
https://learn.microsoft.com/en-us/cpp/windows/latest-supported-vc-redist

A completed setup with the same verified model reuses it. First-time setup requires the existing one-time 1.08 GB model download; translation then works offline. This candidate does not bundle model weights or introduce a cloud translation service. The launcher remains unsigned. Keep Windows security protections enabled.

Verification

Current development checks: `cargo test --locked` includes regressions for the
new phrases, Unicode capitalization and whole-message exclusions. After building,
`python tests/test_greetings.py path/to/lugha-desk.exe` exercises every entry in
`tests/greeting_dictionary.json` through the session worker, desktop worker entry
point and native worker, in a temporary folder containing no model or runtime.
The Windows build runs this check before packaging the app into the launcher.
These checks verify software behavior; they do not certify general meaning.

Historical checks for the original six entries:

Local Rust app: 16 tests passed; one model-dependent test skipped.
R integration: 43 checks passed, covering dictionary matching, Unicode, bypasses and the desktop translation entry point.
Connected-model R check: greeting, longer sentence, greeting completed in one session; model loaded once and was used only for the longer sentence. These are behavior checks, not broad translation quality certification.
