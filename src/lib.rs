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
    }
    out
}

pub fn base64_decode(val: &[u8]) -> Vec<u8> {
    assert!(val%4 != 1);
    let mut out: Vec<u8> = Vec::with_capacity((val.len()*3usize).div_floor(4));
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
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn no_padding() {
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
