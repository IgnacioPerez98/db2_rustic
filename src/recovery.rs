use crate::command_executor::run_command;

///Recupera la base de datos a su estado original
#[allow(dead_code)]
pub fn perfom_recovery(ruta:&str, timestamp:&str) {

    let script = format!(r#"
        #!/bin/sh
        db2 terminate
        db2 deactivate db bbdd2
        db2 restore db TEMADB from {route} taken at {period_mark} replace existing
        db2 rollforward database bbdd2 to end of logs and stop overflow log path (/var/archive_logs)
    "#, route= ruta, period_mark = timestamp);
    let script_path = "/tmp/db2_perform_recovery_script.sh";
    run_command(&script, script_path);    



}