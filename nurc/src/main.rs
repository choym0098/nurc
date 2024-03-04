use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Checkpoint a running container
    Checkpoint,

    /// Create a container
    Create,

    /// Delete any resources held by the container; often used with detached containers
    Delete,

    /// Display container events, such as OOM notifications, CPU, memory, I/O and network statistics
    Events,

    /// Execute a new process inside the container
    Exec,

    /// Send a specified signal to the container's init process
    Kill,

    /// List containers started by runc with the given --root
    List,

    /// Suspend all processes inside the container
    Pause,

    ///  Show processes running inside the container
    Ps,

    /// Restore a container from a previous checkpoint. See runc-restore(8).
    Restore,

    /// Resume all processes that have been previously paused. See runc-resume(8).
    Resume,

    /// Create and start a container. See runc-run(8).
    Run,

    /// Create a new specification file (config.json). See runc-spec(8).
    Spec,

    /// Start a container previously created by runc create. See runc-start(8).
    Start,

    /// Show the container state. See runc-state(8).
    State,

    /// Update container resource constraints. See runc-update(8).
    Update,
}

fn main() {
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::Checkpoint => {
            println!("'nurc checkpoint' was used")
        },
        Commands::Create => {
            println!("'nurc create' was used")
        },
        Commands::Delete => {
            println!("'nurc delete' was used")
        },
        Commands::Events => {
            println!("'nurc events' was used")
        },
        Commands::Exec => {
            println!("'nurc exec' was used")
        },
        Commands::Kill => {
            println!("'nurc kill' was used")
        },
        Commands::List => {
            println!("'nurc list' was used")
        },
        Commands::Pause => {
            println!("'nurc pause' was used")
        },
        Commands::Ps => {
            println!("'nurc ps' was used")
        },
        Commands::Restore => {
            println!("'nurc restore' was used")
        },
        Commands::Resume => {
            println!("'nurc resume' was used")
        },
        Commands::Run => {
            println!("'nurc run' was used")
        },
        Commands::Spec => {
            println!("'nurc spec' was used")
        },
        Commands::Start => {
            println!("'nurc start' was used")
        },
        Commands::State => {
            println!("'nurc state' was used")
        },
        Commands::Update => {
            println!("'nurc update' was used")
        },
    }
}