use anyhow::{Context, Result, bail};
use clap::{Parser, ArgGroup};
use std::{env, fs, io::{self, BufWriter, Write}, path::{self, Path}};

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0",
    about = "現在の作業ディレクトリのパスを返す。"
)]
#[command(group(
    ArgGroup::new("ag")
        .required(false)
        .args(["logical", "physical"])
))]
struct Args {
    #[arg(short = 'L')]
    logical: bool,

    #[arg(short = 'P')]
    physical: bool,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    if args.physical {
        match env::current_dir() {
            Ok(pwd) => {
                println!("{}", pwd.to_str().unwrap());
                Ok(())
            },
            Err(e) => bail!("作業ディレクトリの取得に失敗しました: {}", e),
        }
    }
    else {
        match env::var("PWD") {
            Ok(pwd) => {
                println!("{}", pwd);
                Ok(())
            },
            Err(e) => bail!("作業ディレクトリの取得に失敗しました: {}", e),
        }
    }
}