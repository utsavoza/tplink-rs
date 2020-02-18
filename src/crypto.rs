const INITIAL_KEY: u8 = 0xAB;

/// Encrypts input bytes where each byte is XOR'ed with the previous encrypted byte.
pub(crate) fn encrypt(bytes: &[u8]) -> Vec<u8> {
    let mut key = INITIAL_KEY;
    bytes
        .iter()
        .map(|byte| {
            key ^= *byte;
            key
        })
        .collect()
}

/// Encrypts input bytes with a 4 bytes big-endian length header where each byte is
/// XOR'ed with the previous encrypted byte.
pub(crate) fn encrypt_with_header(bytes: &[u8]) -> Vec<u8> {
    let num_bytes = bytes.len();
    let header = (num_bytes as u32).to_be_bytes();
    let mut buf = Vec::with_capacity(header.len() + num_bytes);
    buf.extend(&header);
    buf.extend(&encrypt(bytes));
    buf
}

/// Decrypts input bytes where each byte is XOR'ed with the previous encrypted byte.
pub(crate) fn decrypt(bytes: &[u8]) -> Vec<u8> {
    let mut key = INITIAL_KEY;
    bytes
        .iter()
        .map(|byte| {
            let xor = *byte ^ key;
            key = *byte;
            xor
        })
        .collect()
}

/// Decrypts input bytes that has a 4 bytes big-endian length header where each byte is
/// XOR'ed with the previous encrypted byte.
pub(crate) fn decrypt_with_header(bytes: &[u8]) -> Vec<u8> {
    decrypt(
        bytes
            .iter()
            .skip(4)
            .copied()
            .collect::<Vec<u8>>()
            .as_slice(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt(b"hello"), vec![195, 166, 202, 166, 201]);
    }

    #[test]
    fn test_encrypt_with_unicode_chars() {
        assert_eq!(
            encrypt("{'hello': 'नमस्ते'}".as_bytes()),
            vec![
                208, 247, 159, 250, 150, 250, 149, 178, 136, 168, 143, 111, 203, 99, 131, 39, 137,
                105, 205, 117, 149, 48, 189, 93, 249, 93, 189, 24, 159, 184, 197
            ]
        );
    }

    #[test]
    fn test_encrypt_with_header() {
        assert_eq!(
            encrypt_with_header(b"hello"),
            vec![0, 0, 0, 5, 195, 166, 202, 166, 201]
        );
    }

    #[test]
    fn test_encrypt_with_header_with_unicode_chars() {
        assert_eq!(
            encrypt_with_header("{'hello': 'नमस्ते'}".as_bytes()),
            vec![
                0, 0, 0, 31, 208, 247, 159, 250, 150, 250, 149, 178, 136, 168, 143, 111, 203, 99,
                131, 39, 137, 105, 205, 117, 149, 48, 189, 93, 249, 93, 189, 24, 159, 184, 197
            ]
        );
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt(&[195, 166, 202, 166, 201]), b"hello");
    }

    #[test]
    fn test_decrypt_with_unicode_chars() {
        assert_eq!(
            decrypt(&[
                208, 247, 159, 250, 150, 250, 149, 178, 136, 168, 143, 111, 203, 99, 131, 39, 137,
                105, 205, 117, 149, 48, 189, 93, 249, 93, 189, 24, 159, 184, 197
            ]),
            "{'hello': 'नमस्ते'}".as_bytes(),
        );
    }

    #[test]
    fn test_decrypt_with_header() {
        assert_eq!(
            decrypt_with_header(&[0, 0, 0, 5, 195, 166, 202, 166, 201]),
            b"hello"
        )
    }

    #[test]
    fn test_decrypt_with_header_with_unicode_chars() {
        assert_eq!(
            decrypt_with_header(&[
                0, 0, 0, 31, 208, 247, 159, 250, 150, 250, 149, 178, 136, 168, 143, 111, 203, 99,
                131, 39, 137, 105, 205, 117, 149, 48, 189, 93, 249, 93, 189, 24, 159, 184, 197
            ]),
            "{'hello': 'नमस्ते'}".as_bytes(),
        );
    }
}
