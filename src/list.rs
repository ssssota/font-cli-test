#[cfg(not(any(target_os = "macos", target_os = "ios",)))]
mod sys;
#[cfg(not(any(target_os = "macos", target_os = "ios",)))]
use sys::get_all;

#[cfg(any(target_os = "macos", target_os = "ios",))]
mod coretext;
#[cfg(any(target_os = "macos", target_os = "ios",))]
use coretext::get_all;

pub fn list_fonts() {
    get_all().iter().for_each(|path| println!("{}", path));
}
