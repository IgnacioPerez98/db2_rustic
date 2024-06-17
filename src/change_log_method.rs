use crate::command_executor::run_command;

#[allow(dead_code)]
pub fn use_circular_log(){
    let script = format!(r#"
        #!/bin/sh

        db2 connect to bbdd2
        db2 update db cfg for bbdd2 using NEWLOGPATH /mnt/7cb78399-e05c-4e96-bee8-19f93b3f1760/Logs/Log1
        db2 update db cfg for bbdd2 using FAILARCHPATH /mnt/7cb78399-e05c-4e96-bee8-19f93b3f1760/Logs/LogFail

        db2stop
        db2start
    "#);
    let script_path = "/tmp/db2_circular_log_script.sh";
    run_command(&script, &script_path);

}


#[allow(dead_code)]
pub fn use_archived_log() {
    let script = format!(r#"
    #!/bin/sh
    
    "#);
    let script_path = "/tmp/db2_archive_log_script.sh";
    run_command(&script, &script_path);


}