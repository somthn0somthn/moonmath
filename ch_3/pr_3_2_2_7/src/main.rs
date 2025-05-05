// this returns an array matching the powers of two, or the "binary index" that 
// are populated by a one. zero values are dropped. The hope is that the logic
// matches the binary representation equation in the book
pub fn bin_decomp(num: u32) -> Vec<u32> {
    if num == 0 { return vec![]};
    if num == 1 { return vec![1]};

    let res:u32 = (num as f64).log2().floor() as u32;
    //println!("bin_decomp res is {}", res);

    let remainder = num as u32 % (2u32.pow(res));
    //println!("bin_decomp remainder is {}", remainder);

    let mut rec_vec = bin_decomp(remainder);
    //println!("bin_decomp rec_vec is {:?}", rec_vec);

    rec_vec.insert(0, res + 1);
    rec_vec
    
}

// TODO: create a helper function that turns above array into a string of 1s & 0s
pub fn arr_to_str() {}

pub fn main(){

    for i in 0..17 {
        let res =  bin_decomp(i);
        println!("num {} == res {:?}", i, res);
    }
    
} 