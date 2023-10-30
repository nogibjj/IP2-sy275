use rusqlite::{Connection, NO_PARAMS, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Animal {
    pub name: String,
    pub color: String,
}

pub fn setup_db() -> Result<()> {
    let conn = Connection::open("animals.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS animals (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE,
             color TEXT NOT NULL
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn insert_animals(animals: &HashMap<String, String>) -> Result<()> {
    let conn = Connection::open("animals.db")?;

    for (name, color) in animals {
        conn.execute(
            "INSERT OR IGNORE INTO animals (name, color) VALUES (?1, ?2)",
            &[name, &color],
        )?;
    }
    Ok(())
}

pub fn get_animals() -> Result<Vec<Animal>> {
    let conn = Connection::open("animals.db")?;
    let mut stmt = conn.prepare("SELECT name, color FROM animals")?;

    let animals = stmt.query_map(NO_PARAMS, |row| {
        Ok(Animal {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    let mut animal_list = Vec::new();
    for animal in animals {
        animal_list.push(animal?);
    }

    Ok(animal_list)
}

pub fn update_animal_name(conn: &Connection, old_name: &str, new_name: &str) -> Result<()> {
    conn.execute(
        "UPDATE animals SET name = ?1 WHERE name = ?2",
        &[new_name, old_name],
    )?;
    Ok(())
}

pub fn delete_animal_by_name(conn: &Connection, animal_name: &str) -> Result<()> {
    conn.execute("DELETE FROM animals WHERE name = ?", &[animal_name])?;
    Ok(())
}

pub fn print_all_animals() -> Result<()> {
    let animals = get_animals()?;
    println!("----------------------");
    for animal in animals {
        println!("{:?} ", animal);
    }
    println!("----------------------");
    Ok(())
}
