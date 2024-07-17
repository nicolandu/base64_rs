const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
fn encode(val: &[u8]) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(val.len()*4usize.div_ceil(3));
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

fn main() {
    println!("{}", String::from_utf8(encode(b"light work")).unwrap());
}
