// part 1

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

// TODO - continue section 3 https://claude.ai/chat/ecff9d10-be5c-4508-b8ae-5232326ec5be

fn main() {
    // let text = "Hello";
    let hello_bits: [u8; 40] = [
        0, 1, 0, 0, 1, 0, 0, 0, 
        0, 1, 1, 0, 0, 1, 0, 1, 
        0, 1, 1, 0, 1, 1, 0, 0, 
        0, 1, 1, 0, 1, 1, 0, 0, 
        0, 1, 1, 0, 1, 1, 1, 1, 
    ];

    let res = bits_to_bytes(&hello_bits);

    println!("{:?}", res);
}
