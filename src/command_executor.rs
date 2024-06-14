use std::{fs::{self, File}, io::{self, Write}, path::Path, process::Command};

pub fn write_script<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


pub fn make_executable(script_path: &str){
     // Make the script executable
     Command::new("chmod")
     .arg("+x")
     .arg(script_path)
     .status()
     .expect("Failed to make script executable");

}


pub fn execute_script<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let output = Command::new("sh")
        .arg(path.as_ref())
        .output()
        .expect("Failed to execute script");
    let _ = fs::remove_file(path).unwrap();
    if output.status.success() {
        io::stdout().write_all(&output.stdout)?;
        Ok(())
    } else {
        io::stderr().write_all(&output.stderr)?;
        Err(io::Error::new(io::ErrorKind::Other, "Script execution failed"))
    }
}