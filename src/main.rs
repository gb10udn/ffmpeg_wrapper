use dialoguer::{Select, Input};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let choices = vec![
        "Create gif",
        "Remove sound",
    ];
    let selection = Select::new().items(&choices).interact()?;
    match selection {
        0_usize => create_gif(),
        1_usize => remove_sound(),
        _ => Err("".into()),
    }
}

fn obtain_file_path() -> Result<String, Box<dyn std::error::Error>> {
    let input = Input::<String>::new()
        .with_prompt("input movie file path")
        .interact()?;
    Ok(input)
}

fn create_gif() -> Result<(), Box<dyn std::error::Error>> {
    let move_path = obtain_file_path()?;
    // TODO: 240411 gif 作成機能を追加する
    Ok(())
}

fn remove_sound() -> Result<(), Box<dyn std::error::Error>> {
    let move_path = obtain_file_path()?;
    // TODO: 240411 音声除去機能を追加する
    Ok(())
}