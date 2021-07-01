use std::{env, str::FromStr};

use notify_rust::{Hint, Notification};
use persian_tools::translate::{Language, Translate};
use x11_clipboard::Clipboard;

pub fn main() {
    let lang_dest = Language::from_str(&env::args().nth(1).unwrap()).unwrap();
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
            .hint(Hint::Category("translate".to_owned()))
            .show()
        {
            eprintln!("failed notify message error message : {}", e);
        }
    }
}
