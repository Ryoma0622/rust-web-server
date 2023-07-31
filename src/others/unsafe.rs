pub fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let mut r2 = &mut num as *mut i32;

    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);

    r2 = ((r1 as usize) + 0x1) as *mut i32;

    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);

    unsafe {
        // *r2 = 10;
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
