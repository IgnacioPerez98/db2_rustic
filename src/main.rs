use backup::{compressed_backup, delta_backup, full_backup, incremental_backup, online_backup, tablespaces_backup};
use clap::{command, Command};
use loaddata::loaddata::load_command;



//importacion de modulos
mod loaddata;
mod command_executor;
mod backup;
mod recovery; 


fn main() {
    let backup_route = String::from("/media/db2ucu/Backups");

    let matches = command!()
    .subcommand(
        Command::new("load")
        .short_flag('l')
        .about("Carga informacion a la base de datos y suele ser mas rapido y para volumenes de datos mayores que import.")
    )
    .subcommand(
        Command::new("backup")
        .short_flag('b')
        .about("Crea un back up de la base de datos")
        .subcommand(
            Command::new("full")
            .short_flag('f')
            .about("Crea un back up completo de la base de datos")
        )
        .subcommand(
            Command::new("incremental")
            .short_flag('i')
            .about("Crea un back up pero de lo nuevo desde el ultimo backup full")
        )
        .subcommand(
            Command::new("delta")
            .short_flag('d')
            .about("Crea un back up delta de la base de datos.")
        ).subcommand(
            Command::new("online")
            .short_flag('o')
            .about("Crea un back up completo, pero manteniendo la base de datos disponible")
        ).subcommand(
            Command::new("compressed")
            .short_flag('c')
            .about("Crea un back up comprimido de la base de datos.")
        ).subcommand(
            Command::new("tablespaces")
            .short_flag('t')
            .about("Crea un back up de un tablespaces en especifico.")
        )
    )
    .subcommand(
        Command::new("recovery")
        .short_flag('r')
        .about("Carga un respaldo a la base de datos.")
        
    )
    .get_matches();

    match matches.subcommand() {
        Some(("load", _)) => load_command(),
        Some(("backup", bck_arg)) => match bck_arg.subcommand() {
            Some(("full",_)) => full_backup(&backup_route),
            Some(("incremental", _)) => incremental_backup(&backup_route),
            Some(("delta",_)) => delta_backup(&backup_route),
            Some(("online",_)) => online_backup(&backup_route),
            Some(("tablespaces",_)) => tablespaces_backup(&backup_route),
            Some(("compressed",_)) => compressed_backup(&backup_route),
            _ => println!("Comando no admitido")
        }
        _ => println!("Comando no admitido")

        
    }
        
}



