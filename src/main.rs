use std::time::Instant;

use satlight_parser::tokenizer::tokenize;

fn main() {
    // #[cfg(feature = "profiling")]
    // {
    //     tracy_client::Client::start();
    //     std::thread::sleep(std::time::Duration::from_secs(2));
    // }
    let source = std::fs::read_to_string("src/sample.txt").unwrap();
    // #[cfg(debug_assertions)]
    // {
    //     let tokens = tokenize(&source);
    //     for token in tokens {
    //         if token.token_type().is_invalid() {
    //             panic!("{token:#?}");
    //         } else {
    //             println!("{token:#?}");
    //         }
    //     }
    // }
    let now = Instant::now();
    for _token in tokenize(&source) {}
    let elapsed = now.elapsed();
    println!("{elapsed:.2?}");
    // #[cfg(not(debug_assertions))]
    // for _ in 0..50 {
    //     let tokens = tokenize(&source);
    //     profiling::scope!("iterating tokens");
    //     for _token in tokens {}
    // }
}
