fn main() {

    let mut a = 2+3*4;
    println!("{}", a);
    a = a+1;    // doesnot support ++, --

    println!("remainder of {} / {} = {}", a, 2, (a%2));

    let a_cube = i32::pow(a,3);
    println!("{} cubed is {}",a, a_cube);

    let b = 2.5;
    let b_cube = f64::powi(b,3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);

    println!("{} cubed = {}, {}^pi = {}", b, b_cube, b, b_to_pi);

    // bitwise
    let c = 1 | 2;
    println!("1|2 = {}", c);

    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_4 = std::f64::consts::PI < 4.0;
    let x = 5;
    let x_is_5 = x == 5;
}