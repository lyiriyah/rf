mod info;
mod pprinter;

fn main() {
    let hostname = whoami::hostname();
    let username = whoami::username();
    let cpuinfo = info::get_cpu();
    let editor = info::get_editor();
    let shell = info::get_shell(&username);
    let meminfo = info::get_meminfo();

    let system_info = info::Info::new(username, hostname, cpuinfo, editor, shell, meminfo);

    pprinter::pprint_info(&system_info);
}
