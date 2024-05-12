use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct DevtoolsTestArg {
    #[arg(
        long,
        short,
        default_value = "false",
        help = "Accept newly generated macro-snapshots"
    )]
    pub(crate) accept_snapshot: bool,

    #[arg(
        long,
        short,
        help = "Filter tests by name",
    )]
    pub(crate) filter: Option<String>,
}
