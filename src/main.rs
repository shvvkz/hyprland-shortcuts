use std::fs::File;
use std::io::{self, BufRead};
use std::os::unix::process;
use std::path::Path;

fn main() -> io::Result<()> {
    let path = "./hyprland.conf";
    let commands = process_file(path)?;
    let processable_commands = commands_cut_down(commands);
    commands_to_json_file_with_translation(processable_commands, "./commands.json")?;
    Ok(())
}

fn process_file<P>(path: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let mut commands = Vec::new();
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(content) = line {
                let command_line = process_line(&content);
                if command_line.is_some() {
                    let command = command_line.unwrap();
                    commands.push(command);
                }
            }
        }
    }
    Ok(commands)
}

fn process_line(line: &str) -> Option<String> {
    if line.contains("bind =") {
        let line = if let Some(pos) = line.find('#') {
            &line[..pos]
        } else {
            line
        };
        let line = line.replace("bind =", "").trim().to_string();
        Some(line)
    } else {
        None
    }
}

// Helper function to read lines from a file
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn commands_cut_down(commands: Vec<String>) -> Vec<Vec<String>> {
    let mut processable_commands: Vec<Vec<String>> = Vec::new();
    for command in commands {
        let mut processable_command: Vec<String> = Vec::new();
        let parts: Vec<&str> = command.split(',').collect();
        for part in parts {
            processable_command.push(part.trim().to_string());
        }
        processable_command.retain(|part| !part.is_empty());
        processable_commands.push(processable_command);
    }
    processable_commands
}

fn translate_commands(commands: Vec<Vec<String>>, dictionary: &std::collections::HashMap<String, String>) -> Vec<Vec<String>> {
    let mut translated_commands: Vec<Vec<String>> = Vec::new();
    for command in commands {
        let mut translated_command: Vec<String> = Vec::new();
        for part in command {
            if let Some(translation) = dictionary.get(&part) {
                translated_command.push(translation.clone());
            } else {
                translated_command.push(part.clone());
            }
        }
        translated_commands.push(translated_command);
    }
    translated_commands
}
fn commands_to_json_file_with_translation(
    commands: Vec<Vec<String>>,
    dictionary_path: &str,
) -> io::Result<()> {
    // Load the dictionary from the JSON file
    let dictionary_file = File::open(dictionary_path)?;
    let dictionary: std::collections::HashMap<String, String> =
        serde_json::from_reader(dictionary_file)?;

    // Translate the commands using the dictionary
    let commands: Vec<Vec<String>> = commands
        .into_iter()
        .map(|command| {
            command
                .into_iter()
                .filter(|part| part != "exec")
                .collect()
        })
        .collect();
    let translated_commands = translate_commands(commands, &dictionary);
    for c in translated_commands {
        println!("{:?}", c);
    }
    Ok(())
}
