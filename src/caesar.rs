pub fn caesar_cipher(text: &str, shift: i8) -> String {
    let mut new_text = String::new();
    for character in text.chars() {
        if character.is_alphabetic() {
            if character.is_uppercase() {
                new_text.push(caesar_shift(character, shift));
            } else {
                let temp = caesar_shift(character.to_ascii_uppercase(), shift);
                new_text.push(temp.to_ascii_lowercase());
            }
        } else {
            new_text.push(character);
        }
    }
    new_text
}

fn caesar_shift(character: char, shift: i8) -> char {
    let mut new_letter = character as i8 + shift;

    while new_letter > 90 {
        new_letter = new_letter - 26;
    }
    while new_letter < 65 {
        new_letter = new_letter + 26;
    }
    let final_code = new_letter as u8;
    return final_code as char;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_lower() {
        let plain_text = "abc";
        let shift = 5;
        let expected_ciphertext = "fgh";

        let ciphertext = caesar_cipher(plain_text, shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_upper() {
        let plain_text = "ABC";
        let shift = 3;
        let expected_ciphertext = "DEF";

        let ciphertext = caesar_cipher(plain_text, shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_space() {
        let plain_text = "ABC DEF";
        let shift = 3;
        let expected_ciphertext = "DEF GHI";

        let ciphertext = caesar_cipher(plain_text, shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_puncuation() {
        let plain_text = "HELLO, WORLD!";
        let shift = 3;
        let expected_ciphertext = "KHOOR, ZRUOG!";

        let ciphertext = caesar_cipher(plain_text, shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_lower() {
        let plain_text = "fgh";
        let shift = 5;
        let expected_ciphertext = "abc";

        let ciphertext = caesar_cipher(plain_text, -shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_upper() {
        let plain_text = "DEF";
        let shift = 3;
        let expected_ciphertext = "ABC";

        let ciphertext = caesar_cipher(plain_text, -shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_with_space() {
        let plain_text = "DEF GHI";
        let shift = 3;
        let expected_ciphertext = "ABC DEF";

        let ciphertext = caesar_cipher(plain_text, -shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_with_puncuation() {
        let ciphertext = "KHOOR, ZRUOG!";
        let shift = 3;
        let expected_plain_text = "HELLO, WORLD!";

        let decrypted_text = caesar_cipher(ciphertext, -shift);
        assert_eq!(decrypted_text, expected_plain_text);
    }
}
