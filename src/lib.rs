pub mod pklp;
use std::os::raw::c_char;
pub type CStr = *const c_char;

#[no_mangle]
pub extern "C" fn pklp_str_to_json(data: CStr) -> CStr {
    string_to_c_str(pklp::str_to_json(c_str_to_str(data)))
}

#[no_mangle]
pub extern "C" fn pklp_path_to_json(path: CStr) -> CStr {
    string_to_c_str(pklp::path_to_json(c_str_to_str(path)))
}

#[no_mangle]
pub extern "C" fn pklp_path_to_json_file(path: CStr, output_path: CStr) {
    pklp::path_to_json_file(c_str_to_str(path), c_str_to_str(output_path));
}

#[no_mangle]
pub extern "C" fn pklp_strs_to_json(data: *const CStr, n: usize) -> CStr {
    string_to_c_str(pklp::strs_to_json(&c_str_array_to_vec(data, n)))
}

#[no_mangle]
pub extern "C" fn pklp_paths_to_json(paths: *const CStr, n: usize) -> CStr {
    string_to_c_str(pklp::paths_to_json(&c_str_array_to_vec(paths, n)))
}

#[no_mangle]
pub extern "C" fn pklp_paths_to_json_file(paths: *const CStr, n: usize, output_path: CStr) {
    pklp::paths_to_json_file(&c_str_array_to_vec(paths, n), c_str_to_str(output_path));
}

fn c_str_to_str(s: CStr) -> &'static str {
    let c_str = unsafe { std::ffi::CStr::from_ptr(s) };
    c_str.to_str().unwrap()
}

fn string_to_c_str(v: String) -> CStr {
    let s = std::ffi::CString::new(v).unwrap();
    let p = s.as_ptr();
    std::mem::forget(s);
    p
}

fn c_str_array_to_vec(data: *const CStr, n: usize) -> Vec<&'static str> {
    let mut s = Vec::new();
    for i in 0..n {
        let p = unsafe { data.add(i).read() };
        s.push(c_str_to_str(p));
    }
    s
}
