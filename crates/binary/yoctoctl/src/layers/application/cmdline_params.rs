
#[derive(Clap, Debug)]
pub struct YoctoctlCmdline {
    #[clap(subcommand)]
    pub command: AppCommand
}

#[derive(Clap, Debug)]
pub enum AppCommand {
    Generate(GenerateProjectParams)
}

#[derive(Clap, Debug)]
pub struct GenerateProjectParams {
    #[clap()]
    pub config_file: String,

    #[clap()]
    pub output_directory: Option<String>
}
