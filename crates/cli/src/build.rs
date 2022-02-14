use clap::Args;

#[derive(Args, Debug)]
#[clap(version)]
/// build the napi-rs crate
pub struct BuildCommand {
  #[clap(short, long)]
  target: String,
}
