use std::ffi::CStr;
use std::mem;

// Bind to libproc functions
unsafe extern "C" {
    fn proc_listpids(type_: u32, typeinfo: u32, buffer: *mut libc::c_void, buffersize: i32) -> i32;
    fn proc_name(pid: i32, buffer: *mut libc::c_void, buffersize: u32) -> i32;
}

// Constants for proc_listpids
const PROC_ALL_PIDS: u32 = 1;

fn main() {
    match get_process_list() {
        Ok(processes) => {
            println!("{:>8} {}", "PID", "COMMAND");
            for (pid, name) in processes {
                println!("{:>8} {}", pid, name);
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_process_list() -> Result<Vec<(i32, String)>, String> {
    // First, get the number of processes
    let num_pids = unsafe {
        proc_listpids(PROC_ALL_PIDS, 0, std::ptr::null_mut(), 0)
    };
    
    if num_pids <= 0 {
        return Err("Failed to get process count".to_string());
    }

    // Allocate buffer for PIDs
    let buffer_size = num_pids * mem::size_of::<i32>() as i32;
    let mut pid_buffer: Vec<i32> = vec![0; (buffer_size / 4) as usize];

    // Get the actual PIDs
    let actual_size = unsafe {
        proc_listpids(
            PROC_ALL_PIDS,
            0,
            pid_buffer.as_mut_ptr() as *mut libc::c_void,
            buffer_size,
        )
    };

    if actual_size <= 0 {
        return Err("Failed to get process list".to_string());
    }

    let actual_count = (actual_size / mem::size_of::<i32>() as i32) as usize;
    pid_buffer.truncate(actual_count);

    // Get process names for each PID
    let mut processes = Vec::new();
    for &pid in &pid_buffer {
        if pid > 0 {
            let name = get_process_name(pid).unwrap_or_else(|_| "<unknown>".to_string());
            processes.push((pid, name));
        }
    }

    // Sort by PID for consistent output
    processes.sort_by_key(|&(pid, _)| pid);
    
    Ok(processes)
}

fn get_process_name(pid: i32) -> Result<String, String> {
    let mut buffer = [0u8; 256];
    
    let ret = unsafe {
        proc_name(pid, buffer.as_mut_ptr() as *mut libc::c_void, buffer.len() as u32)
    };
    
    if ret <= 0 {
        return Err("Failed to get process name".to_string());
    }
    
    let name = unsafe {
        CStr::from_ptr(buffer.as_ptr() as *const i8)
            .to_string_lossy()
            .into_owned()
    };
    
    Ok(name)
}
