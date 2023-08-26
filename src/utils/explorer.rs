use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32First, Process32Next, PROCESSENTRY32, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};

pub fn kill_explorer() {
    let snapshot = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }.unwrap();
    if snapshot.is_invalid() {
        panic!("CreateToolhelp32Snapshot failed");
    }

    unsafe {
        let pe: *mut PROCESSENTRY32 = &mut std::mem::zeroed();
        (*pe).dwSize = std::mem::size_of::<PROCESSENTRY32>() as u32;

        let result = Process32First(snapshot, pe);
        if result.is_ok() {
            loop {
                let file_string = convert_sz_to_string((*pe).szExeFile);
                if file_string.starts_with("explorer.exe") {
                    let pid = (*pe).th32ProcessID;
                    let h_proc = OpenProcess(PROCESS_TERMINATE, false, pid);
                    if h_proc.is_ok() {
                        let _ = TerminateProcess(h_proc.unwrap(), 0);
                    }
                    break;
                }

                let result = Process32Next(snapshot, pe);

                if !result.is_ok() {
                    break;
                }
            }
        }
    }
}

fn convert_sz_to_string(file: [u8; 260]) -> String {
    let chars = file
        .map(|x| if x == 0 { ' ' } else { char::from(x) })
        .into_iter()
        .collect::<String>();
    chars
}
