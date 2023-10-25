use rusqlite::{Connection, NO_PARAMS, Result};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Cat {
    pub name: String,
    pub color: String,
}

pub fn setup_db() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}

pub fn insert_cats() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec!["Tigger", "Sammy"]);
    cat_colors.insert(String::from("Black"), vec!["Oreo", "Biscuit"]);

    for (color, catnames) in &cat_colors {
        conn.execute(
            "INSERT INTO cat_colors (name) values (?1)",
            &[&color.to_string()],
        )?;
        let last_id: String = conn.last_insert_rowid().to_string();

        for cat in catnames {
            conn.execute(
                "INSERT INTO cats (name, color_id) values (?1, ?2)",
                &[&cat.to_string(), &last_id],
            )?;
        }
    }
    Ok(())
}

pub fn get_cats() -> Result<Vec<Cat>> {
    let conn = Connection::open("cats.db")?;
    let mut stmt = conn.prepare(
        "SELECT c.name, cc.name from cats c
         INNER JOIN cat_colors cc
         ON cc.id = c.color_id;",
    )?;

    let cats = stmt.query_map(NO_PARAMS, |row| {
        Ok(Cat {
            name: row.get(0)?,
            color: row.get(1)?,
        })
    })?;

    let mut cat_list = Vec::new();
    for cat in cats {
        cat_list.push(cat?);
    }

    Ok(cat_list)
}

pub fn update_cat_name(conn: &Connection, old_name: &str, new_name: &str) -> Result<()> {
    conn.execute(
        "UPDATE cats SET name = ?1 WHERE name = ?2",
        &[&new_name, &old_name],
    )?;
    Ok(())
}

pub fn delete_cat_by_name(conn: &Connection, cat_name: &str) -> Result<()> {
    conn.execute("DELETE FROM cats WHERE name = ?", &[&cat_name])?;
    Ok(())
}