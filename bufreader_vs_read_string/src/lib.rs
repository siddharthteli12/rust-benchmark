use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Error},
};

#[cfg(test)]
mod tests;

// Find pattern in file using buffer read with read line.
pub fn buffer(pattern: String, path: String) -> Result<Vec<(usize, String)>, Error> {
    let reader = BufReader::new(File::open(path)?);
    let mut result_vec: Vec<(usize, String)> = vec![];

    for (counter, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(&pattern) {
            result_vec.push((counter, line));
        }
    }
    Ok(result_vec)
}

// Find pattern in file using read to string at once.
pub fn read_to_string(pattern: String, path: String) -> Result<Vec<(usize, String)>, Error> {
    let content = fs::read_to_string(path)?;
    let mut result_vec: Vec<(usize, String)> = vec![];

    for (counter, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            result_vec.push((counter, line.to_string()));
        }
    }
    Ok(result_vec)
}
