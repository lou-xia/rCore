#![no_std]
#![no_main]

use user_lib::{get_time, info, println, yield_, TaskInfo};


#[no_mangle]
fn main() -> i32 {

    let current_time = get_time();
    assert!(current_time > 0);
    println!("get_time OK! {}", current_time);
    let wait_for = current_time + 5000;
    while get_time() < wait_for {
        if (get_time() - current_time) % 1000 == 0 {
            println!("Waiting for {} us", wait_for - get_time());
        }
        yield_();
    }
    
    let mut task_info = TaskInfo::init();
    for i in 0..7 {
        let ret = info(i, &mut task_info);
        if ret < 0 {
            println!("Error: {}", ret);
            return 1;
        }
        task_info.display();
    }
    0
}
