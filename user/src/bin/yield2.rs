#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{info, yield_, TaskInfo};

/*
理想结果：三个程序交替输出 ABC
*/

const WIDTH: usize = 10;
const HEIGHT: usize = 5;

#[no_mangle]
fn main() -> i32 {
    // let mut task_info = TaskInfo::init();
    // info(1, &mut task_info);
    // task_info.display();

    for i in 0..HEIGHT {
        let buf = ['C' as u8; WIDTH];
        println!(
            "{} [{}/{}]",
            core::str::from_utf8(&buf).unwrap(),
            i + 1,
            HEIGHT
        );
        yield_();
    }
    println!("Test write C OK!");

    // let mut task_info = TaskInfo::init();
    // info(1, &mut task_info);
    // task_info.display();
    0
}
