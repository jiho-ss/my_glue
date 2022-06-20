use gluesql::prelude::*;

fn main() {
    let storage = MemoryStorage::default();
    let mut glue = Glue::new(storage);
    let create = "CREATE TABLE Account (id INTEGER, name TEXT);";
    let insert = "INSERT INTO Account VALUES(1, \"jiho\");";
    let select = "SELECT * FROM Account WHERE id = 1";
    glue.execute(create).unwrap();
    glue.execute(insert).unwrap();
    let result = glue.execute(select).unwrap();
    println!("{:?}", result)
}
