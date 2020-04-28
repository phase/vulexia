mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const POS: &'static str = include_str!("../wordlist/pos/part-of-speech.txt");

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn color(i: String) -> String {
    let mut output = String::new();
    for word in i.split(" ").into_iter() {
        let stripped_word = word.replace(&['(', ')', ',', '\"', '.', ';', ':', '\'', '!', '?', '^', '*', '#'][..], "");
        if stripped_word.len() > 0 {
            // obviously inefficient
            if let Some(line) = POS.split::<&str>("\n").filter(|e| e.starts_with(&stripped_word)).nth(0) {
                let pos: &str = line.split("\t").nth(1).expect("bad format");
                let mut c: char = pos.chars().nth(0).expect("bad format");
                if c == '|' {
                    c = pos.chars().nth(1).expect("bad format");
                }

                let pos = match c {
                    'N' | 'p' | 'h' => "noun",
                    'V' | 't' | 'i' => "verb",
                    'A' => "adjective",
                    'v' => "adverb",
                    'C' => "conjunction",
                    'P' => "preposition",
                    '!' => "interjection",
                    'r' => "pronoun",
                    'D' | 'I' => "article",
                    'o' => "nominative",
                    _ => ""
                };

                output.push_str(&format!("<span class=\"{}\">{}</span> ", pos, word));
            } else {
                output.push_str(&format!("{} ", word));
            }
        } else {
            output.push_str(&format!("{} ", word));
        }
    }
    output
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, vulexia!");
}
