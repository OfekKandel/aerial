use std::{fs, io, path::PathBuf};

pub fn list_dir(directorh_path: PathBuf) -> Result<(), io::Error> {
    for entry in fs::read_dir(directorh_path)? {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }
    Ok(())
}

pub fn read_file_plaintext(file_path: PathBuf) -> Result<(), io::Error> {
    let s = fs::read_to_string(&file_path)?;
    print!("{}", s);
    Ok(())
}
