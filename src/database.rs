use rusqlite::{params, Connection, Result};

pub fn initialize_database() -> Result<Connection> {
    let conn = Connection::open("finance_manager.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY,
            type TEXT NOT NULL,
            amount REAL NOT NULL,
            description TEXT,
            date TEXT NOT NULL
        )",
        [],
    )?;
    Ok(conn)
}

pub fn add_transaction(
    conn: &Connection,
    trans_type: &str,
    amount: f64,
    description: Option<&str>,
) -> Result<()> {
    conn.execute(
        "INSERT INTO transactions (type, amount, description, date)
        VALUES (?1, ?2, ?3, datetime('now'))",
        params![trans_type, amount, description.unwrap_or("")],
    )?;
    Ok(())
}

pub fn list_transactions(conn: &Connection) -> Result<Vec<(i32, String, f64, String, String)>> {
    let mut stmt = conn.prepare(
        "SELECT id, type, amount, description, date FROM transactions ORDER BY date DESC",
    )?;
    let transactions = stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get::<_, String>(3)?,
                row.get(4)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(transactions)
}

pub fn delete_transaction(conn: &Connection, id: i32) -> Result<()> {
    conn.execute("DELETE FROM transactions WHERE id = ?1", params![id])?;
    Ok(())
}


pub fn calculate_balance(conn: &Connection) -> Result<f64> {
    let mut stmt = conn.prepare(
        "SELECT SUM(CASE WHEN type = 'income' THEN amount ELSE -amount END) AS balance FROM transactions",
    )?;
    let balance: f64 = stmt.query_row([], |row| row.get(0)).unwrap_or(0.0);
    Ok(balance)
}
