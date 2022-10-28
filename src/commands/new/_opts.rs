use clap::Args;

#[derive(Debug, Args)]
#[clap()]
pub struct NewCommandOpts {

} impl ToString for NewCommandOpts {
    fn to_string(&self) -> String {
        format!("{{}}")
    }
}