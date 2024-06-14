use clap::{command, Command};
use fullbackupdb2::make_full_backup;
use incrementalbackup::make_incremental_backup;
use loaddata::loaddata::load_command;
//importacion de modulos
mod loaddata;
mod command_executor;
mod fullbackupdb2;
mod incrementalbackup;





fn main() {

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
            .about("Crea un backaup completo de la base de datos")
        )
        .subcommand(
            Command::new("incremental")
            .short_flag('i')
            .about("Crea unn backaup pero de lo nuevo desde el ultimo backup full")
        )
        .subcommand(
            Command::new("delta")
            .short_flag('d')
            .about("Createsun backup delta de la base de datos.")
        ).subcommand(
            Command::new("online")
            .short_flag('o')
            .about("Crea un backup completo, pero manteniendo la base de datos disponible")
        ).subcommand(
            Command::new("compressed")
            .short_flag('c')
            .about("Creates un backup comprimido de la base de datos.")
        ).subcommand(
            Command::new("tablespaces")
            .short_flag('t')
            .about("Crea un backup de un tablespaces en especifico.")
        )
    )

    .get_matches();

    match matches.subcommand() {
        Some(("load", _)) => load_command(),
        Some(("backup", bck_arg)) => match bck_arg.subcommand() {
            Some(("full",_)) => make_full_backup(),
            Some(("incremental", _)) => make_incremental_backup(),
            _ => println!("Comando no admitido")
        }
        _ => println!("Comando no admitido")

        
    }
        
}



