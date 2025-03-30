use crate::config::MAX_SYSCALL_NUM;

#[derive(Copy, Clone)]
pub struct TaskInfo {
    pub id: usize,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize,
    pub start_time: usize,
}

#[derive(Copy, Clone)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize
}

impl TaskInfo {
    pub fn init() -> Self {
        Self {
            id: 0,
            call: [SyscallInfo { id: 0, times: 0 }; MAX_SYSCALL_NUM],
            time: 0,
            start_time: 0,
        }
    }
    fn get_call_by_id(&mut self, id: usize) -> Option<&mut SyscallInfo> {
        for info in self.call.iter_mut() {
            if info.id == id {
                return Some(info)
            }
            if info.id == 0 {
                break
            }
        }
        None
    }
    pub fn add_syscall_by_id(&mut self, id: usize) {
        if let Some(info) = self.get_call_by_id(id) {
            info.times += 1;
        } else {
            let mut flag = false;
            for info in self.call.iter_mut() {
                if info.id == 0 {
                    info.id = id;
                    info.times = 1;
                    flag = true;
                    break;
                }
            }
            if!flag {
                self.call[MAX_SYSCALL_NUM - 1].id = id;
                self.call[MAX_SYSCALL_NUM - 1].times = 1;
            }
        }
    }
}
