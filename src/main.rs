use sysinfo::System;
use std::{thread, time};

const BYTES_PER_MIB: u64 = 1_048_576;

struct Process {
    pid: u32,
    uid: u32,
    cpu: f32,
    mem: u64,       
}           

impl Process {
    fn new(pid: u32, uid: u32, cpu: f32, mem: u64) -> Self {
        Self {pid, uid, cpu, mem }
    }
}

fn main() {
    loop {    
        let mut sys = System::new_all();
        sys.refresh_all();

        println!("Mem (MiB): {}", sys.total_memory() / BYTES_PER_MIB);
        println!("Used mem (MiB): {}", sys.used_memory() / BYTES_PER_MIB);
        println!("Swap (MiB): {}", sys.total_swap() / BYTES_PER_MIB);
        println!("Used swap (MiB): {}", sys.used_swap() / BYTES_PER_MIB);   
        
        let mut procs: Vec<Process> = Vec::new();
        for (pid, process) in sys.processes() {
            if let Some(uid ) = process.user_id() {
                let proc = Process::new(
                    pid.as_u32(),
                    **uid,
                    process.cpu_usage(),
                    process.memory() / BYTES_PER_MIB,
                );
                procs.push(proc);
            }
            if procs.len() == 10 {
                break;
            }
        }   
        
        procs.sort_by(|a, b| b.mem.partial_cmp(&a.mem).unwrap_or(std::cmp::Ordering::Equal));

        for p in &procs {
            println!(
                "PID: {}, UID: {}, CPU: {}, Mem (MiB): {}",
                p.pid, p.uid, p.cpu, p.mem
            );
        }
        let ten_millis = time::Duration::from_millis(1000);
        let now = time::Instant::now();
    
        thread::sleep(ten_millis);
    
        assert!(now.elapsed() >= ten_millis);   
    }
}
