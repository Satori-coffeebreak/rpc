mod echo;
mod pwd;

use anyhow::{Result, bail};
use std::{env, path::Path};

// 64KB
const DEFAULT_BUFFER_SIZE: usize = 64 * 1024;

fn main() -> Result<()> {
    let cmd_path = env::args().next().unwrap();

    let cmd = Path::new(&cmd_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    match cmd {
        "echo" => echo::main(),
        "pwd" => pwd::main(),
        _ => bail!("存在しないコマンドです。"),
    }
}
