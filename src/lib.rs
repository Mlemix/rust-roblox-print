use winapi::um::libloaderapi::GetModuleHandleA; 
use std::ffi::CString;
use std::os::raw::c_char;
use std::mem::transmute;
use std::ptr;

#[repr(u8)]
enum Colours {
    Regular,
    Info,
    Warn,
    Error
}

unsafe fn entry() {
    // address is rebased to 0 for version-af653eb90d574aa0
    let intermediate_ptr = (GetModuleHandleA(ptr::null_mut()) as usize + 0xc2dce0) as *const ();
    let print = transmute::<*const (), extern "C" fn (Colours, *const c_char, ...) -> ()>(intermediate_ptr);

    let mut print_msg = CString::new("this is white").unwrap();
    print(Colours::Regular, print_msg.as_ptr());
    std::thread::sleep(std::time::Duration::from_millis(100));

    print_msg = CString::new("this is blue").unwrap();
    print(Colours::Info, print_msg.as_ptr());
    std::thread::sleep(std::time::Duration::from_millis(100));

    print_msg = CString::new("this is yellow").unwrap();
    print(Colours::Warn, print_msg.as_ptr());
    std::thread::sleep(std::time::Duration::from_millis(100));

    print_msg = CString::new("this is red").unwrap();
    print(Colours::Error, print_msg.as_ptr());
}

#[no_mangle]
unsafe extern "system" fn DllMain(_module: usize, reason_for_call: u8, _reserved: usize) -> bool {
    match reason_for_call {
        1 => {
            std::thread::spawn(move || entry());
            true
        }
        _ => true
    }
}