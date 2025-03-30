#![no_std]
#![no_main]

use user_lib::{get_time, info, println, yield_, TaskInfo};


#[no_mangle]
fn main() -> i32 {

    let current_time = get_time();
    assert!(current_time > 0);
    println!("get_time OK! {}", current_time);
    let wait_for = current_time + 1000;
    while get_time() < wait_for {
        yield_();
    }
    
    let mut task_info = TaskInfo::init();
    for i in 0..6 {
        let ret = info(i, &mut task_info);
        if ret < 0 {
            println!("Error: {}", ret);
            return 1;
        }
        task_info.display();
    }

    // let mut task_info = TaskInfo::init();
    // info(1, &mut task_info);
    // task_info.display();
    println!("info OK!");
    let pc: usize;
    unsafe {
        core::arch::asm!(
            "auipc {0}, 0",  // 将 PC 的高 20 位写入寄存器
            "addi {0}, {0}, 0",  // 加上低 12 位偏移（此处为 0）
            out(reg) pc,
            options(nostack, nomem)
        );
    }
    println!("PC: {:#x}", pc);
    // exit(0);
    0
}
