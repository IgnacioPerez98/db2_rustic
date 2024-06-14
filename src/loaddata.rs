pub mod loaddata {
    use crate::command_executor::{execute_script, make_executable, write_script};

    pub fn load_command() {
        // Define the commands to be executed in a single shell script
        let script_content = r#"
        #!/bin/sh
        db2 connect to bbdddos user db2ucu using penarolcds1891
        db2 "LOAD FROM '/home/db2ucu/Desktop/dataci.csv' OF DEL MODIFIED BY COLDEL; INSERT INTO URUGUAYDATA.CITIZENS (CI, NOMBRE1, NOMBRE2, APELLIDO1, APELLIDO2) NONRECOVERABLE"
        db2 commit
        db2 disconnect bbdddos
        "#;

        // Define the path to the temporary shell script
        let script_path = "/tmp/db2_load_script.sh";

        // Write the shell script to a file
        write_script(script_path, script_content).expect("Failed to write shell script");
        make_executable(script_path);
        // Execute the shell script
        println!("Executing the script:\n{}", script_content);
        execute_script(script_path).expect("Failed to execute shell script");
        println!("Script execution finished.");
    }

    #[cfg(test)]
    mod tests {
        use super::load_command;

        #[test]
        fn test_load_command() {
            // Note: This test will actually run the `load_command` function.
            // Ensure the environment is set up correctly for this test to succeed.
            load_command();
        }
    }
}