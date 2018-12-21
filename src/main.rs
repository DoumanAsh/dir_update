use structopt::StructOpt;
use dir_update::DirUpdate;

use std::path::{Path, PathBuf};

#[derive(StructOpt, Debug)]
#[structopt(name = "dir_update", author = "", about = "Copy files into directory, if they are updated", raw(setting = "structopt::clap::AppSettings::ArgRequiredElseHelp"))]
struct Args {
    #[structopt(long = "verbose")]
    ///Enables verbose output to print each action.
    verbose: bool,
    #[structopt(name = "FROM", parse(from_os_str))]
    ///Directory from which to copy files.
    from: PathBuf,
    #[structopt(name = "TO", parse(from_os_str))]
    ///Directory into which to copy files.
    to: PathBuf,
}

struct NormalUpdater;
impl DirUpdate for NormalUpdater {
    fn on_new_file_update(file: &Path) {
        println!("{}: Updated...", file.display());
    }
}

struct VerboseUpdater;
impl DirUpdate for VerboseUpdater {
    fn on_new_file_update(file: &Path) {
        println!("{}: Updated...", file.display());
    }

    fn on_new_file_skip(file: &Path) {
        println!("{}: Skipped...", file.display());
    }
}

fn main() -> Result<(), dir_update::UpdateError> {
    let opt = Args::from_args();

    match opt.verbose {
        false => NormalUpdater::update_dir(&opt.from, &opt.to),
        true => VerboseUpdater::update_dir(&opt.from, &opt.to),
    }.map(|num| println!("{} files are updated", num))
}
