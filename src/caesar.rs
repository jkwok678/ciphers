pub fn caesar_cipher(text: &str, shift: i8) -> String {
    let mut new_text = String::new();
    for character in text.chars() {
        if character.is_alphabetic() {
            let mut new_letter = character as i8 + shift;
            while new_letter > 90 {
                new_letter = new_letter - 26;
            }
            while new_letter < 65 {
                new_letter = new_letter + 26;
            }
            let final_letter = new_letter as u8;
            new_text.push(final_letter as char);
        } else {
            new_text.push(character);
        }
    }
    new_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn caesar_cipher_encrypt_simple() {
        let plaintext = "0ABCD!";
        let shift = 26;
        let expected_ciphertext = "0ABCD!";

        let ciphertext = caesar_cipher(plaintext, shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn caesar_cipher_encrypt() {
        let plaintext = "HELLO, WORLD!";
        let shift = 3;
        let expected_ciphertext = "KHOOR, ZRUOG!";

        let ciphertext = caesar_cipher(plaintext, shift);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn caesar_cipher_decrypt() {
        let ciphertext = "KHOOR, ZRUOG!";
        let shift = 3;
        let expected_plaintext = "HELLO, WORLD!";

        let decrypted_text = caesar_cipher(ciphertext, -shift);
        assert_eq!(decrypted_text, expected_plaintext);
    }
}
