use std::{
    fs::File,
    io::{BufReader, Result},
    path::Path,
};

/// Create a `File` `BufReader` from a `Path`. Otherwise throw IO Error.
pub fn read_file<P>(path: P) -> Result<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}
