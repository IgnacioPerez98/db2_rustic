use crate::command_executor::run_command;

pub fn full_backup(reference :&str){
    let script = format!(r#"
    #!/bin/sh
    db2 terminate
    db2 deactivate db bbdd2
    
    db2 connect to bbdd2
    db2 quiesce database immediate force connections
    db2 connect reset
    
    db2 backup database bbdd2 to {route}
    
    db2 connect to bbdd2
    db2 unquiesce database
    db2 connect reset
    
    db2 activate db bbdd2
    
    "#, route = reference);
    let script_path = "/tmp/db2_full_backup_script.sh";
    run_command(&script, script_path);    
}




pub fn incremental_backup(reference :&str){

    let script =  format!(r#"
        #!/bin/sh
        db2 terminate
        db2 deactivate db bbdd2
    
        db2 connect to bbdd2
        db2 quiesce database immediate force connections
        db2 connect reset

        db2 "BACKUP DATABASE bbdd2 INCREMENTAL TO {route}"
    
        db2 connect to bbdd2
        db2 unquiesce database
        db2 connect reset

        db2 activate db bbdd2
    "#, route = reference);
    let script_path = "/tmp/db2_incremental_backup_script.sh";
    run_command(&script, script_path);    
}




pub fn delta_backup(reference :&str){

    let script = format!(r#"
        #!/bin/sh
        db2 terminate
        db2 deactivate db bbdd2
    
        db2 connect to bbdd2
        db2 quiesce database immediate force connections
        db2 connect reset

        db2 "BACKUP DATABASE bbdd2 INCREMENTAL DELTA TO {route}"
    
        db2 connect to bbdd2
        db2 unquiesce database
        db2 connect reset
        
        db2 activate db bbdd2
    "#, route = reference);
    let script_path = "/tmp/db2_delta_backup_script.sh";
    run_command(&script, script_path);    
}



pub fn online_backup(reference :&str){

    let script = format!(r#"
        #!/bin/sh
        db2 terminate
        db2 deactivate db bbdd2
    
        db2 connect to bbdd2
        db2 quiesce database immediate force connections
        db2 connect reset

        db2 "BACKUP DATABASE bbdd2 ONLINE TO {route}"
        
        db2 connect to bbdd2
        db2 unquiesce database
        db2 connect reset
        
        db2 activate db bbdd2
    "#, route = reference);
    let script_path = "/tmp/db2_online_backup_script.sh";
    run_command(&script, script_path);    
}

pub fn compressed_backup(reference :&str){

    let script = format!(r#"
        #!/bin/sh
        db2 terminate
        db2 deactivate db bbdd2
    
        db2 connect to bbdd2
        db2 quiesce database immediate force connections
        db2 connect reset

        db2 "BACKUP DATABASE bbdd2 TO {route} COMPRESS "
        
        db2 connect to bbdd2
        db2 unquiesce database
        db2 connect reset

        db2 activate db bbdd2
    "#, route = reference);
    let script_path = "/tmp/db2_compressed_backup_script.sh";
    run_command(&script, script_path);    
}

/// Crear backup de TableSpaces: 
/// 
///     
pub fn tablespaces_backup(reference :&str){

    let script = format!(r#"
        #!/bin/sh
        db2 terminate
        db2 deactivate db bbdd2
    
        db2 connect to bbdd2
        db2 quiesce database immediate force connections
        db2 connect reset
        
        db2 "BACKUP DATABASE bbdd2 TABLESPACE (DB2RUSTICTTBPS) TO {route}"
        
        db2 connect to bbdd2
        db2 unquiesce database
        db2 connect reset

        db2 activate db bbdd2
    "#, route = reference);
    let script_path = "/tmp/db2_tablespaces_backup_script.sh";
    run_command(&script, script_path);    
}