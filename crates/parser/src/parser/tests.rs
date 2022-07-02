use super::Parser;
use crate::tokenizer::Tokens;

mod scripts {
    use super::*;

    macro_rules! test_script {
        ($name:ident, $path:literal) => {
            #[test]
            fn $name() {
                let contents = include_str!(concat!("../scripts/", $path));
                let mut parser = Parser::new(Tokens::new(contents, true));
                if let Err(err) = parser.ast() {
                    eprintln!("{err:#?}");
                    std::process::exit(1);
                }
            }
        };
    }

    test_script!(luaminify, "luaminify.txt");
    test_script!(profile_service, "profileservice.txt");
}

#[test]
fn pass_cases() {
    let cases = include_str!("./cases/pass.txt")
        .split("\n")
        .enumerate()
        .map(|v| (v.0 + 1, v.1));

    for (line, case) in cases {
        eprint!("line #{line}...\t");
        let mut parser = Parser::new(Tokens::new(case, true));
        if let Err(err) = parser.ast() {
            eprintln!("failed!");
            eprintln!("{err:#?}");
            std::process::exit(1);
        } else {
            eprintln!("done!");
        }
    }
}

#[test]
fn fail_cases() {
    let cases = include_str!("./cases/fail.txt")
        .split("\n")
        .enumerate()
        .map(|v| (v.0 + 1, v.1));

    for (line, case) in cases {
        eprint!("line #{line}...\t");
        let mut parser = Parser::new(Tokens::new(case, true));
        if let Ok(ast) = parser.ast() {
            eprintln!("failed!");
            eprintln!("{ast:#?}");
            std::process::exit(1);
        } else {
            eprintln!("done!");
        }
    }
}
