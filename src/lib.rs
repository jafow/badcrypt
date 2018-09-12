pub trait FromHex: Sized {
    type Error;

    fn from_hex<T: AsRef<[u8]>>(s: T) -> Result<Self, Self::Error>;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FromHexError {
    InvalidHexCharacter {
        c: char,
        index: usize,
    },
    InvalidHexLength,
}

impl FromHex for Vec<u8> {
    type Error = FromHexError;
    
    fn from_hex<T: AsRef<[u8]>>(s: T) -> Result<Self, Self::Error> {
        let bytes = s.as_ref();
        let mut b = Vec::with_capacity(bytes.len() / 2);
        let mut modulus = 0;
        let mut buf = 08;

        for (i, byte) in bytes.iter().enumerate() {
            buf <<= 4;

            match *byte {
                b'A'...b'F' => buf |= byte - b'A' + 10,
                b'a'...b'f' => buf |= byte - b'a' + 10,
                b'0'...b'9' => buf |= byte - b'0',
                _ => {
                    return Err(FromHexError::InvalidHexCharacter {
                        c: bytes[i] as char,
                        index: i,
                    })
                },
            }

            modulus += 1;
            if modulus == 2 {
                modulus = 0;
                b.push(buf);
            }
        }

        match modulus {
            0 => Ok(b.into_iter().collect()),
            _ => Err(FromHexError::InvalidHexLength),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FromHex, FromHexError};

    #[test]
    fn test_decode_hex_to_bits() {
        assert_eq!(Vec::from_hex("68656C6C6F").unwrap(), b"hello");
    }

    #[test]
    fn test_odd_numbered_hex() {
        assert_eq!(Vec::from_hex("6656C6C6F").unwrap_err(),
            FromHexError::InvalidHexLength
        );
    }

    #[test]
    fn test_whitespace_in_hex() {
        assert_eq!(Vec::from_hex("6865 6C6C6F").unwrap_err(),
            FromHexError::InvalidHexCharacter {
                c: ' ',
                index: 4,
            },
        );
    }
}
