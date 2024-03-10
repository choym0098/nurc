use clap::Args;

#[derive(Args)]
pub struct DeleteArgs {
    #[arg(short, long)]
    force: bool,
}