//! Types related to task management

use crate::syscall::*;
use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub is_sched: bool,            // 是否已经被调度过
    pub first_sched_time: TimeVal, // 第一次被调度的时刻
    pub syscall_counts: [u32; MAX_SYSCALL_NUM],
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}
