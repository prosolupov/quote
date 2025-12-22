#[derive(Parser)]
pub struct CliCommands {
    #[command(subcommand)]
    pub command: Commands
}

#[derive(Subcommand)]
pub enum Commands {
    Stream {
        endpoint: String,

        #[arg(value_delimiter=',')]
        tickets: Vec<String>,
    }
}