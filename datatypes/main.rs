use std::mem;




fn main() {
    let a:u8 = 123;
    println!("{}",a);

    // mutable values
    let mut b:i8 = 100;
    println!("b = {}", b);

    b = 40;
    println!("b = {}", b);


    let mut c = 123456789; // 32 bit signed i32
    println!("c = {}, size = {}", c, mem::size_of_val(&c));

    c = -987654321;
    println!("c = {}, size = {}", c, mem::size_of_val(&c));

    // i8 u8 i16 u16 i64 u64
    let z:isize = 123;  // isize / usize
    let size_of_z = mem::size_of_val(&z);
    println!("z = {}, takes {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);
    
    let d = 'x';
    println!("d = {}, takes {} bytes", d, mem::size_of_val(&d));

    let e = 2.5;    // double precision, 8bytes or 64bits, f64
    println!("e = {}, takes {} bytes", e, mem::size_of_val(&e));

    let g = false;
    println!("g = {}, takes {} bytes", g, mem::size_of_val(&g));

    let f = 4>0; // true;
}