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

fn encrypt(plaintext: String, key: String) -> Result<String, EncryptionInputError> {
    let encyption = |i: u8, k: u8| {
        let out = i + k;
        return if out > 90 { out % 91 + 65 } else { out };
    };
    vigenere(plaintext, key, encyption)
}

fn decrypt(plaintext: String, key: String) -> Result<String, EncryptionInputError> {
    let decrypt = |i: u8, k: u8| {
        let out = i - k;
        return if out < 65 { 91 - (65 - out) } else { out };
    };
    vigenere(plaintext, key, decrypt)
}

/// encrypts or decrypts the provi, dd input based on the closure received
pub fn vigenere<F>(input: String, key: String, function: F) -> Result<String, EncryptionInputError>
where
    F: Fn(u8, u8) -> u8,
{
    let incorrect_byte = input.as_bytes().into_iter().find(|&&b| b < 65 || b > 90);
    if let Some(b) = incorrect_byte {
        return Err(EncryptionInputError::IncorrectPlaintextByteValue(*b));
    }
    let incorrect_byte = key.as_bytes().into_iter().find(|&&b| b < 65 || b > 90);
    if let Some(b) = incorrect_byte {
        return Err(EncryptionInputError::IncorrectKeyByteValue(*b));
    }
    let lk = key.len();
    let encrypted: Vec<u8> = input
        .as_bytes()
        .into_iter()
        .enumerate()
        .map(|(i, x)| function(*x, key.as_bytes()[i % lk] - 65))
        .collect();
    Ok(String::from_utf8(encrypted).map_err(EncryptionInputError::from)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "IncorrectPlaintextByteValue")]
    fn test_vigenere_incorrect_bytes_input() {
        vigenere(String::from("@*(&%"), String::from("LEMON"), |i, k| i * k).unwrap();
    }

    #[test]
    #[should_panic(expected = "IncorrectKeyByteValue")]
    fn test_vigenere_incorrect_bytes_key() {
        vigenere(
            String::from("ATTACKATDAWN"),
            String::from("@*(&%"),
            |i, k| i * k,
        )
        .unwrap();
    }

    #[test]
    fn test_encrypted_passes() {
        assert_eq!(
            String::from("LXFOPVEFRNHR"),
            encrypt(String::from("ATTACKATDAWN"), String::from("LEMON")).unwrap()
        )
    }

    #[test]
    fn test_decrypt_passes() {
        assert_eq!(
            String::from("ATTACKATDAWN"),
            decrypt(String::from("LXFOPVEFRNHR"), String::from("LEMON")).unwrap()
        )
    }
}
