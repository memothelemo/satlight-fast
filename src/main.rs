use std::time::Instant;

use satlight_parser::{parser::Parser, tokenizer::Tokens};

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
    let mut parser = Parser::new(Tokens::new(&source, true));
    let now = Instant::now();
    let ast = parser.ast().unwrap();
    let elapsed = now.elapsed();
    println!("{elapsed:.2?}");
    println!("{ast:#?}");
    // #[cfg(not(debug_assertions))]
    // for _ in 0..50 {
    //     let tokens = tokenize(&source);
    //     profiling::scope!("iterating tokens");
    //     for _token in tokens {}
    // }
}
