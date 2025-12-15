use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    pub(crate) year: u16,
    pub(crate) day: u8,
    pub(crate) level: u8,

    /// only fetch input; do not run solution
    #[arg(long)]
    pub(crate) fetch_input: bool,
}
