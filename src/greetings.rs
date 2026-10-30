// Reference-checked standalone greetings. Local candidate only.
pub fn lookup(direction: &str, text: &str) -> Option<&'static str> {
 if text.contains(['\n','\r']) { return None; }
 let text=text.trim().trim_end_matches(['.','!','?']).trim();
 let key=text.split(|c| c==' ' || c=='\t').filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ").to_ascii_lowercase();
 match (direction,key.as_str()) {
  ("en-rw","good morning") => Some("Mwaramutse."),
  ("rw-en","mwaramutse") => Some("Good morning."),
  ("en-am","good morning") => Some("\u{12a5}\u{1295}\u{12f0}\u{121d}\u{1295} \u{12a0}\u{12f0}\u{1229}?"),
  ("am-en","\u{12a5}\u{1295}\u{12f0}\u{121d}\u{1295} \u{12a0}\u{12f0}\u{1229}") => Some("Good morning."),
  ("en-mg","how are you") => Some("Manao ahoana?"),
  ("mg-en","manao ahoana") => Some("How are you?"),
  _ => None,
 }
}
#[cfg(test)] mod tests {
 use super::*;
 #[test] fn six_reference_checked_routes() {
  for (d,s,t) in [
   ("en-rw","Good morning","Mwaramutse."),("rw-en","Mwaramutse","Good morning."),
   ("en-am","Good morning","\u{12a5}\u{1295}\u{12f0}\u{121d}\u{1295} \u{12a0}\u{12f0}\u{1229}?"),
   ("am-en","\u{12a5}\u{1295}\u{12f0}\u{121d}\u{1295} \u{12a0}\u{12f0}\u{1229}","Good morning."),
   ("en-mg","How are you","Manao ahoana?"),("mg-en","Manao ahoana","How are you?")
  ] {assert_eq!(lookup(d,s),Some(t));assert_eq!(lookup(d,&format!("  {}?!  ",s.to_ascii_uppercase())),Some(t));}
 }
 #[test] fn whole_message_only_and_lingala_excluded() {
  for (d,s) in [("en-ln","Good morning"),("ln-en","Mbote na ntongo"),("en-rw","Good morning, Rehema."),("en-rw","She said Good morning."),("en-rw","Good morning. Hello."),("en-rw","Good\nmorning"),("en-rw",""),("sw-en","Good morning"),("en-rw","I borrowed a book.")] {assert_eq!(lookup(d,s),None);}
 }
}

