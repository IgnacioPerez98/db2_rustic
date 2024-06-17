pub mod loaddata {
    use crate::command_executor::run_command;


    /// DB2 LOAD:
    /// 
    ///     Ejecuta el comando :
    ///     db2 "LOAD FROM '/home/db2ucu/Desktop/MOCK_DATA.csv OF DEL MODIFIED BY COLDEL;
    ///     INSERT INTO MOCK.PERSONAS (ID,FIRSTNAME,LASTNAME,EMAIL,GENDER,IPADRESS) NONRECOVERABLE"
    /// 
    /// El mismo es mas rápido que el import y es usado para realizar cargas masivas a una base de datos.
    pub fn load_command() {
        let script = r#"
        #!/bin/sh
        db2 connect to bbdd2 user db2ucu using 1234
        db2 "LOAD FROM '/home/db2ucu/Desktop/MOCKDATA.csv' OF DEL MODIFIED BY COLDEL, INSERT INTO PERSONAS.ALUMNOS (ID,FIRSTNAME,LASTNAME,EMAIL,GENDER,IPADRESS) NONRECOVERABLE"
        db2 commit
        db2 disconnect bbdd2
        "#;
        let script_path = "/tmp/db2_load_script.sh";
        run_command(script, script_path);
    }


    

}