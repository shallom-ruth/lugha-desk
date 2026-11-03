# Lugha Desk privacy information — Store draft

This text describes the proposed complete Microsoft Store package. Review it against the final tested package and publish it at a stable HTTPS address before Store submission. The app remains experimental.

## Translation on your computer

Lugha Desk sends translation requests to an authenticated model server on the same computer using the local loopback address. The Store package includes its model and native engine; it does not use an online translation fallback, paid translation API or separate translation login. Translation text and results exist temporarily in memory. Lugha Desk does not automatically save translation history or automatically upload translation text, results or feedback. Windows may separately manage memory, crash data or system diagnostics.

## Installation and other services

Obtaining and updating the Store package and required Microsoft framework uses Microsoft’s services and their privacy practices. This is separate from the app’s offline translation. The separately distributed GitHub setup launcher can download a model from Hugging Face during first-time setup; that is not needed by the complete Store package. Download providers receive normal connection information under their own policies.

## Voluntary support and reports

To report an incorrect or inappropriate translation, contact shalomruth90@gmail.com. The Coverage and support dialog displays the contact details and sends nothing automatically. Choose what to include in a report; avoid sharing passwords, identity documents, personal or confidential text. Sending email or visiting the project website/GitHub uses the services you choose and their privacy practices.

If you contact the developer, the contact information and report you choose to send are used to investigate and respond. Reports are not automatically added to model training or posted publicly. The developer may keep a report while resolving it; contact the same address for a deletion request, subject to applicable retention obligations. Do not promise deletion of data held independently by email providers, GitHub or Microsoft.

## Licensing and responsibility

Original app/launcher source is MIT. The translation model, runtime and dependencies retain their separate license notices. Open source and local operation do not guarantee perfect translations or constitute a complete security audit. See the project’s component licensing and limitations.

Contact: Ruth Shallom, shalomruth90@gmail.com.
