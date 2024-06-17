use crate::command_executor::run_command;

///Recupera la base de datos a su estado original
#[allow(dead_code)]
pub fn perfom_recovery(timestamp:&str,folder:&str) {

    let script = format!(r#"
        #!/bin/sh
        db2 terminate
        db2 deactivate db bbdd2
        db2 restore db bbdd2 from {folder} taken at {time}
        db2 activate db bbdd2
    "#, time= timestamp);
    let script_path = "/tmp/db2_perform_recovery_script.sh";
    run_command(&script, script_path);    
}