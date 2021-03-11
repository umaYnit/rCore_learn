#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    // 未初始化
    UnInit,
    // 准备运行
    Ready,
    // 正在运行
    Running,
    // 已退出
    Exited,
}

pub struct TaskControlBlock {
    pub task_cx_ptr: usize,
    pub task_status: TaskStatus,
}

impl TaskControlBlock {
    pub fn get_task_cx_ptr2(&self) -> *const usize {
        &self.task_cx_ptr as *const usize
    }
}