use anyhow::{Context, Result, bail};
use clap::{Parser, ArgGroup};
use std::{env, io::{self, BufWriter, Write}};

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

    match env::var("PWD") {
        Ok(val) => {
            println!("{}", val);
            Ok(())
        },
        Err(e) => {
            bail!("PWD変数が設定されていません: {}", e);
        }
    }
}