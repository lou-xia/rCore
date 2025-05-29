#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{close, fstat, linkat, open, read, unlinkat, write, OpenFlags, Stat, AT_FDCWD};

#[no_mangle]
pub fn main() -> i32 {
    let test_str: &'static str = "Hello, world!";
    let filea = "filea\0";
    let fileb = "fileb\0";
    let fd = open(filea, OpenFlags::CREATE | OpenFlags::WRONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    write(fd, test_str.as_bytes());
    close(fd);

    // link filea to fileb
    let link_res = linkat(AT_FDCWD, filea, AT_FDCWD, fileb, 0);
    assert!(link_res == 0);

    // read fileb and check content
    let fd = open(fileb, OpenFlags::RDONLY);
    assert!(fd > 0);
    let fd = fd as usize;
    let mut buffer = [0u8; 100];
    let read_len = read(fd, &mut buffer) as usize;
    // read fileb stat
    let mut stat = Stat::default();
    let _fd_stat = fstat(fd as i32, &mut stat);
    println!("fd_stat: {:?}", stat);
    close(fd);

    // unlink fileb
    let unlink_res = unlinkat(AT_FDCWD, fileb, 0);
    assert!(unlink_res == 0);

    // read fileb again, should fail
    let fd = open(fileb, OpenFlags::RDONLY);
    assert!(fd < 0);

    let fd = open(filea, OpenFlags::RDONLY);
    let mut stat = Stat::default();
    let _fd_stat = fstat(fd as i32, &mut stat);
    println!("fd_stat2: {:?}", stat);
    close(fd as usize);

    let fd = open(fileb, OpenFlags::RDONLY);
    println!("now open fileb, result = {}", fd);

    for i in 0..read_len {
        print!("{} ", buffer[i] as char);
    }
    println!("");

    assert_eq!(test_str, core::str::from_utf8(&buffer[..read_len]).unwrap(),);
    println!("file_test passed!");
    0
}
