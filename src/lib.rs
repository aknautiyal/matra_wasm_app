use wasm_bindgen::prelude::*;
use matra_lib::Charan;

#[wasm_bindgen]
pub fn analyze_text(lines: Vec<String>) -> String {
    let mut results = String::new();
    for (i, line) in lines.iter().enumerate() {
        let charan = Charan::from_str(&line);
        results.push_str(&format!("Line {}: {}\nAnalysis: {}\n", i + 1, line, charan.analysis()));
    }
    results
}
