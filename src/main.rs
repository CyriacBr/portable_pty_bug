fn main() {
    use portable_pty::{native_pty_system, CommandBuilder, PtySize, PtySystem};

    // Use the native pty implementation for the system
    let pty_system = native_pty_system();

    // Create a new pty
    let mut pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        // Not all systems support pixel_width, pixel_height,
        // but it is good practice to set it to something
        // that matches the size of the selected font.  That
        // is more complex than can be shown here in this
        // brief example though!
        pixel_width: 0,
        pixel_height: 0,
    }).expect("err");

    // Spawn a shell into the pty
    let cmd = CommandBuilder::new("bash");
    let child = pair.slave.spawn_command(cmd).expect("err");

    // Read and parse output from the pty with reader
    let mut reader = pair.master.try_clone_reader().expect("err");

    // Send data to the pty by writing to the master
    writeln!(pair.master, "ls -l\r\n").expect("err");
}
