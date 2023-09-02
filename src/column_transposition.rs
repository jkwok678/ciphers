pub fn column_transposition_cipher(text: &str, key: &str) -> String {
    let mut new_text = String::new();
    let key_chars: Vec<_> = key.chars().collect();
    let key_len = key.len();
    let columns = key.len();
    let mut rows = text.len() / columns;
    if (text.len() % columns) != 0 {
        rows += 1;
    }
    let mut grid: Vec<Vec<Option<char>>> = vec![vec![None; rows]; columns];
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
        let expected_ciphertext = "EORHLODLWL";

        let ciphertext = column_transposition_cipher(plain_text, key);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_lower() {
        let plain_text = "helloworld";
        let key = "KEY";
        let expected_ciphertext = "eorhlodlwl";

        let ciphertext = column_transposition_cipher(plain_text, key);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_space() {
        let plain_text = "HELLO WORLD";
        let key = "KEY";
        let expected_ciphertext = "EOODHLWLL R";

        let ciphertext = column_transposition_cipher(plain_text, key);
        assert_eq!(ciphertext, expected_ciphertext);
    }

    #[test]
    fn encrypt_with_puncuation() {
        let plain_text = "hello, world!";
        let key = "KEY";
        let expected_ciphertext = "eowlhl r!l,od";

        let ciphertext = column_transposition_cipher(plain_text, key);
        assert_eq!(ciphertext, expected_ciphertext);
    }
}