use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptionInputError {
    #[error("incorrect plaintext byte value found {0}")]
    IncorrectPlaintextByteValue(u8),
    #[error("incorrect key byte value found {0}")]
    IncorrectKeyByteValue(u8),
    #[error("incorrect value for is_encryption found {0}")]
    IncorrectIsEncryptionValue(i8),
    #[error("utf8 error")]
    UTF8Error(#[from] std::string::FromUtf8Error),
}

/// encrypts or decrypts the provided input based on the i8 is_encryption
///
/// is_encryption = 1: the input is encrypted
/// is_encryption = -1: the input is decrypted
pub fn vigenere(
    input: String,
    key: String,
    is_encryption: i8,
) -> Result<String, EncryptionInputError> {
    let incorrect_byte = input.as_bytes().into_iter().find(|&&b| b < 65 || b > 90);
    if let Some(b) = incorrect_byte {
        return Err(EncryptionInputError::IncorrectPlaintextByteValue(*b));
    }
    let incorrect_byte = key.as_bytes().into_iter().find(|&&b| b < 65 || b > 90);
    if let Some(b) = incorrect_byte {
        return Err(EncryptionInputError::IncorrectKeyByteValue(*b));
    }
    if ![-1i8, 1i8].contains(&is_encryption) {
        return Err(EncryptionInputError::IncorrectIsEncryptionValue(
            is_encryption,
        ));
    }
    let lk = key.len();
    let encrypted: Vec<u8> = input
        .as_bytes()
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            slide(
                *x,
                (key.as_bytes()[i % lk] - 65) as i8 * is_encryption as i8,
            )
        })
        .collect();
    Ok(String::from_utf8(encrypted).map_err(EncryptionInputError::from)?)
}

fn slide(input: u8, key: i8) -> u8 {
    let out = input as i8 + key;
    match out {
        out if out > 90 => (out % 91 + 65) as u8,
        out if out < 65 => (91 - (65 - out)) as u8,
        _ => out as u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide_encrypt() {
        assert_eq!(90, slide(70, 20));
        assert_eq!(65, slide(90, 1));
        assert_eq!(80, slide(70, 10));
        assert_eq!(65, slide(70, 21));
        assert_eq!(75, slide(76, 25));
    }

    #[test]
    fn test_slide_decrypt() {
        assert_eq!(65, slide(70, -5));
        assert_eq!(75, slide(90, -15));
        assert_eq!(76, slide(70, -20));
        assert_eq!(65, slide(65, -26));
    }

    #[test]
    #[should_panic(expected = "IncorrectPlaintextByteValue")]
    fn test_vigenere_incorrect_bytes_input() {
        vigenere(String::from("@*(&%"), String::from("LEMON"), 1).unwrap();
    }

    #[test]
    #[should_panic(expected = "IncorrectKeyByteValue")]
    fn test_vigenere_incorrect_bytes_key() {
        vigenere(String::from("ATTACKATDAWN"), String::from("@*(&%"), 1).unwrap();
    }

    #[test]
    #[should_panic(expected = "IncorrectIsEncryptionValue")]
    fn test_vigenere_incorrect_is_encryption() {
        vigenere(String::from("ATTACKATDAWN"), String::from("LEMON"), 2).unwrap();
    }

    #[test]
    fn test_encrypted_passes() {
        assert_eq!(
            String::from("LXFOPVEFRNHR"),
            vigenere(String::from("ATTACKATDAWN"), String::from("LEMON"), 1).unwrap()
        )
    }

    #[test]
    fn test_decrypt_passes() {
        assert_eq!(
            String::from("ATTACKATDAWN"),
            vigenere(String::from("LXFOPVEFRNHR"), String::from("LEMON"), -1).unwrap()
        )
    }
}
