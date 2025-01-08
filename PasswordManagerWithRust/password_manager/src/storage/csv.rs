use std::fs::File;
use std::io::{self, BufReader, BufWriter};
use csv::{Reader, Writer};
use crate::models::PasswordEntry;

pub struct CsvStorage;

impl CsvStorage {
    pub fn import(path: &str) -> io::Result<Vec<PasswordEntry>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut csv_reader = Reader::from_reader(reader);
        let mut entries = Vec::new();

        for result in csv_reader.records() {
            let record = result?;
            if let Ok(entry) = PasswordEntry::new(
                record.get(0).unwrap_or_default().to_string(),
                record.get(1).unwrap_or_default().to_string(),
                record.get(2).unwrap_or_default().to_string(),
            ) {
                entries.push(entry);
            }
        }

        Ok(entries)
    }

    pub fn export(path: &str, entries: &[PasswordEntry]) -> io::Result<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        let mut csv_writer = Writer::from_writer(writer);

        for entry in entries {
            csv_writer.write_record(&[
                entry.username(),
                entry.password(),
                entry.place(),
            ])?;
        }

        csv_writer.flush()?;
        Ok(())
    }
}