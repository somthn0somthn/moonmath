#[allow(dead_code)]
//Note: Should I add a modulus?
fn double_add(g: u32, x: u32) -> Result<u32, ()> {
    if x == 0 || g == 0{
        return Ok(0);
    };

    let mut y: u32 = 0;
    let mut g_current = g;
    let bit_len = u32::BITS - x.leading_zeros();

    for n in 0..bit_len {
        let bit = (x >> n) & 1;
        if bit == 1 {
            y = y.checked_add(g_current).ok_or(())?;
        };

        if n < bit_len - 1 {
            g_current = g_current.checked_add(g_current).ok_or(())?;
        }
    }

    Ok(y)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroes() {
        let res = double_add(0, 0);
        assert_eq!(res, Ok(0));
    }

    #[test]
    fn zero_multiplicant() {
        let res = double_add(0, 5);
        assert_eq!(res, Ok(0));
    }

    #[test]
    fn zero_multiplier() {
        let res = double_add(12, 0);
        assert_eq!(res, Ok(0));
    }

    #[test]
    fn two_to_twelve() {
        let res = double_add(2, 12);
        assert_eq!(res, Ok(24));
    }

    #[test]
    fn five_to_six() {
        let res = double_add(5, 6);
        assert_eq!(res, Ok(30));
    }

    #[test]
    fn twenty_to_six() {
        let res = double_add(20, 6);
        assert_eq!(res, Ok(120));
    }
}

