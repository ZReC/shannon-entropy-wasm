use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

/// Calculates the shannon entropy of 's'.
/// https://en.wiktionary.org/wiki/Shannon_entropy
#[wasm_bindgen]
pub fn shannon_entropy(s: &str) -> f32 {
    if s.is_empty() {
        return 0.0;
    }
    let mut char_map: BTreeMap<char, usize> = BTreeMap::new();
    let mut ascii_map: [usize; 128] = [0; 128];
    let mut s_len = 0;

    for ch in s.chars() {
        s_len += 1;
        if ch.is_ascii() {
            ascii_map[ch as usize] += 1;
        } else {
            *char_map.entry(ch).or_insert(0) += 1;
        }
    }

    let s_len = (s_len as f32).round();

    let result = char_map
        .values()
        .chain(ascii_map.iter())
        .fold(0.0, |acc, &c| match c {
            0 => acc,
            c => acc + (c as f32 * (c as f32 / s_len).ln()),
        })
        .abs();

    result / (s_len * std::f32::consts::LN_2)
}
