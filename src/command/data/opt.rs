use structopt::StructOpt;

use crate::command::data::command::Command;

#[derive(Debug, StructOpt)]
#[structopt(name = "blockchain_rust")]
pub struct Opt {
    #[structopt(subcommand)]
    pub command: Command,
}
