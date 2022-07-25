mod migrate;
mod utils;

use std::error::Error;
use std::fmt::Formatter;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::ops::Index;
use std::path::{Path, PathBuf};
use std::{env, fs};

//Timestamp, Price
#[derive(Debug)]
struct ParsedRecord(u64, u64);

impl std::fmt::Display for ParsedRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(0, {}, {})", self.1, self.0)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    if !env::current_dir()?.ends_with("tools/btc-price-parser") {
        env::set_current_dir("./tools/btc-price-parser").expect("cannot set cwd");
    }

    fs::create_dir_all("data").expect("cannot create dist dir");
    fs::create_dir_all("dist").expect("cannot create dist dir");

    let mut all_sql_file = File::create("./dist/all.sql")?;

    add_file_header(&mut all_sql_file)?;

    for entry in fs::read_dir("data").expect("Unable to list") {
        let entry = entry.expect("Unable to get entry");

        process_file(entry.path(), &mut all_sql_file)?;

        println!(
            "Processed file {}",
            entry.path().file_name().unwrap().to_str().unwrap()
        );
    }

    replace_comma_with_semicolon(&mut all_sql_file)?;

    if let Some(_) = env::args().find(|it| it == "migrate") {
        migrate::migrate()?;
        println!("Migrated all.sql");
    }

    Ok(())
}

fn process_file<P: AsRef<Path>>(path: P, all_sql_file: &mut File) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();

    let mut sql_file = File::create(&get_path_new_file(path))?;

    add_file_header(&mut sql_file)?;

    let mut is_first = true;

    for line in read_csv_file(path)?.lines().map(|line| line.unwrap()) {
        let result: Vec<&str> = line.split(",").collect();

        let timestamp = match result.index(0).parse::<u64>() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let price = match result.index(6).parse::<f64>() {
            Ok(v) => v.round() as u64,
            Err(_) => continue,
        };

        let row = ParsedRecord(timestamp, price);

        if is_first {
            write!(sql_file, "  {}", row)?;
        } else {
            write!(sql_file, ",\n  {}", row)?;
        }

        write!(all_sql_file, "  {},\n", row)?;

        is_first = false;
    }

    write!(sql_file, ";")?;

    Ok(())
}

fn read_csv_file<P: AsRef<Path>>(path: P) -> Result<BufReader<File>, Box<dyn Error>> {
    let file = File::open(path)?;

    Ok(BufReader::new(file))
}

fn get_file_name(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("csv", "sql")
}

fn get_path_new_file(path: &Path) -> PathBuf {
    let name_file = get_file_name(path.as_ref());
    let path_result = Path::new("dist");
    path_result.join(name_file)
}

fn add_file_header(file: &mut File) -> std::io::Result<()> {
    write!(
        file,
        "INSERT INTO\n  public.price(ticker, value, \"timestamp\")\nVALUES\n"
    )
}

fn replace_comma_with_semicolon(file: &mut File) -> Result<(), Box<dyn Error>> {
    let size = utils::imp::get_file_size(&file)?;
    if size > 2 {
        file.set_len(size - 2)?;
        file.seek(SeekFrom::End(0))?;
    }
    write!(file, ";")?;
    Ok(())
}
