pub fn railfence_cipher(text: &str, rails: usize, encrypt: bool) -> String {
    let mut fence: Vec<Vec<Option<char>>> = vec![vec![None; text.len() * 2]; rails];
    let mut new_text = String::new();
    let characters: Vec<char> = text.chars().collect();
    let mut rail: usize = 0;
    let mut down: bool = false;
    for index in 0..characters.len() {
        fence[rail][index] = Some(characters[index]);
        if rail == 0 || rail == rails - 1 {
            down = !down;
        }
        rail = if down { rail + 1 } else { rail - 1 };
    }

    if encrypt {
        new_text = fence.iter().flatten().filter_map(|&char| char).collect();
    } else {
        let mut decrypted_index = 0;
        for rail in 0..rails {
            for col in 0..characters.len() {
                if fence[rail][col].is_some() {
                    fence[rail][col] = Some(characters[decrypted_index]);
                    decrypted_index += 1;
                }
            }
        }
        rail = 0;
        down = false;
        for index in 0..characters.len() {
            if fence[rail][index].is_some() {
                new_text.push(fence[rail][index].unwrap());
            }
            if rail == 0 || rail == rails - 1 {
                down = !down;
            }
            rail = if down { rail + 1 } else { rail - 1 };
        }
    }
    return new_text;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_lower() {
        let plain_text = "hello";
        let expected_cipher_text = "hoell";
        let num_rails = 3;

        let cipher_text = railfence_cipher(plain_text, num_rails, true);
        assert_eq!(cipher_text, expected_cipher_text);
    }

    #[test]
    fn encrypt_upper() {
        let plain_text = "HELLO";
        let expected_cipher_text = "HOELL";
        let num_rails = 3;

        let cipher_text = railfence_cipher(plain_text, num_rails, true);
        assert_eq!(cipher_text, expected_cipher_text);
    }
    #[test]
    fn encrypt_with_punctuation() {
        let plain_text = "HELLO!";
        let expected_plain_text = "HOEL!L";
        let num_rails = 3;

        let cipher_text = railfence_cipher(plain_text, num_rails, true);
        assert_eq!(cipher_text, expected_plain_text);
    }

    #[test]
    fn encrypt_with_space() {
        let plain_text = "HELLO WORLD!";
        let expected_plain_text = "HOREL OL!LWD";
        let num_rails = 3;

        let cipher_text = railfence_cipher(plain_text, num_rails, true);
        assert_eq!(cipher_text, expected_plain_text);
    }

    #[test]
    fn decrypt_lower() {
        let cipher_text = "hoell";
        let expected_plain_text = "hello";
        let num_rails = 3;

        let plain_text = railfence_cipher(cipher_text, num_rails, false);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_upper() {
        let cipher_text = "HOELL";
        let expected_plain_text = "HELLO";
        let num_rails = 3;

        let plain_text = railfence_cipher(cipher_text, num_rails, false);
        assert_eq!(plain_text, expected_plain_text);
    }
    #[test]
    fn decrypt_with_punctuation() {
        let cipher_text = "HOEL!L";
        let expected_plain_text = "HELLO!";
        let num_rails = 3;

        let plain_text = railfence_cipher(cipher_text, num_rails, false);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_with_space() {
        let cipher_text = "HOREL OL!LWD";
        let expected_plain_text = "HELLO WORLD!";
        let num_rails = 3;

        let plain_text = railfence_cipher(cipher_text, num_rails, false);
        assert_eq!(plain_text, expected_plain_text);
    }
}
