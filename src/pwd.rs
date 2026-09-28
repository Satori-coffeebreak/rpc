use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, BufWriter, Write};

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0",
    about = "現在の作業ディレクトリのパスを返す。"
)]
struct Args {
    strings: Vec<String>,
}