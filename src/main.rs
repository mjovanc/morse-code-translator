use std::env;

mod morse;

use morse::Morse;

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
        Morse::new("SK", "... -.-"),
        Morse::new("HH", ".... ...."),
        Morse::new("KN", "-.- -."),
        Morse::new("CT", "-.-. -"),
        Morse::new("RN", ".-. -."),
        Morse::new("AS", "... ... ... ..."),
        Morse::new("Wait", ".-- .- .. -"),
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

fn char_to_morse(c: char, morse_data: &Vec<Morse>) -> Option<&str> {
    for morse in morse_data.iter() {
        if morse.letter.chars().next() == Some(c) {
            return Some(&morse.code);
        }
    }
    None
}


fn text_to_morse(text: &str, morse_data: &Vec<Morse>) -> String {
    let mut morse_code = String::new();
    
    let words: &Vec<_> = &text.split(' ').collect();
    println!("args {:?}", words);

    //Check every word and see if it is a Prosign, if not, decode chars.
    for word in words {
        let new = word.to_string();
        let mut prosign = false;
        for morse in morse_data.iter() {
            if new == morse.letter {
                println!("Morse special character {:?}", &morse.letter);
                morse_code.push_str(&morse.code);
                prosign = true;
                add_space(&mut morse_code);
            }
    
        }

        if prosign == true { continue ;}
        let word = word.to_lowercase();
        //Decode chars
        for c in word.chars() {
            if let Some(code) = char_to_morse(c, morse_data) {
                morse_code.push_str(code);
                morse_code.push(' ');
            }
        }
        
        //Add space after word is processed
        add_space(&mut morse_code);

        fn add_space(morse_code: &mut String) {
            for i in 0..7 {
                morse_code.push(' ');
            }
        }
    }


    morse_code
        .trim_matches(|c| c == '"' || c == '\'')
        .to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let morse_data: Vec<Morse> = create_morse_code_data();

    if args.len() != 3 {
        eprintln!("Usage: {} (--text|-t) <text>", args[0]);
        std::process::exit(1);
    }

    let option = &args[1];
    if option != "--text" && option != "-t" {
        eprintln!("Usage: {} (--text|-t) <text>", args[0]);
        std::process::exit(1);
    }

    let text = &args[2];
    let morse_code_text = &text_to_morse(text, &morse_data);

    println!("{}", morse_code_text);
}
