use std::{fs::{self, File}, io::{self, Write}, path::Path, process::Command};
///Escribe el script en un path dado.
fn write_script<P: AsRef<Path>>(path: P, content: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

///Asigna permisos a el script creado
fn make_executable(script_path: &str){
     // Make the script executable
     Command::new("chmod")
     .arg("+x")
     .arg(script_path)
     .status()
     .expect("Failed to make script executable");

}

///Ejecuta el script y redirecciona el std output
fn execute_script<P: AsRef<Path>>(path: P) -> io::Result<()> {
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

pub fn run_command(script :&str, script_path: &str){
    println!("Ejecutar comando: \n {} \n",script);
    write_script(script_path, &script).unwrap();
    make_executable(&script_path);
    execute_script(script_path).expect("Error making full backup");
}