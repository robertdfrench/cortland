use std::ffi::CStr;

// Bind to libc functions
unsafe extern "C" {
    fn getuid() -> u32;
    fn getpwuid(uid: u32) -> *mut Passwd;
}

// Simplified passwd structure
#[repr(C)]
struct Passwd {
    pw_name: *mut libc::c_char,
    pw_passwd: *mut libc::c_char,
    pw_uid: u32,
    pw_gid: u32,
    pw_gecos: *mut libc::c_char,
    pw_dir: *mut libc::c_char,
    pw_shell: *mut libc::c_char,
}

fn main() {
    match get_username() {
        Ok(username) => println!("{}", username),
        Err(e) => {
            eprintln!("whoami: {}", e);
            std::process::exit(1);
        }
    }
}

fn get_username() -> Result<String, String> {
    // Get the current user ID
    let uid = unsafe { getuid() };
    
    // Get the passwd entry for this UID
    let passwd_ptr = unsafe { getpwuid(uid) };
    
    if passwd_ptr.is_null() {
        return Err("cannot find username for current user".to_string());
    }
    
    // Extract the username from the passwd structure
    let passwd = unsafe { &*passwd_ptr };
    
    if passwd.pw_name.is_null() {
        return Err("username field is null".to_string());
    }
    
    let username = unsafe {
        CStr::from_ptr(passwd.pw_name)
            .to_string_lossy()
            .into_owned()
    };
    
    Ok(username)
}
