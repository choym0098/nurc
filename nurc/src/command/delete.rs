use clap::Args;

#[derive(Args)]
pub struct DeleteArgs {
    /// Forcibly delete the running container, using SIGKILL signal(7) to stop it first.
    #[arg(short, long)]
    force: bool,
}