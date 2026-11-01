// Reviewed, whole-message phrases only. No name substitution or word replacement.
pub fn lookup(direction: &str, text: &str) -> Option<&'static str> {
 let normalized=text.split_whitespace().collect::<Vec<_>>().join(" ");
 let key=normalized.strip_suffix('.').unwrap_or(&normalized);
 match (direction,key) {
  ("en-sw","I borrowed a ladder from Rehema")=>Some("Niliazima ngazi kutoka kwa Rehema."),
  ("sw-en","Niliazima ngazi kutoka kwa Rehema")=>Some("I borrowed a ladder from Rehema."),
  ("en-sw","I lent a ladder to Rehema")=>Some("Nilimwazimisha Rehema ngazi."),
  ("sw-en","Nilimwazimisha Rehema ngazi")=>Some("I lent a ladder to Rehema."),
  ("en-sw","Rehema has returned the ladder")=>Some("Rehema amerudisha ngazi."),
  ("sw-en","Rehema amerudisha ngazi")=>Some("Rehema has returned the ladder."),
  _=>None,
 }
}

#[cfg(test)] mod tests {
 use super::*;
 #[test] fn reviewed_lending_and_borrowing() {
  assert_eq!(lookup("sw-en","Niliazima ngazi kutoka kwa Rehema."),Some("I borrowed a ladder from Rehema."));
  assert_eq!(lookup("sw-en","Nilimwazimisha Rehema ngazi."),Some("I lent a ladder to Rehema."));
  assert_eq!(lookup("en-sw","Rehema has returned the ladder."),Some("Rehema amerudisha ngazi."));
 }
 #[test] fn unreviewed_inputs_do_not_match() {
  for s in ["I did not borrow a ladder from Rehema.","I borrowed a ladder from Neema.","I borrowed a ladder from Rehema. Extra text.","I borrowed a ladder from Rehema?","i borrowed a ladder from Rehema.","I borrowed a ladder from Rehema.."] { assert_eq!(lookup("en-sw",s),None); }
 }
}
