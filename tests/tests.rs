#[cfg(test)]
mod tests {
    use std::collections::HashMap; // Importing the HashMap type
    use rusqlite::Connection; // Importing the Connection type
    use ip2_sy275::{setup_db, insert_animals, get_animals, update_animal_name, delete_animal_by_name}; // Importing the functions from lib.rs

    #[test]
    fn test_setup_db() {
        let res = setup_db();
        assert!(res.is_ok(), "Failed to set up the database");
    }

    #[test]
    fn test_insert_animals() {
        let mut animals = HashMap::new();
        animals.insert(String::from("Whiskers"), String::from("Gray"));
        let res = insert_animals(&animals);
        assert!(res.is_ok(), "Failed to insert animals into the database");
    }

    #[test]
    fn test_get_animals() {
        let animals = get_animals();
        assert!(animals.is_ok(), "Failed to get animals from the database");
    }

    #[test]
    fn test_update_animal_name() {
        let conn = Connection::open("animals.db").unwrap();
        // insert first to make sure test successful
        let mut animals = HashMap::new();
        animals.insert(String::from("Whiskers"), String::from("Gray"));
        let res = insert_animals(&animals);
        assert!(res.is_ok(), "Failed to insert animals into the database");

        let res = update_animal_name(&conn, "Whiskers", "WhiskersUpdated");
        assert!(res.is_ok(), "Failed to update animal name");

        let updated_animals = get_animals().unwrap();
        assert!(updated_animals.iter().any(|animal| animal.name == "WhiskersUpdated"), "Updated name not found in the database");
    }

    #[test]
    fn test_delete_animal_by_name() {
        let conn = Connection::open("animals.db").unwrap();
        let res = delete_animal_by_name(&conn, "WhiskersUpdated");
        assert!(res.is_ok(), "Failed to delete animal by name");

        let updated_animals = get_animals().unwrap();
        assert!(!updated_animals.iter().any(|animal| animal.name == "WhiskersUpdated"), "Deleted animal was found in the database");
    }
}
