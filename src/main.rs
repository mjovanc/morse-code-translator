#[derive(Hash, Eq, PartialEq, Debug)]
struct Morse {
    letter: String,
    code: String,
}

impl Morse {
    fn new(letter: &str, code: &str) -> Morse {
        Morse {
            letter: letter.to_string(),
            code: code.to_string(),
        }
    }
}

fn create_morse_code_data() -> Vec<Morse> {
    vec![
        Morse::new("a", ".-"),
        Morse::new("b", "-..."),
        Morse::new("c", "-.-."),
        Morse::new("d", "-.."),
        Morse::new("e", "."),
        Morse::new("f", "..-."),
        Morse::new("g", "--."),
        Morse::new("h", "...."),
        Morse::new("i", ".."),
        Morse::new("j", ".---"),
        Morse::new("k", "-.-"),
        Morse::new("l", ".-.."),
        Morse::new("m", "--"),
        Morse::new("n", "-."),
        Morse::new("o", "---"),
        Morse::new("p", ".--."),
        Morse::new("q", "--.-"),
        Morse::new("r", ".-."),
        Morse::new("s", "..."),
        Morse::new("t", "-"),
        Morse::new("u", "..-"),
        Morse::new("v", "...-"),
        Morse::new("w", ".--"),
        Morse::new("x", "-..-"),
        Morse::new("y", "-.--"),
        Morse::new("z", "--.."),
        Morse::new("0", "-----"),
        Morse::new("1", ".----"),
        Morse::new("2", "..---"),
        Morse::new("3", "...--"),
        Morse::new("4", "....-"),
        Morse::new("5", "....."),
        Morse::new("6", "-...."),
        Morse::new("7", "--..."),
        Morse::new("8", "---.."),
        Morse::new("9", "----."),
        Morse::new(".", ".-.-.-"),
        Morse::new(",", "--..--"),
        Morse::new("?", "..--.."),
        Morse::new("'", ".----."),
        Morse::new("!", "-.-.--"),
        Morse::new("/", "-..-."),
        Morse::new("(", "-.--."),
        Morse::new(")", "-.--.-"),
        Morse::new("&", ".-..."),
        Morse::new(":", "---..."),
        Morse::new("=", "-...-"),
        Morse::new("+", ".-.-."),
        Morse::new("-", "-....-"),
        Morse::new("_", "..--.-"),
        Morse::new("\"", ".-..-."),
        Morse::new("$", "...-..-"),
        Morse::new("@", ".--.-."),
        Morse::new("$", "-...-"),  // prosigns
        Morse::new("$", ".-.-."),  // prosigns
        Morse::new("$", ".-...-"), // prosigns
        Morse::new("$", ".-..."),  // prosigns
        Morse::new("$", "-.-."),   // prosigns
        Morse::new("$", "-.-.-"),  // prosigns
        Morse::new("$", ".-.-"),   // prosigns
        Morse::new("à", ".--.-"),
        Morse::new("ä", ".-.-"),
        Morse::new("å", ".--.-"),
        Morse::new("ą", ".-.-"),
        Morse::new("æ", ".-.-"),
        Morse::new("ć", "-.-.."),
        Morse::new("ĉ", "-.-.-"),
        Morse::new("ç", "-.-.."),
        Morse::new("ch", "----"),
        Morse::new("đ", "-..-."),
        Morse::new("ð", "..--."),
        Morse::new("é", "..-.."),
        Morse::new("è", ".-..-"),
        Morse::new("ę", "..-.."),
        Morse::new("ĝ", "--.-."),
        Morse::new("ĥ", "-.-.-"),
        Morse::new("à", ".--.-"),
        Morse::new("ĵ", ".---."),
        Morse::new("ł", ".-..-"),
        Morse::new("ń", "--.--"),
        Morse::new("ñ", "--.--"),
        Morse::new("ó", "---."),
        Morse::new("ö", "---."),
        Morse::new("ø", "---."),
        Morse::new("ś", "...-..."),
        Morse::new("ŝ", "...-."),
        Morse::new("š", "----"),
        Morse::new("þ", ".--.."),
        Morse::new("ü", "..--"),
        Morse::new("ŭ", "..--."),
        Morse::new("ź", "--..-"),
        Morse::new("ż", "--..--"),
    ]
}

fn main() {
    let code_data: Vec<Morse> = create_morse_code_data();

    println!("{:?}", code_data);
}
