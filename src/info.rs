use raw_cpuid;
use std::env;
use std::path::Path;
use passwd::Passwd;
use regex::Regex;
use sys_info;

pub struct Info {
    pub username: String,
    pub hostname: String,
    pub cpu_name: String,
    pub editor: String,
    pub shell: String,
    pub memory: sys_info::MemInfo,
}

impl Info {
    pub fn new(
        username: String,
        hostname: String,
        cpu_name: String,
        editor: String,
        shell: String,
        memory: sys_info::MemInfo) -> Info {
        Info {
            username: username,
            hostname: hostname,
            cpu_name: cpu_name,
            editor: editor,
            shell: shell,
            memory: memory
        }
    }
}

pub fn get_cpu_info() -> String {
    let re = Regex::new(" +").unwrap();
    let cpuid = raw_cpuid::CpuId::new();
    let cpuextinf = cpuid.get_extended_function_info();
    
    return re.replace_all(cpuextinf.as_ref().map_or_else(
        || "n/a",
        |extfuninfo| extfuninfo.processor_brand_string().unwrap_or("unreadable"),
    ), " ").to_string();
}

pub fn get_editor() -> String {
    let mut editor = String::from("");
    let mut editorpath = String::new();
    
    match env::var("EDITOR") {
        Ok(path) => editorpath = path,
        Err(e) => println!("Failed to get editor with error {}", e)
    }

    match Path::new(&editorpath).file_name() {
        Some(path) => match path.to_str() {
            Some(basename) => editor = basename.to_string(),
            None => println!("Failed to convert the basename of {} to a &str", editorpath)
        },
        None => println!("Could not get basename of editor {}", editor)
    }

    return editor;
}

pub fn get_shell(username: &str) -> String {
    let mut shell = String::new();

    match Passwd::from_name(username) {
        Some(user) => match Path::new(&user.shell).file_name() {
            Some(basename) => match basename.to_str() {
                Some(basestr) => shell = basestr.to_string(),
                None => println!("Failed to convert the basename of {} to a &str", user.shell)
            },
            None => println!("Failed to get path of shell {}", user.shell) 
        },
        None => println!("Failed to get shell for user {}", username)
    }

    return shell;
}

pub fn get_meminfo() -> sys_info::MemInfo {
    return match sys_info::mem_info() {
        Ok(meminfo) => meminfo,
        Err(e) => panic!("Failed to get memory info with error {}", e)
    };
}
