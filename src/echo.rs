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

pub fn echo() -> Result<()> {
    let error_messages = [
        "バッファの書き込みに失敗しました。",
        "標準入力のフラッシュ失敗しました。",
    ];

    let args = Args::parse();

    let handle = io::stdout().lock();
    let mut buf = BufWriter::with_capacity(crate::DEFAULT_BUFFER_SIZE, handle);

    for s in args.strings {
        write!(buf, "{} ", s).context(error_messages[0])?;
    }
    write!(buf, "\n").context(error_messages[0])?;

    buf.flush().context(error_messages[1])?;

    Ok(())
}
