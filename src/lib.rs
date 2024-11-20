use std::fs::File;
use std::io:{self,Read,Write};
use std::net::{TcpListener,TcpStream};
use std::path:Path;

#[no_mangle]

pub extern "C" fn send(file_path: *const i8, address: *const i8) -> i32
{
    let c_file_path=unsafe{std::ffi::CStr::from_ptr(file_path)};//unsafe because raw pointers are unsafe inherently
    let c_address=unsafe{std::ffi:Cstr::from_ptr(address)};
    let file_path=match c_file_path.to_str()
    {
        Ok(path) => path,
        Err(_) => return -1,
    };
    let address=match c_address.to_str()
    {
        Ok(addr)=>addr,
        Err(_)=>return -1,
    };
    
}
