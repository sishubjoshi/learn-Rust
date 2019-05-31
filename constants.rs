
const GOOD_NAME:u8 = 42;    // no fixed address

static mut Z:i32 = 123;

fn main() {

    println!("{}", GOOD_NAME);

    unsafe {                // own risk
        println!("{}", Z);
    }
}