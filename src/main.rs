use rusqlite::Connection;
// <-- Add this import
use ip2_sy275::{setup_db, insert_cats, update_cat_name, delete_cat_by_name, print_all_cats};
use rusqlite::Result;
use std::collections::HashMap;


fn main() -> Result<()> {
    // Setup the database and tables
    setup_db()?;

    // Insert cats into the database
    let mut cat_colors = HashMap::new();
    cat_colors.insert(String::from("Blue"), vec![String::from("Tigger"), String::from("Sammy")]);
    cat_colors.insert(String::from("Black"), vec![String::from("Oreo"), String::from("Biscuit")]);

    insert_cats(&cat_colors).unwrap();

    // Initial state of the database
    println!("Initial state of the database:");
    print_all_cats()?;

    // Demonstrate update
    let conn = Connection::open("cats.db")?;
    update_cat_name(&conn, "Tigger", "TiggerUpdated")?;
    println!("After updating Tigger's name:");
    print_all_cats()?;

    // Demonstrate delete
    delete_cat_by_name(&conn, "Oreo")?;
    println!("After deleting Oreo:");
    print_all_cats()?;

    Ok(())
}
