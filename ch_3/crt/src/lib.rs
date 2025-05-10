// Warning: This practice implementation ignores integer over and underflows
pub fn xgcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 && b == 0 {
        return (0, 0, 0);
    } else if b == 0 {
        return (a, 1, 0);
    } else if b > a {
        let (gcd, t, s) = xgcd(b, a);
        return (gcd, s, t);
    }

    let mut r0: i32 = a;
    let mut r1: i32 = b;
    let mut s0: i32 = 1;
    let mut s1: i32 = 0;
    let mut t0: i32 = 0;
    let mut t1: i32 = 1;

    while r1 != 0 {
        let (qk, rk) = div_rem(r0, r1);
        let sk = s0 - (qk * s1);
        let tk: i32 = t0 - (qk * t1);

        r0 = r1;
        r1 = rk;

        s0 = s1;
        s1 = sk;

        t0 = t1;
        t1 = tk;
    }

    if r0 < 0 {
        return (-r0, -s0, -t0);
    }
    (r0, s0, t0)
}

// Note: this is used to avoid native behavious of (%) and (/) that are not ideal for handling
// negative integers for the crt
fn div_rem(a: i32, b: i32) -> (i32, i32) {
    let rem = a.rem_euclid(b);
    let quot = (a - rem) / b;
    (quot, rem)
}

fn are_coprime(mods: &[i32]) -> bool {
    for i in 0..mods.len() {
        for j in i + 1..mods.len() {
            let (gcd, _, _) = xgcd(mods[i], mods[j]);
            if gcd != 1 {
                return false;
            }
        }
    }
    true
}

pub fn crt(factors: &[i32], moduli: &[i32]) -> Result<(i32, i32), ()> {
    if factors.len() != moduli.len() {
        return Err(());
    }

    if !are_coprime(moduli) {
        return Err(());
    };

    let big_n: i32 = moduli.into_iter().product();

    let mut x_prime: i32 = 0;

    for j in 0..factors.len() {
        let a_i = factors[j];
        let n = moduli[j];

        let nj = big_n / n;
        let (_, s, _) = xgcd(nj, n);

        x_prime = x_prime + (a_i * nj * s)
    }

    Ok((x_prime.rem_euclid(big_n), big_n))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xgcd_rand0() {
        let res = xgcd(900803152, 439484900);
        assert_eq!(res, (4, 26692002, -54710047));
    }

    #[test]
    fn xgcd_rand1() {
        let res = xgcd(-767898930, -314596854);
        assert_eq!(res, (6, -11315396, 27619731))
    }

    #[test]
    fn xgcd_rand2() {
        let res = xgcd(-1871482624, 965392962);
        assert_eq!(res, (2, -90542000, -175522079));
    }

    #[test]
    fn xgcd_rand3() {
        let res = xgcd(90513, -28788);
        assert_eq!(res, (3, -4177, -13133));
    }

    #[test]
    fn xgcd_a_zero() {
        let res = xgcd(0, -812634);
        assert_eq!(res, (812634, 0, -1));
    }

    #[test]
    fn xgcd_b_zero() {
        let res = xgcd(423517, 0);
        assert_eq!(res, (423517, 1, 0));
    }

    #[test]
    fn xgcd_both_zero() {
        let res = xgcd(0, 0);
        assert_eq!(res, (0, 0, 0));
    }

    #[test]
    fn xgcd_equal() {
        let res = xgcd(17, 17);
        assert_eq!(res, (17, 0, 1));
    }

    #[test]
    fn xgcd_common_factor() {
        let res = xgcd(12, 8);
        assert_eq!(res, (4, 1, -1));
    }

    #[test]
    fn xgcd_coprime() {
        let res = xgcd(17, 13);
        assert_eq!(res, (1, -3, 4));
    }

    #[test]
    fn crt0() {
        let factors = &[2, 3, 3];
        let moduli = &[3, 5, 7];

        assert_eq!(crt(factors, moduli), Ok((38, 105)));
    }

    #[test]
    fn crt1() {
        let factors = &[4, 1, 3, 0];
        let moduli = &[7, 3, 5, 11];

        assert_eq!(crt(factors, moduli), Ok((88, 1155)));
    }

    #[test]
    fn crt_with_zeroes() {
        let factors = &[0, 0, 0];
        let moduli = &[2, 3, 5];

        assert_eq!(crt(factors, moduli), Ok((0, 30)));
    }
}
