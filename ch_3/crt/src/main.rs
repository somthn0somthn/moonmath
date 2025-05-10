pub fn xgcd(a: i32, b: i32) -> (i32, i32, i32) {
    if b > a {
        let (gcd, t, s) = xgcd(b, a);
        return (gcd, s, t);
    } else if b == 0 {
        return (a, 1, 0);
    }

    let mut r0: i32 = a;
    let mut r1: i32 = b;
    let mut s0: i32 = 1;
    let mut s1: i32 = 0;
    let mut t0: i32 = 0;
    let mut t1: i32 = 1;

    while r1 != 0 {
        let qk = r0 / r1;
        let rk = r0 % r1;
        let sk = s0 - (qk * s1);
        let tk = t0 - (qk * t1);

        r0 = r1;
        r1 = rk;

        s0 = s1;
        s1 = sk;

        t0 = t1;
        t1 = tk;
    }

    if r0 < 0 {
        return (-r0, -s0, -t0)
    }
    (r0, s0, t0)
}


fn main() {
    println!("55 & 34: {:?}\n", xgcd(55, 34));
    println!("17 & 13: {:?}\n", xgcd(17, 13));
    println!("12 & 8: {:?}\n", xgcd(12, 8));
    println!("25 & 25: {:?}\n", xgcd(25, 25));
    println!("42 & -15: {:?}\n", xgcd(42, -15));

    println!("12 & 8: {:?}\n", xgcd(12,8));
    println!("17 & 13: {:?}\n", xgcd(17,13));
    println!("0 & 5: {:?}\n", xgcd(0,5));
}