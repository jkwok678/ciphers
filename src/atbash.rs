pub fn atbash_cipher(input: &str) -> String {
    let mut new_text = String::new();
    for character in input.chars() {
        if (character.is_alphabetic()) {
            if (character.is_uppercase()) {
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
    fn atbash_cipher_encrypt_simple() {
        let plaintext = "0ABCD!";
        let expected_ciphertext = "0ZYXW!";

        let ciphertext = atbash_cipher(plaintext);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn caesar_cipher_encrypt_simple_lower() {
        let plaintext = "0abc!";
        let expected_ciphertext = "0zyx!";

        let ciphertext = atbash_cipher(plaintext);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn atbash_cipher_encrypt() {
        let plaintext = "HELLO, WORLD!";
        let expected_ciphertext = "SVOOL, DLIOW!";

        let ciphertext = atbash_cipher(plaintext);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn atbash_cipher_decrypt() {
        let ciphertext = "SVOOL, DLIOW!";
        let expected_plaintext = "HELLO, WORLD!";

        let decrypted_text = atbash_cipher(ciphertext);
        assert_eq!(decrypted_text, expected_plaintext);
    }
}
