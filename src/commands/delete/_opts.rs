use clap::Args;

#[derive(Debug, Args)]
#[clap()]
pub struct DeleteCommandOpts {

} impl ToString for DeleteCommandOpts {
    fn to_string(&self) -> String {
        format!("{{}}")
    }
}