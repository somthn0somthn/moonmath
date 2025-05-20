#[allow(dead_code)]
fn sum_mul(g: u32, x: u32) -> Result<u32, ()> {
    if g == 0 && x == 0 {
        return Err(());
    };

    let mut y: u32 = 1;
    let mut g_current = g;
    let bit_len = u32::BITS - x.leading_zeros();

    for n in 0..bit_len {
        let bit = (x >> n) & 1;
        if bit == 1 {
            y = y.checked_mul(g_current).ok_or(())?
        };

        if n < bit_len - 1 {
            g_current = g_current.checked_mul(g_current).ok_or(())?;
        }
    }

    Ok(y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zeroes() {
        let res = sum_mul(0, 0);
        assert!(res.is_err());
    }

    #[test]
    fn zero_base() {
        let res = sum_mul(0, 5);
        assert_eq!(res, Ok(0));
    }

    #[test]
    fn zero_power() {
        let res = sum_mul(12, 0);
        assert_eq!(res, Ok(1));
    }

    #[test]
    fn two_to_twelve() {
        let res = sum_mul(2, 12);
        assert_eq!(res, Ok(4096));
    }

    #[test]
    fn five_to_six() {
        let res = sum_mul(5, 6);
        assert_eq!(res, Ok(15625));
    }

    #[test]
    fn twenty_to_six() {
        let res = sum_mul(20, 6);
        assert_eq!(res, Ok(64000000));
    }
}
