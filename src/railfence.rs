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
    fn railfence_cipher_encrypt_simple() {
        let plain_text = "HELLO WORLD";
        let expected_ciphertext = "HOREL OLLWD";
        let num_rails = 3;

        let ciphertext = railfence_cipher(plain_text, num_rails, true);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn railfence_cipher_decrypt_simple() {
        let ciphertext = "HOREL OLLWD";
        let expected_plain_text = "HELLO WORLD";
        let num_rails = 3;

        let decoded_text = railfence_cipher(ciphertext, num_rails, false);
        assert_eq!(decoded_text, expected_plain_text);
    }
}
