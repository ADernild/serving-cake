use rusqlite::{Connection, Result};
use uuid::Uuid;

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("cake.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cake_slices (
            id TEXT PRIMARY KEY,
            message TEXT NOT NULL,
            surprise TEXT,
            slices_left INTEGER NOT NULL
        )",
        [],
    )?;
    // Initialize with 42 slices if empty
    let count: i32 = conn.query_row("SELECT COUNT(*) FROM cake_slices", [], |row| row.get(0))?;
    if count == 0 {
        conn.execute(
            "INSERT INTO cake_slices (id, message, surprise, slices_left) VALUES (?, ?, ?, ?)",
            ("initial", "Cake initialized", None::<String>, 42),
        )?;
    }
    Ok(conn)
}

pub fn get_slices_left(conn: &Connection) -> Result<i32> {
    conn.query_row(
        "SELECT slices_left FROM cake_slices ORDER BY slices_left ASC LIMIT 1",
        [],
        |row| row.get(0),
    )
}

pub fn take_slice(conn: &Connection, message: &str, surprise: Option<&str>) -> Result<()> {
    let slices_left = get_slices_left(conn)?;
    println!("{}", &slices_left);
    if slices_left > 0 {
        let new_slices = slices_left - 1;
        conn.execute(
            "INSERT INTO cake_slices (id, message, surprise, slices_left) VALUES (?, ?, ?, ?)",
            (Uuid::new_v4().to_string(), message, surprise, new_slices),
        )?;
    }
    Ok(())
}
