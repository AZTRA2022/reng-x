use std::{fmt::Debug, io::Read, str::FromStr};

use anyhow::Error;

use super::*;

#[allow(unused)]
fn parse_txt<T>(filepath: &str, output_vec: &mut Vec<T>) -> Result<(), Error>
where
    T: Copy + FromStr + Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut file_content = String::new();
    let mut file = File::open(filepath).unwrap_or_else(|error| {
        panic!(
            "Failed to open the file {} , exiting with error: {}",
            filepath, error
        );
    });
    file.read_to_string(&mut file_content)
        .unwrap_or_else(|error| panic!("Failed to read the file , exiting with error: {}", error));
    let mut token = String::new();
    for c in file_content.trim().chars() {
        if c != ',' && c != ';' && c != ':' {
            token.push(c);
        } else {
            let parsed = token
                .parse::<T>()
                .expect(&format!("Failed to parse {:?}", token));
            output_vec.push(parsed);
            token.clear();
        }
    }
    if !token.is_empty() {
        let parsed = token
            .parse::<T>()
            .expect(&format!("Failed to parse {:?}", token));
        output_vec.push(parsed);
    }
    Ok(())
}

fn parse_csv<T>(filepath: &str, output_vec: &mut Vec<T>, field: i32) -> Result<(), Error>
where
    T: FromStr + Debug + Copy,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let file = File::open(filepath)?;
    let mut rdr = csv::Reader::from_reader(file);

    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
        let parsed_record = record[field as usize].parse::<T>().expect(&format!(
            "Failed to parse record[{}] = {} ",
            field,
            record[field as usize].to_string()
        ));
        output_vec.push(parsed_record);
    }
    Ok(())
}

pub fn read_from<T>(file: &FileExt) -> Vec<T>
where
    T: Copy + FromStr + Debug,
    <T as FromStr>::Err: std::fmt::Debug,
{
    let mut output_vec: Vec<T> = Vec::new();
    match file {
        FileExt::CSV(filepath, field) => {
            if !filepath.ends_with(".csv") {
                panic!("File is not a .csv file")
            }
            _ = parse_csv::<T>(filepath, &mut output_vec, *field).unwrap();
        }
        FileExt::TXT(filepath) => {
            if !filepath.ends_with(".txt") {
                panic!("File is not a .txt file")
            }
            _ = parse_txt::<T>(filepath, &mut output_vec).unwrap();
        }
    }
    output_vec
}
