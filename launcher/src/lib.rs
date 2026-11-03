pub mod setup;
pub fn response_offset(status:u32,range:Option<&str>,offset:u64,expected:u64)->Result<u64,String>{
 if expected==0||offset>expected{return Err("Invalid download size.".into());}
 if status==200{return Ok(0)}
 if status!=206{return Err(format!("Download server returned HTTP {status}. Your saved progress is kept."))}
 let range=range.and_then(|s|s.strip_prefix("bytes ")).ok_or("Missing resume information.")?;
 let (span,total)=range.split_once('/').ok_or("Invalid resume information.")?;
 let (start,end)=span.split_once('-').ok_or("Invalid resume range.")?;
 let start=start.parse::<u64>().map_err(|_|"Invalid resume start.")?;
 let end=end.parse::<u64>().map_err(|_|"Invalid resume end.")?;
 let total=total.parse::<u64>().map_err(|_|"Invalid download total.")?;
 if start!=offset||total!=expected||end<start||end.checked_add(1)!=Some(expected){return Err("The server returned a different resume range. Nothing was appended.".into());}
 Ok(offset)
}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn resume_requires_matching_range(){
   assert_eq!(response_offset(206,Some("bytes 4-9/10"),4,10).unwrap(),4);
   assert!(response_offset(206,Some("bytes 3-9/10"),4,10).is_err());
   assert!(response_offset(206,None,4,10).is_err());
   assert!(response_offset(206,Some("bytes 4-9/11"),4,10).is_err());
 }
 #[test]fn ignored_range_restarts(){assert_eq!(response_offset(200,None,4,10).unwrap(),0);}
 #[test]fn reject_failure_and_bad_range(){
  for code in [302,403,404,416,500]{assert!(response_offset(code,None,4,10).is_err());}
  for r in ["bytes 4-3/10","bytes 4-10/10","bytes 4-8/10","junk"]{assert!(response_offset(206,Some(r),4,10).is_err());}
 }
}
