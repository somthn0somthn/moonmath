use sha2::{Sha256, Digest};

fn get_bit(num: u8, position: u8) -> u8 {
    if position > 7 {
        return 0;
    };

    (num >> position) & 1
}

fn set_bit(num: u8, position: u8) -> u8 {
    let pos = 2u8.pow(position as u32);

    num | pos
}

fn byte_to_bits(byte: u8) -> Vec<u8> {
    let mut res = Vec::new();

    for i in 0..=7 {
        res.push((byte >> i) & 1);
    }
    res.into_iter().rev().collect()
}

//part 2

fn count_pattern(data: &str) -> usize {
    let top_bits: u8 = 0b00001010;
    let mut c: usize = 0;

    for byte in data.as_bytes() {
        if (byte >> 4) == top_bits {
            c += 1
        };
    }

    c
}

fn string_to_all_bits(s: &str) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    for byte in s.as_bytes().into_iter().rev() {
        for i in 0..8 {
            v.push((byte >> i) & 1);
        }
    }

    v.into_iter().rev().collect()
}

fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    // /let mut bytes = Vec::new();
    let chunks = bits.chunks(8);

    chunks
        .map(|byte| {
            let mut n = 0b0000_0000;
            for i in 0..8 {
                let bit = byte[i] & 1;
                println!(">> BIT {i} is {bit}");
                n = n | (bit << (7 - i));
                println!(">> n is {n:08b}");
            }
            n
        })
        .collect()
}

fn hash_prefix(input: &str) -> [u8; 4] {
    let dig = Sha256::digest(input);
    //let res: [u8; 4] = <[u8; 4]>::try_from(&dig[0..4]).unwrap();
    //let res: [u8; 4] = dig[0..4].try_into().unwrap();
    let [a, b, c, d, ..]: [u8; 32] = dig.into();

    [a, b, c, d]
}

fn hash_to_bits(input: &str) -> Vec<u8> {
    let dig = Sha256::digest(input);
    let first_b = dig[0].try_into().unwrap();

    byte_to_bits(first_b)
}

fn hash_with_counter(base: &str, counter: u32)  -> Vec<u8> {
    let input = format!("{}{}", base, counter);
    let res: [u8; 32] = Sha256::digest(&input).into();

    res.into()
}

//TODO: cont from 4. Number Base Conversions & Large Numbers
// https://claude.ai/chat/ecff9d10-be5c-4508-b8ae-5232326ec5be

fn main() {
    let base = "password";
    let res = hash_with_counter(base, 1);

    println!("{:?}", res);

}
