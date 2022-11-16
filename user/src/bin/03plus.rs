#![no_std]
#![no_main]

use core::arch::asm;

#[macro_use]
extern crate user_lib;

#[no_mangle]
fn main() -> i32 {
    let num1 = 10;
    let num2 = 2;
    let mut res:i32;

    res = num1 + num2;
    println!("Sum:{}",res);
            unsafe {        
                asm!("sret");
            }
    0
}

