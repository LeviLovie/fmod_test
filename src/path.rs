use std::ffi::CString;

pub fn path(path: &str) -> CString {
    let exec_path = std::env::current_exe().expect("Failed to get current executable path");
    let path = exec_path
        .parent()
        .expect("Failed to get parent directory")
        .join("banks")
        .join(path)
        .to_str()
        .unwrap()
        .to_string();

    CString::new(path).expect("Failed to create CString")
}
