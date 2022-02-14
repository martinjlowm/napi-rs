use clap::Args;

#[derive(Args, Debug)]
#[clap(version)]
/// create a new project with pre-configured boilerplate
pub struct NewCommand {
  #[clap(short, long)]
  name: String,
}
