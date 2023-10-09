pub fn column_transposition_cipher(text: &str, key: &str, encrypt: bool) -> String {
    let mut new_text = String::new();
    let key_chars: Vec<_> = key.chars().collect();
    let key_len = key.len();
    let columns = key.len();
    let mut rows = text.len() / columns;
    if (text.len() % columns) != 0 {
        rows += 1;
    }
    let mut grid: Vec<Vec<Option<char>>> = vec![vec![None; rows]; columns];
    if encrypt {
        for (i, c) in text.chars().enumerate() {
            let col = i % columns;
            let row = i / columns;
            grid[col][row] = Some(c);
        }
        let mut sorted_columns: Vec<_> = (0..key_len).collect();
        sorted_columns.sort_by_key(|&col| &key_chars[col]);
        for col in sorted_columns {
            for row in 0..rows {
                if let Some(c) = grid[col][row] {
                    new_text.push(c);
                } else {
                    new_text.push('_')
                }
            }
        }
    } else {
        let mut sorted_columns: Vec<_> = (0..key_len).collect();
        sorted_columns.sort_by_key(|&col| &key_chars[col]);
        let mut index = 0;
        for col in sorted_columns {
            for row in 0..rows {
                if index < text.len() {
                    grid[col][row] = Some(text.chars().nth(index).unwrap());
                    index += 1;
                }
            }
        }
        for row in 0..rows {
            for col in 0..columns {
                if let Some(c) = grid[col][row] {
                    if c != '_' {
                        new_text.push(c);
                    }
                }
            }
        }
    }

    new_text
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_upper() {
        let plain_text = "HELLOWORLD";
        let key = "KEY";
        let expected_ciphertext = "EOR_HLODLWL_";

        let ciphertext = column_transposition_cipher(plain_text, key, true);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_lower() {
        let plain_text = "helloworld";
        let key = "KEY";
        let expected_ciphertext = "eor_hlodlwl_";

        let ciphertext = column_transposition_cipher(plain_text, key, true);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_space() {
        let plain_text = "HELLO WORLD";
        let key = "KEY";
        let expected_ciphertext = "EOODHLWLL R_";

        let ciphertext = column_transposition_cipher(plain_text, key, true);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_puncuation() {
        let plain_text = "hello, world!";
        let key = "KEY";
        let expected_ciphertext = "eowl_hl r!l,od_";

        let ciphertext = column_transposition_cipher(plain_text, key, true);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn decrypt_upper() {
        let cipher_text = "EOR_HLODLWL_";
        let key = "KEY";
        let expected_plain_text = "HELLOWORLD";

        let plain_text = column_transposition_cipher(cipher_text, key, false);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_lower() {
        let cipher_text = "eor_hlodlwl_";
        let key = "KEY";
        let expected_plain_text = "helloworld";

        let plain_text = column_transposition_cipher(cipher_text, key, false);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_with_space() {
        let cipher_text = "EOODHLWLL R_";
        let key = "KEY";
        let expected_plain_text = "HELLO WORLD";

        let plain_text = column_transposition_cipher(cipher_text, key, false);
        assert_eq!(plain_text, expected_plain_text);
    }

    #[test]
    fn decrypt_with_puncuation() {
        let cipher_text = "eowl_hl r!l,od_";
        let key = "KEY";
        let expected_plain_text = "hello, world!";

        let plain_text = column_transposition_cipher(cipher_text, key, false);
        assert_eq!(plain_text, expected_plain_text);
    }
}
