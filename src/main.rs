use ip2_sy275::{setup_db, insert_cats, get_cats};
use rusqlite::Result;

fn main() -> Result<()> {
    // Setup the database and tables
    setup_db()?;

    // Insert cats into the database
    insert_cats()?;

    // Retrieve cats from the database
    let cats = get_cats()?;
    for cat in cats {
        println!("Found cat {:?}", cat);
    }

    // update cat
    update_cat()?;

    // delete cat
    delete_cat()?;

    Ok(())
}
