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
        let expected_cipher_text = "zyx";

        let cipher_text = atbash_cipher(plain_text);
        assert_eq!(cipher_text, expected_cipher_text);
    }

    #[test]
    fn encrypt_upper() {
        let plain_text = "ABC";
        let expected_cipher_text = "ZYX";

        let cipher_text = atbash_cipher(plain_text);
        assert_eq!(cipher_text, expected_cipher_text);
    }

    #[test]
    fn encrypt_with_space() {
        let plain_text = "ABC DEF";
        let expected_cipher_text = "ZYX WVU";

        let cipher_text = atbash_cipher(plain_text);
        assert_eq!(cipher_text, expected_cipher_text);
    }

    #[test]
    fn encrypt_with_puncuation() {
        let plain_text = "HELLO, WORLD!";
        let expected_cipher_text = "SVOOL, DLIOW!";

        let cipher_text = atbash_cipher(plain_text);
        assert_eq!(cipher_text, expected_cipher_text);
    }

    #[test]
    fn decrypt_lower() {
        let cipher_text = "zyx";
        let expected_plain_text = "abc";

        let plain_text = atbash_cipher(cipher_text);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_upper() {
        let cipher_text = "ZYX";
        let expected_plain_text = "ABC";

        let plain_text = atbash_cipher(cipher_text);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_with_space() {
        let cipher_text = "ZYX WVU";
        let expected_plain_text = "ABC DEF";

        let plain_text = atbash_cipher(cipher_text);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_with_puncuation() {
        let cipher_text = "SVOOL, DLIOW!";
        let expected_plain_text = "HELLO, WORLD!";

        let plain_text = atbash_cipher(cipher_text);
        assert_eq!(plain_text, expected_plain_text);
    }
}
