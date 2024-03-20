use std::{env, str::FromStr};

use notify_rust::Notification;
use persian_tools::translate::{Language, Translate};
use x11_clipboard::Clipboard;

pub fn main() {
    let arg = env::args()
        .nth(1)
        .expect("please pass target language argument");
    let lang_dest = Language::from_str(&arg).expect("please set valid target language");
    let clipboard = Clipboard::new().unwrap();

    while let Ok(content_src) = clipboard.load_wait(
        clipboard.getter.atoms.clipboard,
        clipboard.getter.atoms.utf8_string,
        clipboard.getter.atoms.property,
    ) {
        let content_src = match std::str::from_utf8(&content_src) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed parse string error message : {}", e);
                continue;
            }
        };
        let content_dest = match content_src.translate(lang_dest) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("failed translate to {:?} error message : {}", lang_dest, e);
                continue;
            }
        };
        if let Err(e) = Notification::new()
            .summary(&content_src)
            .body(&content_dest)
            .icon("Translate")
            .appname("Translate")
            .show()
        {
            eprintln!("failed notify message error message : {}", e);
        }
    }
}
