use kernel32;
use winapi;

use winapi::{
    DWORO, // Rust 里就是 u32

    HANDLE, // 各种内部 API 的指针类型，没有关联类型。
            // 在 Rust 里 std::os::raw::c_void 定义了 void 指针
    LPVOID, // Handle 是指向 Windows 内一些不透明资源的指针

    PVOID, // Windows 里，数据类型名的前缀通常是其类型的缩写
    SIZE_T, // 这台机器上 u64 是 usize
    LPSYSTEM_INFO, // 到 SYSTEM_INFO struct 的指针

    MEMORY_BASIC_INFORMATION as MEMINFO, // Windows 内部定义的一些 Struct
    SYSTEM_INFO,
};

fn main() {
    // 这些变量将在 unsafe 块进行初始化
    let this_pid: DWORO;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMINFO;

    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMINFO>();

    // 保证所有的内存都初始化了
    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    // 系统调用
    unsafe {
        this_pid = kernel32::GetCurrentProcessId();
        this_proc = kernel32::GetCurrentProcess();
        // 下面代码使用 C 的方式将结果返回给调用者。
        // 提供一个到预定义 Struct 的指针，一旦函数返回就读取 Struct 的新值
        kernel32::GetSystemInfo(&mut proc_info as LPSYSTEM_INFO);
    };

    // 对变量重命名
    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("{:?} @ {:p}", this_pid, this_proc);
    println!("{:?}", proc_info);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);

    // 扫描地址空间
    loop {
        let rc: SIZE_T = unsafe {
            // 提供运行程序内存地址空间特定段的信息，从 base_addr 开始
            kernel32::VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE as usize)
        };

        if rc == 0 {
            break;
        }

        println!("{:#?}", mem_info);
        base_addr = ((base_addr as u64) + mem_info.RegionSize) as PVOID;
    }
}
