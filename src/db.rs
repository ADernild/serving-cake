use rusqlite::{Connection, Result};

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

pub fn take_slice(conn: &Connection, message: &str, surprise: Option<&str>, uid: &str) -> Result<()> {
    let slices_left = get_slices_left(conn)?;
    if slices_left > 0 {
        let new_slices = slices_left - 1;
        conn.execute(
            "INSERT INTO cake_slices (id, message, surprise, slices_left) VALUES (?, ?, ?, ?)",
            (uid, message, surprise, new_slices),
        )?;
    }
    Ok(())
}

pub fn fetch_cake_by_uid(conn: &Connection, uid: &str) -> Result<(String, String, Option<String>, i32)> {
    conn.query_row(
        "SELECT id, message, surprise, slices_left FROM cake_slices WHERE id = ?1",
        [uid],
        |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
            ))
        },
    )
}
