use clap::Args;

#[derive(Debug, Args)]
#[clap()]
pub struct InitCommandOpts {
    
} impl ToString for InitCommandOpts {
    fn to_string(&self) -> String {
        format!("{{}}")
    }
}