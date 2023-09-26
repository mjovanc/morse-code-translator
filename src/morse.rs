#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Morse {
    pub letter: String,
    pub code: String,
}

impl Morse {
    pub fn new(letter: &str, code: &str) -> Morse {
        Morse {
            letter: letter.to_string(),
            code: code.to_string(),
        }
    }
}
