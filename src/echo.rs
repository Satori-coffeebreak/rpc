use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, BufWriter, Write};

#[derive(Parser, Debug)]
#[command(
    version = "1.0.0",
    about = "引数に渡された文字列を全て半角スペースで区切って出力する。 "
)]
struct Args {
    strings: Vec<String>,
}

pub fn main() -> Result<()> {
    let args = Args::parse();

    let handle = io::stdout().lock();
    let mut buf = BufWriter::with_capacity(crate::DEFAULT_BUFFER_SIZE, handle);

    for s in args.strings {
        write!(buf, "{} ", s).context("バッファの書き込みに失敗しました。")?;
    }
    write!(buf, "\n").context("バッファの書き込みに失敗しました。")?;

    buf.flush().context("標準入力のフラッシュ失敗しました。")?;

    Ok(())
}
