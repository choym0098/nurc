use clap::Args;

#[derive(Args)]
pub struct CreateArgs {
    /// Path to the root of the bundle directory.
    #[arg(short, long, default_value_t = String::from("."))]
    bundle: String,

    /// Path to an AF_UNIX socket which will receive a file descriptor referencing the master end of the console's pseudoterminal.
    #[arg(long = "console-socket")]
    console_socket: Option<String>,

    /// Specify the file to write the initial container process' PID to.
    #[arg(long = "pid-file")]
    pid_file: Option<String>,

    /// Do not use pivot root to jail process inside rootfs. This should not be used except in exceptional circumstances, and may be unsafe from the security standpoint.
    #[arg(long = "no-pivot")]
    no_pivot: bool,

    /// Do not create a new session keyring for the container. This will cause the container to inherit the calling processes session key.
    #[arg(long = "no-new-keyring")]
    no_new_keyring: bool,

    /// Pass N additional file descriptors to the container (stdio + $LISTEN_FDS + N in total).
    #[arg(long = "preserve-fds", default_value_t = 0)]
    preserve_fds: u64,
}
