use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum EncryptionInputError {
    #[error("incorrect plaintext byte value found {0}")]
    IncorrectPlaintextByteValue(u8),
    #[error("incorrect key byte value found {0}")]
    IncorrectKeyByteValue(u8),
    #[error("utf8 error")]
    UTF8Error(#[from] std::string::FromUtf8Error),
}

fn encrypt_string(plaintext: String, key: String) -> Result<String, EncryptionInputError> {
    let incorrect_byte = plaintext
        .as_bytes()
        .into_iter()
        .find(|&&b| b < 65 || b > 90);
    if let Some(b) = incorrect_byte {
        return Err(EncryptionInputError::IncorrectPlaintextByteValue(*b));
    }
    let incorrect_byte = key.as_bytes().into_iter().find(|&&b| b < 65 || b > 90);
    if let Some(b) = incorrect_byte {
        return Err(EncryptionInputError::IncorrectKeyByteValue(*b));
    }
    let lk = key.len();
    let encrypted: Vec<u8> = plaintext
        .as_bytes()
        .into_iter()
        .enumerate()
        .map(|(i, x)| slide(*x, key.as_bytes()[i % lk] - 65))
        .collect();
    Ok(String::from_utf8(encrypted).map_err(EncryptionInputError::from)?)
}

fn slide(input: u8, key: u8) -> u8 {
    let mut out = input + key;
    loop {
        if out < 91 {
            break out;
        }
        out = out % 91 + 65;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slide() {
        assert_eq!(90, slide(70, 20));
        assert_eq!(65, slide(90, 1));
        assert_eq!(80, slide(70, 10));
        assert_eq!(80, slide(70, 36));
        assert_eq!(89, slide(76, 65));
    }

    #[test]
    fn test_encrypted() {
        assert_eq!(
            String::from("LXFOPVEFRNHR"),
            encrypt_string(String::from("ATTACKATDAWN"), String::from("LEMON")).unwrap()
        )
    }
}
