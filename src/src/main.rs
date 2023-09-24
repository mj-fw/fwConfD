extern crate daemonize;
fn main() {

    let daemonize = Daemonize::new()
        .pid_file("/opt/fw/tmp/fwConfd.pid")
        .chown_pid_file(true)
        .working_directory("/opt/fw/tmp")
        .user("nobody")
        .group("daemon");

    match daemonize.start() {
        Ok(_) => println!("fwConfd daemonized"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
