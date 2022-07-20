//! Process management syscalls

use crate::config::{MAX_APP_NUM, MAX_SYSCALL_NUM};
use crate::task::*;
use crate::timer::get_time_us;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

pub struct TaskInfo {
    status: TaskStatus,
    syscall_times: [u32; MAX_SYSCALL_NUM],
    time: usize,
}

impl From<TimeVal> for usize {
    fn from(tv: TimeVal) -> Self {
        tv.sec * 1_000_000 + tv.usec
    }
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    let first_sched_ts = get_first_sched_time();
    let syscall_times = list_syscall_counts();
    let curr_ts = get_time_us();
    unsafe {
        *ti = TaskInfo {
            status: TaskStatus::Running,
            syscall_times: syscall_times,
            time: (usize::from(curr_ts) - usize::from(first_sched_ts)) / 1000,
        }
    }
    0
}
