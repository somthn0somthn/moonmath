pub fn num_to_bin_str(n: u32) -> String {
    if n == 0 {
        return "0".to_string();
    }

    let mut temp = n;
    let mut k = 0;
    while temp > 0 {
        temp /= 2;
        k += 1;
    }

    let mut res = String::with_capacity(k);

    for j in (0..k).rev() {
        let power = 2u32.pow(j as u32);
        if n / power % 2 == 1 {
            res.push('1');
        } else {
            res.push('0')
        }
    }

    res
}

#[test]
pub fn t_zero() {
    let zero = "0".to_string();
    assert_eq!(zero, num_to_bin_str(0));
}

#[test]
pub fn t_one() {
    let one = "1".to_string();
    assert_eq!(one, num_to_bin_str(1));
}

#[test]
pub fn t_four() {
    let four = "100".to_string();
    assert_eq!(four, num_to_bin_str(4));
} 

#[test]
pub fn t_nine() {
    let nine = "1001".to_string();
    assert_eq!(nine, num_to_bin_str(9));
}

pub fn t_sixteen() {
    let sixteen = "10000".to_string();
    assert_eq!(sixteen, num_to_bin_str(16));
}