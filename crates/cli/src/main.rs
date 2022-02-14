use clap::{Parser, Subcommand};

mod build;
mod new;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Command,
}

#[derive(Subcommand)]
enum Command {
  New(new::NewCommand),
  Build(build::BuildCommand),
}

fn main() {
  let cli = Cli::parse();

  match cli.command {
    Command::New(new_command) => {
      println!("{:?}", new_command);
    }
    Command::Build(build_command) => {
      println!("{:?}", build_command);
    }
  }
}
