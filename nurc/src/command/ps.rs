use clap::{Args, ValueEnum};

#[derive(Args)]
pub struct PsArgs {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = PsFormatMode::Table)]
    format: PsFormatMode,

    container_id: i64,
    /// Any ps(1) options can be used, but some might break the filtering.
    /// In particular, if PID column is not available, an error is returned,
    /// and if there are columns with values containing spaces before the PID column, the result is undefined.
    ps_options: Vec<String>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum PsFormatMode {
    /// Generates results in a table format
    Table,
    /// Shows a mere array of PIDs belonging to a container; if used, all ps options are gnored
    Json,
}
