#[derive(Debug,Copy,Clone,Eq,PartialEq)]
pub enum Taskstatus{
    Idle,
    Running,
    Waiting,
    Terminated,
    New,
}

#[derive(Copy,Clone)]
pub struct Task {
    pub id: u32,
    cpu_time: u64,
    cpu_burst_length: u64,
    io_burst_length: u64,
    weight: u32,
}