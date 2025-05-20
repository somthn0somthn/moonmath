fn double_add(g: u32, x: u32) -> Result<u32, ()> {
    if x == 0 {
        return Ok(g);
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

fn main() {
    let x = 100;
    let g = 15;

    let value = double_add(g, x);

    println!("value is {:?}", value);

}