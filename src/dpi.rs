// Layout values are 96-DPI logical pixels. Convert only at the Win32 boundary.
pub fn pixels(logical: i32, dpi: u32) -> i32 {
    ((i64::from(logical) * i64::from(dpi.max(96)) + 48) / 96) as i32
}
pub fn logical(physical: i32, dpi: u32) -> i32 {
    (i64::from(physical) * 96 / i64::from(dpi.max(96))) as i32
}
pub fn controls(width: i32, height: i32) -> [(i32, i32, i32, i32); 9] {
    let w=width.max(650);let h=height.max(500);
    [(24,22,w-240,260),(w-190,20,166,34),
     (24,66,w-48,50),(24,124,w-48,22),(24,158+(h-260)/2,w-48,22),
     (24,150,w-48,(h-270)/2),(24,184+(h-260)/2,w-48,(h-270)/2),
     (24,h-40,w-255,32),(w-225,h-42,201,30)]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn scales_for_all_common_monitor_settings_without_accumulated_rounding() {
        for dpi in [96,120,144,168,192,240,288] {
            assert_eq!(pixels(96,dpi),dpi as i32);
            assert_eq!(logical(pixels(720,dpi),dpi),720);
            assert_eq!(pixels(0,dpi),0);
            let rects=controls(900,720);
            for (x,y,w,h) in rects {
                assert!(pixels(x,dpi)>=0 && pixels(y,dpi)>=0);
                assert!(pixels(w,dpi)>0 && pixels(h,dpi)>0);
            }
            // Translation fields stay separate and above the status/buttons.
            let input=rects[5];let output=rects[6];let status=rects[7];
            assert!(pixels(input.1+input.3,dpi)<pixels(rects[4].1,dpi));
            assert!(pixels(output.1+output.3,dpi)<pixels(status.1,dpi));
        }
    }
}
