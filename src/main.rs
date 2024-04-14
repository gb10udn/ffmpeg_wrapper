use dialoguer::{Select, Input};
use std::path::Path;
use std::process::{Command, Stdio};
use std::fs;
use std::io;
use reqwest::blocking::Client;
use zip::read::ZipArchive;


fn main() {
    const FFMPEG_PATH: &str = "./misc/ffmpeg.exe";
    download_ffmpeg(FFMPEG_PATH).expect("Fail to setup ffmpeg");
    
    let choices = vec![
        "gif",
        "mute",
        "compress",
    ];
    let selection = Select::new().items(&choices).interact().unwrap();
    match selection {
        0_usize => create_gif(FFMPEG_PATH).unwrap(),
        1_usize => remove_sound(FFMPEG_PATH).unwrap(),
        2_usize => compress(FFMPEG_PATH).unwrap(),
        _ => println!("Error: Incorrect selection ..."),
    }
    stop();
}

fn stop() {
    println!("");
    println!("finished !!!");
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("");
    }
}

fn download_ffmpeg(dst: &str) -> Result<(), Box<dyn std::error::Error>> {
    const FFMPEG_URL: &str = "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-win64-gpl.zip";
    if !Path::new(dst).exists() {
        println!("Start to download ffmpeg ...");
        let temp_dir = Path::new("./temp_ffmpeg");
        fs::create_dir_all(temp_dir)?;

        let client = Client::new();
        let mut response = client.get(FFMPEG_URL).send()?;
        if response.status().is_success() {
            let mut dest = fs::File::create(temp_dir.join("ffmpeg.zip"))?;
            response.copy_to(&mut dest)?;

            let zip_path = temp_dir.join("ffmpeg.zip");
            let target_dir = temp_dir.join("ffmpeg");
            let mut archive = ZipArchive::new(fs::File::open(zip_path)?)?;
            archive.extract(&target_dir)?;

            let src_path = target_dir.join("ffmpeg-master-latest-win64-gpl/bin/ffmpeg.exe");

            if let Some(dst_dir) = Path::new(dst).parent() {
                fs::create_dir_all(dst_dir).unwrap();
            }

            fs::copy(src_path, dst)?;
            fs::remove_dir_all(temp_dir)?;
        } else {
            println!("Failed to download FFmpeg.");
        }
    }
    Ok(())
}

fn obtain_file_path() -> Result<String, Box<dyn std::error::Error>> {
    let input = Input::<String>::new()
        .with_prompt("Input movie file path")
        .interact()?;
    let input = _remove_head_and_tail_double_quotation(&input);
    let path = Path::new(&input);
    if path.exists() & path.is_file() {
        Ok(input)
    } else {
        Err(format!("Error: File not exist -> {:?}", path).into())
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
        .with_prompt("Input gif width (Ex. 1280)")
        .interact()?;
    let width: usize = width.parse().unwrap_or_else(|_| {
        const DEFAULT_VALUE: usize = 1280;
        DEFAULT_VALUE
    });
    Ok(width)
}

fn obtain_dst_path(src: &String, suffix: &str, extension: &str) -> String {
    let src = Path::new(&src);
    let file_stem = src.file_stem().unwrap().to_string_lossy().to_string();
    let dst = src.with_file_name(format!("{}_{}.{}", file_stem, suffix, extension));  // TODO: 240414 既に存在する場合に panic! してもいいかも？
    dst.to_string_lossy().to_string()
}

fn create_gif(ffmpeg_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src = obtain_file_path()?;
    let width = obtain_width().unwrap();
    let dst = obtain_dst_path(&src, &width.to_string(), "gif");

    Command::new(ffmpeg_path)
        .args([
            "-i",
            &src,
            "-vf",
            &format!("scale={}:-1", width),
            "-r",
            "10",
            &dst,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("\n\nFailed to execute command\n\n");
    Ok(())
}

fn remove_sound(ffmpeg_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src = obtain_file_path()?;
    let dst = obtain_dst_path(&src, "mute", "mp4");
    Command::new(ffmpeg_path)
        .args([
            "-i",
           &src,
           "-vcodec",
           "copy",
           "-an",
           &dst,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("\n\nFailed to execute command\n\n");
    Ok(())
}

fn compress(ffmpeg_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src = obtain_file_path().unwrap();
    let compress_param = obtain_compress_parameter().unwrap();
    let dst = obtain_dst_path(&src, &format!("compress={}", compress_param), "mp4");
    Command::new(ffmpeg_path)
        .args([
            "-i",
           &src,
           "-crf",
           &compress_param.to_string(),
           &dst,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("\n\nFailed to execute command\n\n");
    Ok(())
}

fn obtain_compress_parameter() -> Result<usize, Box<dyn std::error::Error>> {
    let width = Input::<String>::new()
        .with_prompt("Input compress parameter (Ex. 30)")
        .interact()?;
    let width: usize = width.parse().unwrap_or_else(|_| {
        const DEFAULT_VALUE: usize = 30;
        DEFAULT_VALUE
    });
    Ok(width)
}