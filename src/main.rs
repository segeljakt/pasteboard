use {pasteboard::Pasteboard, std::path::PathBuf, structopt::StructOpt};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "copy")]
    Copy {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
        #[structopt(
            long = "type",
            short = "t",
            raw(possible_values = "&Pasteboard::variants()"),
            case_insensitive = true,
            raw(default_value = "&Pasteboard::variants()[0]"),
            help = "Data type of file contents"
        )]
        pb: Pasteboard,
    },
    #[structopt(name = "paste")]
    Paste {
        #[structopt(parse(from_os_str))]
        output: PathBuf,
        #[structopt(
            long = "type",
            short = "t",
            raw(possible_values = "&Pasteboard::variants()"),
            case_insensitive = true,
            raw(default_value = "&Pasteboard::variants()[0]"),
            help = "Data type of file contents"
        )]
        pb: Pasteboard,
    },
}

fn main() {
    let opt = Opt::from_args();
    unsafe {
        match opt.cmd {
            Command::Copy { input, pb } => {
                if let Some(path) = input.to_str() {
                    pb.copy(&path)
                }
            }
            Command::Paste { output, pb } => {
                if let Some(path) = output.to_str() {
                    pb.paste(&path)
                }
            }
        }
    }
}
