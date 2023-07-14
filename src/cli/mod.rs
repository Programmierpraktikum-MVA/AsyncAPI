use clap::Parser;

//Add cool slogan for your app here, e.g.:
/// specify the specification file and the output directory
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Add a positional argument that the user has to supply:
    /// The specification file to parse      
    #[arg(short, long)]
    pub specification: String,

    /// The output directory
    /// If not specified, the output will be written to ./output/
    /// If the directory does not exist, it will be created
    #[arg(short, long, default_value = "./output/")]
    pub output: String,

    /// The name of the project
    /// If not specified, the name will be taken from the specification file
    #[arg(short, long)]
    pub title: Option<String>,

    ///Optionally generate Rustdoc for generated Microservice
    #[arg(short, long, default_value = "false")]
    pub doc: bool,

    ///Optionally enable logging in generated Microservice
    #[arg(short, long, default_value = "false")]
    pub log: bool,
}
