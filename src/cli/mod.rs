use structopt::StructOpt;

//Add cool slogan for your app here, e.g.:
/// specify the specification file and the output directory
#[derive(Debug, StructOpt)]
pub struct Cli {
    // Add a positional argument that the user has to supply:
    /// The file to read
    #[structopt(
        long = "spec",
        short = "s",
        default_value = "./example/specs/userSignupSubscriber.yaml"
    )]
    pub specification_file: String,

    /// The output directory
    /// If not specified, the output will be written to ./output/
    /// If the directory does not exist, it will be created
    #[structopt(long = "output", short = "o", default_value = "./output/")]
    pub output_directory: String,

    /// The name of the project
    /// If not specified, the name will be taken from the specification file
    #[structopt(long = "title", short = "t")]
    pub project_title: Option<String>,
}
