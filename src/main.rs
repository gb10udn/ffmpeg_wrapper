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
        2_usize => compress().unwrap(),
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

fn create_gif(ffmpeg_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src = obtain_file_path()?;
    let src = Path::new(&src);
    let file_stem = src.file_stem().unwrap().to_string_lossy().to_string();
    let width = obtain_width().unwrap();
    let dst = src.with_file_name(format!("{}_{}.gif", file_stem, width));
    println!("{:?}", dst);

    Command::new(ffmpeg_path)
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

fn remove_sound(ffmpeg_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let src = obtain_file_path()?;
    let src = Path::new(&src);
    let file_stem = src.file_stem().unwrap().to_string_lossy().to_string();
    let dst = src.with_file_name(format!("{}_mute.mp4", file_stem));
    Command::new(ffmpeg_path)
        .args([
            "-i",
           src.to_str().unwrap(),
           "-vcodec",
           "copy",
           "-an",
           dst.to_str().unwrap(),
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("\n\nFailed to execute command\n\n");
    Ok(())
}

fn compress() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: 240411 動画ファイルを圧縮する機能を付ける。
    Ok(())
}