const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn base64_encode(val: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity((val.len()*4usize).div_ceil(3));
    let chunks = val.chunks_exact(3);
    let rem = chunks.remainder();
    for s in chunks {
        let buf = u32::from_be_bytes([0, s[0], s[1], s[2]]);
        // In any case, the index is <64, so even if usize is 16-bit, this cast won't truncate
        out.extend::<&[u8]>(&[
            ALPHABET[(buf>>18&0b111111) as usize],
            ALPHABET[(buf>>12&0b111111) as usize],
            ALPHABET[(buf>> 6&0b111111) as usize],
            ALPHABET[(buf    &0b111111) as usize]
        ])
    }
    let rl = rem.len();
    if rl == 1 {
        out.extend::<&[u8]>(&[
            ALPHABET[(rem[0]>>2&0b111111) as usize],
            ALPHABET[(rem[0]<<4&0b111111) as usize],
            b'=',
            b'='
        ])
    } else if rl == 2 {
        let buf = u32::from_be_bytes([0, 0, rem[0], rem[1]]);
        // In any case, the index is <64, so even if usize is 16-bit, this cast won't truncate
        out.extend::<&[u8]>(&[
            ALPHABET[(buf>>10&0b111111) as usize],
            ALPHABET[(buf>> 4&0b111111) as usize],
            ALPHABET[(buf<< 2&0b111111) as usize],
            b'='
        ])
    } // else: rl is 0, we don't care
    out
}

pub fn base64_decode(val: &[u8]) -> Result<Vec<u8>, &'static str> {
    let mut out: Vec<u8> = Vec::with_capacity((val.len()*3usize)/(4));
    let temp = val.iter().filter_map(|&e| ALPHABET.iter().position(|&v| v == e)).collect::<Vec<_>>();
    let chunks = temp.chunks_exact(4);
    let rem = chunks.remainder();
    for s in chunks {
        // In any case, the index is <64, so even if usize is 16-bit, this cast won't truncate
        out.extend::<&[u8]>(&[
            (s[0] as u8)<<2 | (s[1] as u8) >>4,
            (s[1] as u8)<<4 | (s[2] as u8) >>2,
            (s[2] as u8)<<6 | (s[3] as u8),
        ]);
    }
    let rl = rem.len();
    if rl == 2 {
        // In any case, the index is <64, so even if usize is 16-bit, this cast won't truncate
        out.push(
            (rem[0] as u8)<<2 | (rem[1] as u8) >>4,
        );
    } else if rl == 3 {
        out.extend::<&[u8]>(&[
            (rem[0] as u8)<<2 | (rem[1] as u8) >>4,
            (rem[1] as u8)<<4 | (rem[2] as u8) >>2,
        ]);
    } else if rl == 1{
        return Err("Base64 remainder is 1 byte. It must be 0, 2 or 3 bytes!")
    } // else, rl is 0, we don't care
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    mod encode {
        use super::*;
        
        #[test]
        fn exact() {
            assert_eq!(base64_encode(b"Many hands make light work."), b"TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu");
        }
        #[test]
        fn padding_1() {
            assert_eq!(base64_encode(b"light wo"), b"bGlnaHQgd28=");
        }
        #[test]
        fn padding_2() {
            assert_eq!(base64_encode(b"light w"), b"bGlnaHQgdw==");
        }
    }
    mod decode {
        use super::*;
        
        #[test]
        fn exact() {
            assert_eq!(base64_decode(b"TWFueSBoYW5kcyBtYWtlIGxpZ2h0IHdvcmsu").unwrap(), b"Many hands make light work.");
        }
        #[test]
        fn spurious_characters_exact() {
            assert_eq!(base64_decode(b"TWFueSBoYW5kc yB tYWtlIG *** -- = \n xpZ2h0IHdvcmsu").unwrap(), b"Many hands make light work.");
        }
        #[test]
        fn padding_1() {
            assert_eq!(base64_decode(b"bGlnaHQgd28=").unwrap(), b"light wo");
        }
        #[test]
        fn padding_2() {
            assert_eq!(base64_decode(b"bGlnaHQgdw==").unwrap(), b"light w");
        }
        #[test]
        fn no_padding_1() {
            assert_eq!(base64_decode(b"bGlnaHQgd28").unwrap(), b"light wo");
        }
        #[test]
        fn no_padding_2() {
            assert_eq!(base64_decode(b"bGlnaHQgdw").unwrap(), b"light w");
        }
        #[test]
        #[should_panic]
        fn error() {
            base64_decode(b"bGlnaHQgd").unwrap();
        }
    }
}
