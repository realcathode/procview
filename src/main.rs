use sysinfo::System;

const BYTES_PER_MIB: u64 = 1_048_576;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    println!("Mem (MiB): {}", sys.total_memory() / BYTES_PER_MIB);
    println!("Used mem (MiB): {}", sys.used_memory() / BYTES_PER_MIB);
    println!("Swap (MiB): {}", sys.total_swap() / BYTES_PER_MIB);
    println!("Used swap (MiB): {}", sys.used_swap() / BYTES_PER_MIB);   
    
}
