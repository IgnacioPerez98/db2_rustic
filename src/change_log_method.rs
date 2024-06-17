use crate::command_executor::run_command;

#[allow(dead_code)]
pub fn use_circular_log(){
    let script = format!(r#"
    #!/bin/sh
        
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