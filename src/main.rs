use raw_cpuid;
use whoami;
use std::env;
use std::path::Path;
use passwd::Passwd;
use colored::*;
use regex::Regex;
use sys_info;

fn get_cpu_info() -> String {
    let re = Regex::new(" +").unwrap();
    let cpuid = raw_cpuid::CpuId::new();
    let cpuextinf = cpuid.get_extended_function_info();
    
    return re.replace_all(cpuextinf.as_ref().map_or_else(
            || "n/a",
            |extfuninfo| extfuninfo.processor_brand_string().unwrap_or("unreadable"),
        ), " ").to_string();
}

fn get_editor() -> String {
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

fn get_shell() -> String {
    let mut shell = String::new();

    match Passwd::from_name(&username) {
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

fn get_meminfo() -> sys_info::MemInfo {
    return match sys_info::mem_info() {
        Ok(meminfo) => meminfo,
        Err(e) => std::panic!("Failed to get memory info with error {}", e)
    };
}

fn main() {
    let hostname = whoami::hostname();
    let username = whoami::username();

    let cpuinfo = get_cpu_info();
    let editor = get_editor();
    let shell = get_shell();
    let meminfo = get_meminfo();

    let mut topline = String::from("");

    for i in 0..(username.len() + hostname.len() + 1) {
        if i == (username.len() + hostname.len() + 1) / 2 {
            topline.push('┬')
        } else {
            topline.push('─');
        }
    }
    
    println!("{}@{}", username.green(), hostname.green());
    println!("{}", topline);
    println!("  {} │ {}", "distro".bright_blue(), whoami::distro());
    println!("  {} │ {}", "  arch".bright_blue(), env::consts::ARCH);
    println!("  {} │ {}", "editor".bright_blue(), editor);
    println!("  {} │ {}", " shell".bright_blue(), shell);
    println!("  {} │ {}", "   cpu".bright_blue(), cpuinfo);
    println!("  {} │ {} MB", "memory".bright_blue(), (meminfo.total / 1024));
    println!("  {} │ {} MB/{} MB", " usage".bright_blue(), ((meminfo.total - meminfo.avail) / 1024), (meminfo.total / 1024));
}
