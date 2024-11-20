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
    match send_file_internal(file_path,address)//Calls The send_n_internal_function
    {
        Ok(_)=>0,
        Err(_)=>-1,
    };

}

fn send_file_internal(file_path:&str,address:&str) -> io::Result<()>
{
    let mut file = File::open(file_path)?;
    let mut tcp = TcpStream::connect(address)?;
    let mut buffer=[0;1024];

    while let Ok(bytes_read)=file.read(&mut buffer)
    {
        if bytes_read == 0
        {
            break;
        }
        tcp.write_all(&buffer[..bytes_read])?;
    }
    Ok(())

}
#[no_mangle]

pub extern "C" fn recieve(file_path: const *i8,port: u16) -> i32
{
    let c_file_path=unsafe{std::ffi::Cstr:from_ptr(file_path)};
    let file_path=match c_file_path.to_str()
    {
        Ok(path)=>path,
        Err(_)=>return -1,
    };
    match recieve_file_internal(file_path,port)
    {
        Ok(_)=>0,
        Err(_)=>-1,
    }
}
