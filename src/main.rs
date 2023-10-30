use std::env;
use rusqlite::Connection;
use rusqlite::Result;
use std::collections::HashMap;
use ip2_sy275::{setup_db, insert_animals, update_animal_name, delete_animal_by_name, print_all_animals};

fn main() -> Result<()> {
    setup_db()?;

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Please specify a command: insert, update, delete, or list");
        return Ok(());
    }

    let command = &args[1].to_lowercase();

    match command.as_str() {
        "insert" => {
            if args.len() != 4 {
                println!("Usage: insert <animal_name> <color>");
                return Ok(());
            }
            let mut animals = HashMap::new();
            animals.insert(args[2].clone(), args[3].clone());
            insert_animals(&animals)?;
        },
        "update" => {
            if args.len() != 4 {
                println!("Usage: update <old_name> <new_name>");
                return Ok(());
            }
            let conn = Connection::open("animals.db")?;
            update_animal_name(&conn, &args[2], &args[3])?;
        },
        "delete" => {
            if args.len() != 3 {
                println!("Usage: delete <animal_name>");
                return Ok(());
            }
            let conn = Connection::open("animals.db")?;
            delete_animal_by_name(&conn, &args[2])?;
        },
        "list" => {
            print_all_animals()?;
        },
        _ => {
            println!("Invalid command. Use: insert, update, delete, or list");
        },
    }

    Ok(())
}
