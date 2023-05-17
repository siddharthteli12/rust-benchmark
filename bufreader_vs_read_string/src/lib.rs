use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Error},
};

// Find pattern in file using buffer read with read line.
fn buffer(pattern: String, path: String) -> Result<Vec<(usize, String)>, Error> {
    let mut reader = BufReader::new(File::open(path)?);
    let mut content = String::new();
    let mut result_vec: Vec<(usize, String)> = vec![];
    let mut counter = 0;
    while reader.read_line(&mut content)? != 0_usize {
        counter = counter + 1;
        if content.is_empty() {
            break;
        }
        if content.contains(&pattern) {
            result_vec.push((counter, content.clone()));
        }
    }
    Ok(result_vec)
}

// Find pattern in file using read to string at once.
fn read_to_string(pattern: String, path: String) -> Result<Vec<(usize, String)>, Error> {
    let content = fs::read_to_string(path)?;
    let mut result_vec: Vec<(usize, String)> = vec![];

    for (counter, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            result_vec.push((counter, line.to_string()));
        }
    }
    Ok(result_vec)
}
