// tests/test.rs

use ip2_sy275::{
    get_cats, update_cat_name, delete_cat_by_name
};
use rusqlite::Connection;


#[test]
fn test_update_cat_name() -> rusqlite::Result<()> {
    let conn = Connection::open("cats.db")?;

    // Update a cat's name
    update_cat_name(&conn, "Tigger", "TiggerTest")?;

    // Ensure name was updated
    let cats = get_cats()?;
    let updated_cat = cats.iter().find(|&cat| cat.name == "TiggerTest").unwrap();
    assert_eq!(updated_cat.name, "TiggerTest");

    Ok(())
}

#[test]
fn test_delete_cat_by_name() -> rusqlite::Result<()> {
    let conn = Connection::open("cats.db")?;

    // Delete a cat by name
    delete_cat_by_name(&conn, "Biscuit")?;

    // Ensure cat was deleted
    let cats = get_cats()?;
    let deleted_cat = cats.iter().find(|&cat| cat.name == "Biscuit");
    assert!(deleted_cat.is_none());

    Ok(())
}
