use dialoguer::{Select, Input};
use std::path::Path;
use std::process::{Command, Stdio};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let choices = vec![
        "📷 Create gif",
        "🔇 Remove sound",
    ];
    let selection = Select::new().items(&choices).interact()?;
    match selection {
        0_usize => create_gif(),
        1_usize => remove_sound(),
        _ => Err("❌ Incorrect selection ...".into()),
    }
}

fn obtain_file_path() -> Result<String, Box<dyn std::error::Error>> {
    let input = Input::<String>::new()
        .with_prompt("📄 input movie file path")
        .interact()?;
    let input = _remove_head_and_tail_double_quotation(&input);
    let path = Path::new(&input);
    if path.exists() & path.is_file() {
        Ok(input)
    } else {
        Err(format!("❌ File not exist -> {:?}", path).into())
    }
}

fn _remove_head_and_tail_double_quotation(arg: &String) -> String {
    let mut result = arg.trim().to_string();
    if result.starts_with("\"") {
        result.remove(0);
    }
    if result.ends_with("\"") {
        result.pop();
    }
    result
}

fn obtain_width() -> Result<usize, Box<dyn std::error::Error>> {
    let width = Input::<String>::new()
        .with_prompt("↔ Input gif width (default : 1280)")
        .interact()?;
    let width: usize = width.parse().unwrap_or_else(|_| {
        const DEFAULT_VALUE: usize = 1280;
        DEFAULT_VALUE
    });
    Ok(width)
}

fn create_gif() -> Result<(), Box<dyn std::error::Error>> {
    let src = obtain_file_path()?;
    let src = Path::new(&src);
    let file_stem = src.file_stem().unwrap().to_string_lossy().to_string();
    let width = obtain_width().unwrap();
    let dst = src.with_file_name(format!("{}_{}.gif", file_stem, width));
    println!("{:?}", dst);

    Command::new("ffmpeg")  // TODO: 240411 ffmpeg にパスが通ってない場合もあるので、.ps1 を実行するようにする。
        .args([
            "-i",
            src.to_str().unwrap(),
            "-vf",
            &format!("scale={}:-1", width),
            "-r",
            "10",
            dst.to_str().unwrap(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("\n\nFailed to execute command\n\n");

    Ok(())
}

fn remove_sound() -> Result<(), Box<dyn std::error::Error>> {
    let movie_path = obtain_file_path()?;
    // TODO: 240411 音声除去機能を追加する
    Ok(())
}