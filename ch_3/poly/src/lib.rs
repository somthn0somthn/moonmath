// Warning: This practice implementation ignores integer over and underflows
pub fn xgcd(a: i32, b: i32) -> (i32, i32, i32) {
    if a == 0 && b == 0 {
        return (0, 0, 0);
    } else if b == 0 {
        return (a, 1, 0);
    } else if b > a {
        let (g, t, s) = xgcd(b, a);
        return (g, s, t);
    }

    let (mut r0, mut r1) = (a, b);
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);

    while r1 != 0 {
        let (qk, rk) = div_rem(r0, r1);
        let sk = s0 - qk * s1;
        let tk = t0 - qk * t1;
        (r0, r1) = (r1, rk);
        (s0, s1) = (s1, sk);
        (t0, t1) = (t1, tk);
    }

    if r0 < 0 {
        (-r0, -s0, -t0)
    } else {
        (r0, s0, t0)
    }
}

fn div_rem(a: i32, b: i32) -> (i32, i32) {
    let rem = a.rem_euclid(b);
    let quot = (a - rem) / b;
    (quot, rem)
}

pub struct Ring {
    pub modulus: Option<i32>,
}

impl Ring {
    pub fn inverse_of(&self, x: i32) -> Option<i32> {
        match self.modulus {
            Some(m) => {
                let (g, _s, t) = xgcd(x, m);
                if g != 1 { None } else { Some(t.rem_euclid(m)) }
            }
            None => {
                if x == 1 {
                    Some(1)
                } else if x == -1 {
                    Some(-1)
                } else {
                    None
                }
            }
        }
    }
}

pub fn poly_eucl(a: &[i32], b: &[i32], r: Ring) -> Result<(Vec<i32>, Vec<i32>), ()> {
    if b.iter().all(|&x| x == 0) {
        return Err(());
    }

    let mut p: Vec<i32> = a.iter().copied().rev().collect();
    let b_rev: Vec<i32> = b.iter().copied().rev().collect();

    let mut divisor_len = b_rev.len();
    while divisor_len > 0 && b_rev[divisor_len - 1] == 0 {
        divisor_len -= 1;
    }
    if divisor_len == 0 {
        return Err(());
    }
    let lead_coef = b_rev[divisor_len - 1];
    let inv_lead = r.inverse_of(lead_coef).ok_or(())?;

    let mut q: Vec<i32> = Vec::new();

    while p.len() >= divisor_len {
        let deg_p = p.len() - 1;
        let deg_diff = deg_p + 1 - divisor_len;

        let coef = p[deg_p] * inv_lead;
        if q.len() < deg_diff + 1 {
            q.resize(deg_diff + 1, 0);
        }
        q[deg_diff] = coef;

        for i in 0..divisor_len {
            let idx = i + deg_diff;
            p[idx] -= b_rev[i] * coef;
        }

        while p.last() == Some(&0) {
            p.pop();
        }
    }

    p.reverse();
    q.reverse();
    Ok((q, p))
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
    fn pol01() {
        let a = &[1, 0, 0, -1];
        let b = &[1, -1];
        let r = Ring { modulus: None };

        let res = Ok((vec![1, 1, 1], vec![]));
        assert_eq!(res, poly_eucl(a, b, r));
    }
}
