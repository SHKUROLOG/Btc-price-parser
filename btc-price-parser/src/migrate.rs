use postgres::{Client, NoTls};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Read};

pub fn migrate() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect(
        "host=localhost port=5432 user=user password=pass dbname=moon",
        NoTls,
    )?;

    let file = File::open("./dist/all.sql")?;
    let mut query = String::new();
    BufReader::new(file).read_to_string(&mut query)?;

    client.query(&query, &[])?;

    Ok(())
}
