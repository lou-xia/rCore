//! File and filesystem-related syscalls
use crate::config::AT_FDCWD;
use crate::fs::{open_file, OpenFlags};
use crate::mm::{translated_byte_buffer, translated_refmut, translated_str, UserBuffer};
use crate::task::{current_task, current_user_token};

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        if !file.writable() {
            return -1;
        }
        let file = file.clone();
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.write(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_read(fd: usize, buf: *const u8, len: usize) -> isize {
    let token = current_user_token();
    let task = current_task().unwrap();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        if !file.readable() {
            return -1;
        }
        // release current task TCB manually to avoid multi-borrow
        drop(inner);
        file.read(UserBuffer::new(translated_byte_buffer(token, buf, len))) as isize
    } else {
        -1
    }
}

pub fn sys_open(path: *const u8, flags: u32) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let path = translated_str(token, path);
    if let Some(inode) = open_file(path.as_str(), OpenFlags::from_bits(flags).unwrap()) {
        let mut inner = task.inner_exclusive_access();
        let fd = inner.alloc_fd();
        inner.fd_table[fd] = Some(inode);
        fd as isize
    } else {
        -1
    }
}

pub fn sys_close(fd: usize) -> isize {
    let task = current_task().unwrap();
    let mut inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if inner.fd_table[fd].is_none() {
        return -1;
    }
    inner.fd_table[fd].take();
    0
}

pub fn sys_linkat(old_dirfd: i32, old_path: *const u8, new_dirfd: i32, new_path: *const u8, flags: u32) -> isize {
    assert!(old_dirfd == AT_FDCWD, "old_dirfd must be AT_FDCWD");
    assert!(new_dirfd == AT_FDCWD, "new_dirfd must be AT_FDCWD");
    assert!(flags == 0, "flags must be 0");
    let token = current_user_token();
    let old_file_name = translated_str(token, old_path);
    let new_file_name = translated_str(token, new_path);
    println!("[SYSCALL] linkat: {} -> {}", old_file_name, new_file_name);
    if let Some(inode) = open_file(old_file_name.as_str(), OpenFlags::RDWR) {
        inode.linkat(&new_file_name)
    } else {
        -1
    }
}

pub fn sys_unlinkat(dirfd: i32, path: *const u8, flags: u32) -> isize {
    assert!(dirfd == AT_FDCWD, "old_dirfd must be AT_FDCWD");
    assert!(flags == 0, "flags must be 0");
    let token = current_user_token();
    let file_name = translated_str(token, path);
    println!("[SYSCALL] unlinkat: {}", file_name);
    if let Some(inode) = open_file(file_name.as_str(), OpenFlags::RDWR) {
        inode.unlinkat(&file_name)
    } else {
        -1
    }
}

pub fn sys_fstat(fd: usize, st: *mut Stat) -> isize {
    let task = current_task().unwrap();
    let token = current_user_token();
    let inner = task.inner_exclusive_access();
    if fd >= inner.fd_table.len() {
        return -1;
    }
    if let Some(file) = &inner.fd_table[fd] {
        let file = file.clone();
        let stat = file.get_stat();
        let st = translated_refmut(token, st);
        *st = stat;
        0
    } else {
        -1
    }
}

/// Stat 结构体定义
#[repr(C)]
#[derive(Debug)]
pub struct Stat {
    /// 文件所在磁盘驱动器号，该实验中写死为 0 即可
    pub dev: u64,
    /// inode 文件所在 inode 编号
    pub ino: u64,
    /// 文件类型
    pub mode: StatMode,
    /// 硬链接数量，初始为1
    pub nlink: u32,
    /// 无需考虑，为了兼容性设计
    pub pad: [u64; 7],
}

bitflags! {
    /// StatMode 定义：
    pub struct StatMode: u32 {
        /// no such file or directory
        const NULL  = 0;
        /// directory
        const DIR   = 0o040000;
        /// ordinary regular file
        const FILE  = 0o100000;
    }
}

impl Stat {
    /// 默认 Stat 结构体
    pub fn default() -> Self {
        Stat {
            dev: 0,
            ino: 0,
            mode: StatMode::NULL,
            nlink: 1,
            pad: [0; 7],
        }
    }
}