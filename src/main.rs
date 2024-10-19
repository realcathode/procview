use sysinfo::System;

const BYTES_PER_MIB: u64 = 1_048_576;

#[derive(Debug)]
struct Process {
    pid: u32,
    uid: u32,
    cpu: f32,
    mem: u64,       
}           

impl Process {
    fn new(pid: u32, uid: u32, cpu: f32, mem: u64) -> Self {
        Self {
            pid,
            uid,
            cpu,
            mem,
        }
    }
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Mem (MiB): {}", sys.total_memory() / BYTES_PER_MIB);
    println!("Used mem (MiB): {}", sys.used_memory() / BYTES_PER_MIB);
    println!("Swap (MiB): {}", sys.total_swap() / BYTES_PER_MIB);
    println!("Used swap (MiB): {}", sys.used_swap() / BYTES_PER_MIB);   
    
    let mut procs: Vec<Process> = Vec::new();
    for (pid, process) in sys.processes() {
        // Check if user_id is available
        if let Some(uid ) = process.user_id() {
            let nproc = Process::new(pid.as_u32(), **uid, process.cpu_usage(), process.memory() / BYTES_PER_MIB);
            procs.push(nproc);
        }
    }
    for p in procs {
        println!("{:?}", p);
    }
}
