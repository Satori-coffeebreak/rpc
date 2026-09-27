mod echo;

use anyhow::{Result, bail};
use std::{env, path::Path};

use crate::echo::echo;

fn main() -> Result<()> {
    let cmd_path = env::args().next().unwrap();

    let cmd = Path::new(&cmd_path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap();

    match cmd {
        "echo" => echo(),
        _ => bail!("存在しないコマンドです。"),
    }
}
