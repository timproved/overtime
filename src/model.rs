use anyhow::{bail, Context, Result};
use chrono::{Datelike, NaiveDate};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OvertimeEntry {
    pub date: NaiveDate,
    pub minutes: i32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct OvertimeData {
    pub entries: Vec<OvertimeEntry>,
}

impl OvertimeData {
    fn data_file_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("com", "overtime", "overtime")
            .context("Failed to determine project directories")?;

        let data_dir = proj_dirs.data_dir();
        fs::create_dir_all(data_dir).context("Failed to create data directory")?;

        Ok(data_dir.join("overtime.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::data_file_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path).context("Failed to read overtime data file")?;

        if content.trim().is_empty() {
            return Ok(Self::default());
        }

        serde_json::from_str(&content).context("Failed to parse overtime data file")
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::data_file_path()?;
        let content =
            serde_json::to_string_pretty(self).context("Failed to serialize overtime data")?;

        fs::write(&path, content).context("Failed to write overtime data file")?;

        Ok(())
    }

    pub fn add_overtime(&mut self, date: NaiveDate, minutes: i32) -> Result<()> {
        // Check if we already have an entry for this date
        for entry in &mut self.entries {
            if entry.date == date {
                entry.minutes += minutes;
                return Ok(());
            }
        }

        // If not, add a new entry
        self.entries.push(OvertimeEntry { date, minutes });
        self.save()?;
        Ok(())
    }

    pub fn remove_overtime(&mut self, date: NaiveDate, minutes: i32) -> Result<()> {
        // Check if we have an entry for this date
        for entry in &mut self.entries {
            if entry.date == date {
                entry.minutes -= minutes;
                self.save()?;
                return Ok(());
            }
        }

        bail!("No overtime entry found for the specified date")
    }

    pub fn list_overtime(&self) -> Result<()> {
        if self.entries.is_empty() {
            println!("No overtime entries found.");
            return Ok(());
        }

        let mut sorted_entries = self.entries.clone();
        sorted_entries.sort_by(|a, b| a.date.cmp(&b.date));

        let mut grouped: HashMap<(i32, u32), Vec<&OvertimeEntry>> = HashMap::new();
        for entry in &sorted_entries {
            grouped
                .entry((entry.date.year(), entry.date.month()))
                .or_default()
                .push(entry);
        }

        let mut total_minutes = 0;

        for ((year, month), entries) in grouped.iter() {
            println!("\n{}-{}:", year, month);
            println!("Date       | Overtime");
            println!("-----------+---------");

            for entry in entries {
                let hours = entry.minutes.abs() / 60;
                let mins = entry.minutes.abs() % 60;
                let sign = if entry.minutes < 0 { "-" } else { " " };
                println!(
                    "{} | {}{}h {}m",
                    entry.date.format("%d.%m.%Y"),
                    sign,
                    hours,
                    mins
                );

                total_minutes += entry.minutes;
            }
        }

        // Print summary
        let total_hours = total_minutes.abs() / 60;
        let total_mins = total_minutes.abs() % 60;
        let sign = if total_minutes < 0 { "-" } else { "" };

        println!(
            "\nTotal overtime: {}{} hours and {} minutes",
            sign, total_hours, total_mins
        );

        Ok(())
    }
}

pub fn parse_time(time_str: &str) -> Result<i32> {
    if time_str.is_empty() {
        bail!("Time string cannot be empty");
    }

    let mut total_minutes = 0;
    let mut current_number = String::new();

    for c in time_str.chars() {
        if c.is_digit(10) {
            current_number.push(c);
        } else if c == 'h' || c == 'H' {
            if current_number.is_empty() {
                bail!("Invalid time format: missing number before 'h'");
            }

            let hours: i32 = current_number.parse().context("Failed to parse hours")?;
            total_minutes += hours * 60;
            current_number.clear();
        } else if c == 'm' || c == 'M' {
            if current_number.is_empty() {
                bail!("Invalid time format: missing number before 'm'");
            }

            let minutes: i32 = current_number.parse().context("Failed to parse minutes")?;
            total_minutes += minutes;
            current_number.clear();
        } else {
            bail!("Invalid character in time string: {}", c);
        }
    }

    // Default to minuteas
    if !current_number.is_empty() {
        let minutes: i32 = current_number.parse().context("Failed to parse minutes")?;
        total_minutes += minutes;
    }

    if total_minutes == 0 {
        bail!("Time must be greater than zero");
    }

    Ok(total_minutes)
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate> {
    // Try to parse in the format ddmmYYYY
    if date_str.len() == 8 {
        let day: u32 = date_str[0..2].parse().context("Failed to parse day")?;
        let month: u32 = date_str[2..4].parse().context("Failed to parse month")?;
        let year: i32 = date_str[4..8].parse().context("Failed to parse year")?;

        return NaiveDate::from_ymd_opt(year, month, day).context("Invalid date components");
    }

    // Alternative format: dd.mm.YYYY
    NaiveDate::parse_from_str(date_str, "%d.%m.%Y")
        .context("Failed to parse date, expected format: ddmmYYYY or dd.mm.YYYY")
}
