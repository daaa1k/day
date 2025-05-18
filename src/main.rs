use chrono::{Datelike, Duration, Local, NaiveDate};
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
    process::Command,
};

fn get_date_string(offset_days: i64) -> String {
    (Local::now().date_naive() + Duration::days(offset_days))
        .format("%Y-%m-%d")
        .to_string()
}

fn create_note_if_missing(
    file_path: &PathBuf,
    today: &str,
    yesterday: &str,
    tomorrow: &str,
) -> std::io::Result<()> {
    if !file_path.exists() {
        println!("File does not exist, creating new daily note.");
        let mut file = File::create(file_path)?;
        writeln!(
            file,
            "# {today}\n\n[[{yesterday}]] - [[{tomorrow}]]\n\n## Tracker\n  - [ ] \n\n## Log\n",
        )?;
    }
    Ok(())
}

fn open_editor(editor: &str, file_path: &PathBuf) -> std::io::Result<()> {
    #[cfg(not(test))]
    {
        Command::new(editor).arg(file_path).status().map(|_| ())
    }
    #[cfg(test)]
    Result::Ok(())
}

pub fn run_day_command() -> std::io::Result<()> {
    let zettelkasten =
        env::var("ZETTELKASTEN").expect("ZETTELKASTEN environment variable is not set.");
    let editor = env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

    let today = get_date_string(0);
    let tomorrow = get_date_string(1);
    let yesterday = get_date_string(-1);

    let file_path = PathBuf::from(format!(
        "{}/periodic-notes/daily-notes/{}.md",
        zettelkasten, today
    ));

    env::set_current_dir(&zettelkasten)?;

    create_note_if_missing(&file_path, &today, &yesterday, &tomorrow)?;
    open_editor(&editor, &file_path)?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    run_day_command()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_create_note_if_missing_creates_file_with_correct_content() {
        let dir = tempdir().unwrap();
        let test_path = dir.path().join("2025-01-01.md");

        let today = "2025-01-01";
        let yesterday = "2024-12-31";
        let tomorrow = "2025-01-02";

        create_note_if_missing(&test_path, today, yesterday, tomorrow).unwrap();

        let content = fs::read_to_string(&test_path).unwrap();
        assert!(content.contains("# 2025-01-01"));
        assert!(content.contains("[[2024-12-31]] - [[2025-01-02]]"));
        assert!(content.contains("## Tracker"));
        assert!(content.contains("## Log"));
    }

    #[test]
    fn test_get_date_string_works() {
        // Mocked date logic — you can enhance this with a fixed time using Chrono's TimeZone
        let today = Local::now().date_naive();
        let plus_one = today + Duration::days(1);
        let minus_one = today - Duration::days(1);

        assert_eq!(get_date_string(0), today.format("%Y-%m-%d").to_string());
        assert_eq!(get_date_string(1), plus_one.format("%Y-%m-%d").to_string());
        assert_eq!(
            get_date_string(-1),
            minus_one.format("%Y-%m-%d").to_string()
        );
    }
}
