pub fn long_div(dividend: u64, divisor: u64) -> Result<(u64, u64), ()> {
    //only dealing with positive numbers
    if divisor == 0 {return Err(())};
    if dividend == 0 {return Ok((0,0))}

    let mut dividend_holder: u64 = 0;
    let mut minus_counter: u64 = 1;
    let mut result_array: Vec<u64> = vec![];

    let mut dividend_array = num_to_array(dividend);
    
    for val in dividend_array {
        dividend_holder = consolidate_nums(dividend_holder, val);

        if dividend_holder >= divisor {
            while dividend_holder >= (divisor * minus_counter) {
                minus_counter += 1;
            }
            let new_holder = dividend_holder;
            dividend_holder = new_holder - (divisor * (minus_counter - 1));
            result_array.push(minus_counter - 1);
            minus_counter = 0;
        } else if !result_array.is_empty() {
            result_array.push(0);
        }
    }

    let result = array_to_num(result_array);
    let rem = dividend - (divisor * result);

    Ok((result, rem))
}

pub fn consolidate_nums(div: u64, num: u64) -> u64 {
    (div * 10) + num
}

pub fn num_to_array(num: u64) -> Vec<u64> {
    if num == 0 {
        return vec![0];
    };

    let mut coll = Vec::new();
    let mut m_num: u64 = num.abs_diff(0);

    while m_num > 0 {
        let val = m_num % 10;
        coll.push(val);
        m_num -= val;
        m_num /= 10;
    }

    coll.reverse();
    coll
}

pub fn array_to_num(arr: Vec<u64>) -> u64 {
    let mut holder: u64 = 0;
    for val in arr {
        holder = (holder * 10) + val
    }

    holder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_0() {
        let res = long_div(143785, 17);
        assert_eq!(Ok((8457, 16)), res);
    }

    #[test]
    fn t_1() {
        let res = long_div(27, 5); 

        assert_eq!(Ok((5, 2)), res);
    }
    
    #[test]
    fn t_2() {
        let res = long_div(18_446_744_073_709_551_615, 986_327_532_987);

        assert_eq!(Ok((18702452, 731741767491)), res);
    }
    
    #[test]
    fn t_3() {
        let res = long_div(3006, 5);
        assert_eq!(Ok((601, 1)), res);

    
    }

    #[test]
    fn t_dividend_is_zero() {
        let res = long_div(0, 7);

        assert_eq!(Ok((0,0)), res)

    }

    #[test]
    fn t_divisor_is_zero() {
        let res = long_div(137, 0);
        assert_eq!(Err(()), res);
    }

    #[test]
    fn t_divisor_is_one() {
        let res = long_div(137, 1);
        assert_eq!(Ok((137, 0)), res);
    }


    #[test]
    fn t_dividend_is_multiple_of_divisor() {
        let res = long_div(221, 17);

        assert_eq!(Ok((13, 0)), res);
    }

    #[test]
    fn t_dividend_is_lt_divisor() {
        let res = long_div(27, 153);

        assert_eq!(Ok((0, 27)), res)
    }
}

