#[macro_use]
extern crate afl;
extern crate satlight;

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(source) = std::str::from_utf8(data) {
            let _ = satlight::parser::parse(source);
        }
    });
}
