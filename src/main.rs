
#[no_mangle] //Rust’s no_mangle macro tells the compiler not to change the signature of the function during compilation
pub extern "C" fn add_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world! :{}",add_one(6));

    //if then else
    let n:i32=19;
    if n>18{
        println!("above 18");
    }else{
        println!("Below 18");
    }
}

// fn gcd(mut n: u64, mut m: u64) -> u64 {
//     assert!(n != 0 && m != 0);//  assert! macro, verifying that neither argument is zero
//     while m != 0 {
//         if m < n {
//             let t = m;
//             m = n;
//             n = t;
//         }
//             m = m % n;
//     }
//     n
// }

// #[test] //#[test] atop the definition marks test_gcd as a test function, to be skipped in normal compilations
// fn test_gcd() {
//     assert_eq!(gcd(14, 15), 1);
//     assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
//     3 * 7 * 11 * 13 * 19),
//     3 * 11);
// }
//to run the test function run
//cargo test