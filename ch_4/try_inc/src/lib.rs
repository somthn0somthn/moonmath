use sha2::{Digest, Sha256};

fn try_inc(n: u32, s: String) -> u32 {
    let k = 32 - n.leading_zeros();
    let mut c = 0;

    loop {
        let c_bit_str: String = format!("{:b}", c);
        let mut s_pr = s.clone();
        s_pr.push_str(&c_bit_str);
        let z = Sha256::digest(&s_pr);
        let mut bit_str = "".to_string();

        for byte in z {
            let s = format!("{:08b}", byte);
            bit_str.push_str(&s);
        }

        let first_k_bits = &bit_str[..k as usize];

        let mut z_value = 0u32;
        for (i, bit_char) in first_k_bits.chars().enumerate() {
            let bit_digit = bit_char.to_digit(10).unwrap(); 
            z_value += bit_digit * (1 << i); 
        }

        if z_value < n {
            println!("C was >>> {c}");
            return z_value
        }
        c += 1
    }
}

