use clap::Args;

#[derive(Args, Debug)]

pub(crate) struct MsrvArg {
    #[arg(short='F', long, num_args=1.., value_delimiter=',')]
    pub(crate) features: Option<Vec<String>>,
}
