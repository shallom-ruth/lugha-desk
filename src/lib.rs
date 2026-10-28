pub fn validate(direction: &str, text: &str) -> Result<(), String> {
    language_names(direction)?;
    if text.trim().is_empty() { return Err("Enter some text first.".into()); }
    if text.chars().count()>600 { return Err("Use at most 600 characters in this prototype.".into()); }
    if text.contains('\0') { return Err("NUL characters are not supported.".into()); }
    Ok(())
}
pub fn wide(s: &str) -> Vec<u16> { s.encode_utf16().chain(Some(0)).collect() }
pub const LANGUAGES: &[(&str,&str)] = &[
    ("sw","Swahili"),("zu","Zulu"),("xh","Xhosa"),("st","Southern Sotho"),("tn","Tswana"),("af","Afrikaans"),
    ("so","Somali"),("lg","Luganda"),("rw","Kinyarwanda"),("am","Amharic"),("om","Oromo"),
    ("ha","Hausa"),("ig","Igbo"),("ln","Lingala"),("mg","Malagasy"),("ny","Nyanja"),("sn","Shona"),("wo","Wolof"),("yo","Yoruba")];
pub fn routes() -> Vec<(String,String)> {
    LANGUAGES.iter().flat_map(|(code,name)|{
        let status=if *code=="sw"{"tested; meaning errors remain"}else{"experimental; not validated"};
        [ (format!("en-{code}"),format!("English → {name} ({status})")),
          (format!("{code}-en"),format!("{name} → English ({status})")) ]
    }).collect()
}
pub fn language_names(direction:&str)->Result<(&'static str,&'static str),String>{
    for &(code,name) in LANGUAGES {
        if direction==format!("en-{code}"){return Ok(("English",name));}
        if direction==format!("{code}-en"){return Ok((name,"English"));}
    }
    Err("That language route has no installed model. It is not supported by this pack.".into())
}
pub fn payload(direction:&str,text:&str)->Result<serde_json::Value,String>{
    validate(direction,text)?;let (a,b)=language_names(direction)?;
    Ok(serde_json::json!({"model":"afrislm-local","messages":[
        {"role":"system","content":format!("You are a professional {a} to {b} translator. Your goal is to accurately convey the meaning and nuances of the original {a} text while adhering to {b} grammar, vocabulary, and cultural sensitivities. Produce only the {b} translation, without any additional explanations or commentary. ")},
        {"role":"user","content":format!("Please translate the following {a} text into {b}: {text}.\n\nTranslation:")}],
        "temperature":0,"top_k":1,"top_p":1,"seed":42,"repeat_penalty":1.0,"presence_penalty":0,
        "max_tokens":256,"cache_prompt":false,"stream":false,"chat_template_kwargs":{"enable_thinking":false}}))
}
pub fn translation_from(value:&serde_json::Value)->Result<String,String>{
    let c=&value["choices"][0];
    if c["finish_reason"]!="stop" {return Err("Translation was incomplete. Try a shorter sentence.".into());}
    let text=c["message"]["content"].as_str().unwrap_or("").trim();
    if text.is_empty(){return Err("The model returned no translation.".into());}
    Ok(text.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn nineteen_languages_are_real_routes_not_120() {
        let r=routes();assert_eq!(r.len(),38);
        assert_eq!(r[0].0,"en-sw");assert_eq!(r[1].0,"sw-en");
        for (id,_) in &r {assert!(validate(id,"Hello").is_ok());assert!(language_names(id).is_ok());}
        assert!(r.iter().any(|(id,_)|id=="en-zu"));
        assert!(r.iter().any(|(id,_)|id=="en-xh"));
        assert!(language_names("en-kik").is_err());
        assert!(language_names("zu-xh").is_err());
    }
    #[test] fn prompt_matches_existing_baseline() {
        let p=payload("en-sw","Émile said: \"hello\".\n").unwrap();
        assert_eq!(p["messages"][0]["content"],"You are a professional English to Swahili translator. Your goal is to accurately convey the meaning and nuances of the original English text while adhering to Swahili grammar, vocabulary, and cultural sensitivities. Produce only the Swahili translation, without any additional explanations or commentary. ");
        assert_eq!(p["messages"][1]["content"],"Please translate the following English text into Swahili: Émile said: \"hello\".\n.\n\nTranslation:");
        assert_eq!(p["cache_prompt"],false);assert_eq!(p["max_tokens"],256);
        assert!(payload("en-zu","Sawubona").unwrap()["messages"][0]["content"].as_str().unwrap().contains("Zulu"));
    }
    #[test] fn reject_bad_or_partial_response() {
        for v in [serde_json::json!({}),serde_json::json!({"choices":[{"finish_reason":"length","message":{"content":"cut"}}]}),serde_json::json!({"choices":[{"finish_reason":"stop","message":{"content":" "}}]})] {assert!(translation_from(&v).is_err());}
        assert_eq!(translation_from(&serde_json::json!({"choices":[{"finish_reason":"stop","message":{"content":" Émile 🦀 "}}]})).unwrap(),"Émile 🦀");
    }
    #[test] fn valid_unicode_both_directions() {
        assert!(validate("en-sw", "Émile sent a message.").is_ok());
        assert!(validate("sw-en", "Habari za asubuhi?").is_ok());
        let w=wide("Émile 🦀");
        assert_eq!(String::from_utf16(&w[..w.len()-1]).unwrap(), "Émile 🦀");
    }
    #[test] fn reject_empty_long_invalid_direction_and_nul() {
        assert!(validate("en-sw", " \n").is_err());
        assert!(validate("fr-sw", "Hello").is_err());
        assert!(validate("sw-en", &"a".repeat(601)).is_err());
        assert!(validate("sw-en", "abc\0def").is_err());
        assert!(validate("sw-en", &"é".repeat(600)).is_ok());
    }
}
