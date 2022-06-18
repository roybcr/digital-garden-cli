use color_eyre::eyre::{
    eyre, 
    Result, 
    WrapErr
};
use digital_garden::write;
use directories::UserDirs;
use std::path::PathBuf;
use structopt::StructOpt;

/// A CLI for the growing and curation of a Digital Garden.
#[derive(Debug, StructOpt)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(
        parse(from_os_str), 
        short = "p", 
        long, 
        env,
    )]
    garden_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    /// Write something in your garden
    Write {
        #[structopt(short, long)]
        /// Optionally set a title for what you're going to write about
        title: Option<String>,
    },
}

fn get_default_garden_dir(prefix: &str) -> Result<PathBuf> {
    let user_dirs: UserDirs =
        UserDirs::new().ok_or_else(|| 
            eyre!("Couldn't find home directory"))?;

    Ok(user_dirs.home_dir().join(&format!("{}-garden", prefix)))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt: Opt = Opt::from_args();
    println!("{:?}", opt);
    dbg!(&opt);

    let garden_path: PathBuf = match opt.garden_path {
        Some(pathbuf) => Ok(pathbuf),
        None => 
            get_default_garden_dir("my")
                .wrap_err("`garden_path` was not supplied"),
    }?;

    match opt.cmd {
        Command::Write { 
            title 
        } => write(garden_path, title),
    }
}
