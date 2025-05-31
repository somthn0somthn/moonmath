use sha2::{Digest, Sha256};
use log2::*;

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

// part 2

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
    // let res: [u8; 4] = <[u8; 4]>::try_from(&dig[0..4]).unwrap();
    // let res: [u8; 4] = dig[0..4].try_into().unwrap();
    let [a, b, c, d, ..]: [u8; 32] = dig.into();

    [a, b, c, d]
}

fn hash_to_bits(input: &str) -> Vec<u8> {
    let dig = Sha256::digest(input);
    let first_b = dig[0].try_into().unwrap();

    byte_to_bits(first_b)
}

fn hash_with_counter(base: &str, counter: u32) -> Vec<u8> {
    let input = format!("{}{}", base, counter);
    let res: [u8; 32] = Sha256::digest(&input).into();

    res.into()
}

fn bits_to_decimal(bits: &[u8]) -> u64 {
    let mut res = 0;
    for i in bits {
        res = (res << 1) + i;
    }

    res as u64
}

fn weighted_sum(bytes: &[u8]) -> u128 {
    let mut res: u128 = 0;

    for (i, b) in bytes.iter().enumerate() {
        res += (*b as u128) * 2u128.pow(i as u32);
    }

    res as u128
}

fn fits_in_k_bits(num: u128, k: u32) -> bool {
    num < 2u128.pow(k)
}

fn create_power_pairs(value: Vec<u8>) -> Vec<(usize, u128)> {
    value.iter().enumerate().map(|(i, x)| {
        let n: u128 = (*x as u128) * 2u128.pow(i as u32);
        (i, n)
    })
    .collect()
}

fn find_first_match(data: Vec<u8>, target: u8) -> Option<usize> {
    data.iter().position(|&x| x == target)
}

fn process_bytes(bytes: Vec<u8>) -> Vec<u16> {
    //I changed this because filter will have no effet if you multiply by two b/c that makes all 
    //values even
    bytes.iter().map(|x| (*x as u16) * 3).filter(|&x| x % 2 == 0).collect()
}

fn  find_pattern_indices(input: &str, pattern: u8) -> Vec<usize> {
    let hash = Sha256::digest(input);
    println!("hash >> {:?}", hash[0..8].to_vec());
    // its not necessary to convert the bytes to their binary representation
    hash[0..8].iter().enumerate().filter(|(_, b)| **b == pattern).map(|(i, b)| i).collect()
}

fn safe_get_bit(num: u8, position: u8) -> Result<u8, &'static str> {
    if position >= 8 { return Err("bit position too large")};
    Ok((num >> position) & 1)
}

fn safe_power_of_2(exp: u32) -> Option<u128> {
    2u128.checked_pow(exp)
}

fn safe_weighted_sum(bytes: &[u8]) -> Result<u128, &'static str> {
    let mut res: u128 = 0;

    for (i,  b) in bytes.iter().enumerate() {
        let powr = 2u128.checked_pow(i as u32).ok_or("exponentiation failed")?;
        let pdt = (*b as u128).checked_mul(powr).ok_or("multiplication failed")?;
        res = res.checked_add(pdt).ok_or("addition failed")?;
    }

    Ok(res)
}

fn safe_hash_prefix(input: &str, prefix_len: usize) -> Result<Vec<u8>, &'static str> {
    if prefix_len <= 0 || prefix_len >= 32 {return Err("bad prefix_len")};
    if input == "" {return Err("bad input")};
    let res = Sha256::digest(input);
    Ok(res[0..prefix_len].into())
}

fn main() {
    let res = safe_hash_prefix("hello", 4);

    println!("{:?}", res);
}
