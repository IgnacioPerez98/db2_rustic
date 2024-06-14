use crate::command_executor::{execute_script, make_executable, write_script};


pub fn make_full_backup(){
    let script = r#"
        #!/bin/sh
        db2 connect to bbdddos user db2ucu using penarolcds1891
        db2 "BACKUP DATABASE bbdddos TO /home/db2/Desktop"
        db2 disconnect bbdddos
    "#;
    let script_path = "/tmp/db2_full_backup_script.sh";
    write_script(script_path, &script).unwrap();
    make_executable(&script_path);
    execute_script(script_path).expect("Error making full backup");
}