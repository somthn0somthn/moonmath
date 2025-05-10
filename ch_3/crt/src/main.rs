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

    for i in 0..factors.len() {
        let a_i = factors[i];
        let m_i = moduli[i];

        let n_i = big_n / m_i;
        let (_, s, _) = xgcd(n_i, m_i);

        x_prime = x_prime + (a_i * n_i * s)
    }

    Ok((x_prime.rem_euclid(big_n), big_n))
}

fn main() {
    let val = crt(&[4, 1, 3, 0], &[7, 3, 5, 11]);
    println!("val {:?}", val);
}
