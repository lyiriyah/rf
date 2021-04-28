use std::env;
use std::path::Path;
use passwd::Passwd;
use regex::Regex;

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
            username,
            hostname,
            cpu_name,
            editor,
            shell,
            memory,
        }
    }
}

pub fn get_cpu() -> String {
    let re = Regex::new(" +").unwrap();
    let cpuid = raw_cpuid::CpuId::new();
    let cpuextinf = cpuid.get_extended_function_info();
    
    re.replace_all(cpuextinf.as_ref().map_or_else(
        || "n/a",
        |extfuninfo| extfuninfo.processor_brand_string().unwrap_or("unreadable"),
    ), " ").trim().to_string()
}

pub fn get_editor() -> String {
    let mut editor = String::from("");
 
    if let Ok(path) = env::var("EDITOR") {
        if let Some(basename) = Path::new(&path).file_name() {
           if let Some(editorname) = basename.to_str() {
               editor = editorname.to_string();
           } else {
               println!("Could not cast basename to a String");
           };
        } else {
           println!("Could not get basename of editor {}", editor);
        }
    } else if let Err(e) = env::var("EDITOR") {
        println!("Failed to get editor with error {}", e);
    }
 
    editor
}

pub fn get_shell(username: &str) -> String {
    let mut shell = String::new();

    if let Some(user) = Passwd::from_name(username) {
        if let Some(basename) = Path::new(&user.shell).file_name() {
            if let Some(shellname) = basename.to_str() {
                shell = shellname.to_string();
            } else {
                println!("Could not cast basename to a String");
            }
        } else {
            println!("Failed to get path of shell {}", user.shell);
        }
    } else {
        println!("Failed to get shell for user {}", username)
    }
    
    shell
}

pub fn get_meminfo() -> sys_info::MemInfo {
    match sys_info::mem_info() {
        Ok(meminfo) => meminfo,
        Err(e) => panic!("Failed to get memory info with error {}", e)
    }
}
