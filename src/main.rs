
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
