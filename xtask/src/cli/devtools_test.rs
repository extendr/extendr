use clap::Args;

#[derive(Args, Debug)]
pub(crate) struct DevtoolsTestArg {
    #[arg(
        long,
        short = 'a',
        default_value = "false",
        help = "Accept newly generated macro-snapshots"
    )]
    pub(crate) snapshot_accept: bool,
}
