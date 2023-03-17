#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Taskstatus {
    Idle,
    Running,
    Waiting,
    Terminated,
    New,
}

#[derive(Copy, Clone)]
pub struct Task {
    pub id: u32,
    cpu_time: u64,
    cpu_burst_length: u64,
    io_burst_length: u64,
    weight: u32,
}

impl TaskChara {
    pub fn new(
        id: u32,
        cpu_time: u64,
        cpu_burst_length: u64,
        io_burst_length: u64,
        weight: u32,
    ) -> Self {
        Self {
            id,
            cpu_time,
            cpu_burst_length,
            io_burst_length,
            weight,
        }
    }

    #[inline]
    pub fn get_id(&self) -> u32 {
        self.id
    }

    #[inline]
    pub fn get_cpu_time(&self) -> u64 {
        self.cpu_time
    }

    #[inline]
    pub fn get_cpu_burst_length(&self) -> u64 {
        self.cpu_burst_length
    }

    #[inline]
    pub fn get_io_burst_length(&self) -> u64 {
        self.io_burst_length
    }

    #[inline]
    pub fn get_weight(&self) -> u32 {
        self.weight
    }
}

#[derive(Debug)]
pub struct Task {
    id: u16,
    cpu_time: u64,
    cpu_burst_length: u64,
    io_burst_length: u64,
    state: Taskstatus,
    runtime: u64,
    vruntime: u64,
    idle_time: u64,
    start_time: u128,
    weight: u32,
}

impl Task {
    pub fn new(
        id: u16,
        cpu_time: u64,
        cpu_burst_length: u64,
        io_burst_length: u64,
        start_time: u128,
        weight: u32,
    ) -> Self {
        Self {
            id,
            cpu_time,
            cpu_burst_length,
            io_burst_length,
            state: Taskstatus::New,
            runtime: 0,
            vruntime: 0,
            idle_time: 0,
            start_time,
            weight,
        }
    }

    pub fn get_id(&self) -> u16 {
        self.id
    }

    pub fn get_cpu_time(&self) -> u64 {
        self.cpu_time
    }

    pub fn get_start_time(&self) -> u128 {
        self.start_time
    }

    pub fn get_status(&self) -> TaskStatus {
        self.state
    }

    pub fn get_runtime(&self) -> u64 {
        self.runtime
    }

    pub fn terminate(&mut self) {
        self.state = Taskstatus::Terminated;
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn vruntime(&mut self,now:u128) -> u64 {
        let dt: u64 = now.overflowing_sub(self.start_time).0 ad u64;
        let delta_exec_weightd:u64 = dt / (self.weight as u64);
        self.vruntime += delta_exec_weightd;

        self.vruntime

    }

    pub fn to_idle(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot yield a terminated task ({:?})!", self.id),
            _ => self.state = TaskStatus::Idle
        }
    }

    pub fn schedule(&mut self) {
        match self.state {
            TaskStatus::Terminated => panic!("Cannot schedule a terminated task: ({:?})!", self.id),
            _ => self.state = TaskStatus::Waiting
        }
    }
}
