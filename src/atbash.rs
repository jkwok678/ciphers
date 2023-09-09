pub fn atbash_cipher(input: &str) -> String {
    let mut new_text = String::new();
    for character in input.chars() {
        if character.is_alphabetic() {
            if character.is_uppercase() {
                new_text.push(atbash_shift(character));
            } else {
                let temp = atbash_shift(character.to_ascii_uppercase());
                new_text.push(temp.to_ascii_lowercase());
            }
        } else {
            new_text.push(character);
        }
    }
    new_text
}

fn atbash_shift(character: char) -> char {
    let alphabet: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let reversed_alphabet: Vec<char> = "ZYXWVUTSRQPONMLKJIHGFEDCBA".chars().collect();
    let index = alphabet.iter().position(|&c| c == character).unwrap();
    return *reversed_alphabet.get(index).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_lower() {
        let plain_text = "abc";
        let expected_ciphertext = "zyx";

        let ciphertext = atbash_cipher(plain_text);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_upper() {
        let plain_text = "ABC";
        let expected_ciphertext = "ZYX";

        let ciphertext = atbash_cipher(plain_text);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_space() {
        let plain_text = "ABC DEF";
        let expected_ciphertext = "ZYX WVU";

        let ciphertext = atbash_cipher(plain_text);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_puncuation() {
        let plain_text = "HELLO, WORLD!";
        let expected_ciphertext = "SVOOL, DLIOW!";

        let ciphertext = atbash_cipher(plain_text);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_lower() {
        let cipher_text = "zyx";
        let expected_ciphertext = "abc";

        let ciphertext = atbash_cipher(cipher_text);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_upper() {
        let cipher_text = "ZYX";
        let expected_ciphertext = "ABC";

        let ciphertext = atbash_cipher(cipher_text);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_with_space() {
        let cipher_text = "ZYX WVU";
        let expected_ciphertext = "ABC DEF";

        let ciphertext = atbash_cipher(cipher_text);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_with_puncuation() {
        let ciphertext = "SVOOL, DLIOW!";
        let expected_plain_text = "HELLO, WORLD!";

        let decrypted_text = atbash_cipher(ciphertext);
        assert_eq!(decrypted_text, expected_plain_text);
    }
}
