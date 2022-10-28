use clap::Args;

#[derive(Debug, Args)]
#[clap()]
pub struct EditCommandOpts {

} impl ToString for EditCommandOpts {
    fn to_string(&self) -> String {
        format!("{{}}")
    }
}