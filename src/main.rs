use std::{
    env,
    fs::{create_dir_all, read_to_string, File},
    io::{Read, Write},
    path::Path,
};

enum Mode {
    /// Default; only selected before another mode is chosen.
    Def,

    /// Add job.
    Add,

    /// Remove job.
    Rem,
}

fn main() -> Result<(), String> {
    // Generate path: "...\AppData\Roaming\jodo\todo.txt"
    let path = env::var("APPDATA").map_err(|_| "APPDATA is missing.")? + "\\jodo\\todo.txt";
    let path = Path::new(&path);

    // If no commands are given, just print the file
    if env::args().count() == 1 {
        read_to_string(path)
            .map_err(|_| "Failed to open file.")?
            .lines()
            .enumerate()
            .for_each(|(i, s)| println!("{i}: {s}"));
    }

    // Run through commands
    let mut jobs = read_file_lines(path)?;
    let mut mode = Mode::Def;
    let mut adds = Vec::new();
    let mut rems = Vec::new();

    for cmd in env::args().skip(1) {
        match cmd.as_str() {
            "-a" => mode = Mode::Add,
            "-r" => mode = Mode::Rem,
            _ => match mode {
                Mode::Add => adds.push(cmd),
                Mode::Rem => rems.push(
                    cmd.parse::<usize>()
                        .map_err(|_| format!("Failed to parse item '{}' into an index.", cmd))?,
                ),
                Mode::Def => return Err(format!("Expected mode ('-a' or '-r'), found '{}'", cmd)),
            },
        }
    }

    // Perform removals
    rems.sort_unstable_by(|a, b| b.cmp(a));

    for i in rems {
        jobs.remove(i);
    }

    // Perform additions
    for s in adds {
        jobs.push(s);
    }

    // Done; write new lines
    write_file_lines(path, jobs)
}

/// Returns a vector containing the file's content separated into lines.
///
/// If the path to the file doesn't exist, creates it.
///
/// If the file itself doesn't exist, creates it.
fn read_file_lines(path: &Path) -> Result<Vec<String>, String> {
    // Ensure the path to the file exists
    create_dir_all(path.parent().unwrap()).map_err(|_| "Failed to create file directory.")?;

    // Open the file, creating it if it doesn't already exist
    let mut file = File::options()
        .create(true)
        .read(true)
        .write(true)
        .open(path)
        .map_err(|_| "Failed to create/open file.")?;

    // Read file content into a string
    let mut cont = String::new();
    file.read_to_string(&mut cont)
        .map_err(|_| "Failed to read from file.")?;

    Ok(cont.lines().map(|x| x.to_string()).collect())
}

/// Writes the vector into the file with each line being separated by a newline.
fn write_file_lines(path: &Path, lines: Vec<String>) -> Result<(), String> {
    let mut file = File::options()
        .write(true)
        .truncate(true)
        .open(path)
        .map_err(|_| "Failed to open file.")?;

    write!(file, "{}", lines.join("\n")).map_err(|_| "Failed to write to file.")?;

    Ok(())
}
