use csv::Writer;
use rusqlite::Connection;

pub fn export_to_csv(conn: &Connection, file_path: &str) -> std::io::Result<()> {
    let mut wtr = Writer::from_path(file_path)?;

    wtr.write_record(&["ID", "Type", "Amount", "Description", "Date"])?;

    let mut stmt = conn.prepare("SELECT id, type, amount, description, date FROM transactions")?;
    let transactions = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i32>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, f64>(2)?,
            row.get::<_, String>(3)?,
            row.get::<_, String>(4)?,
        ))
    })?;

    for transaction in transactions {
        let (id, trans_type, amount, description, date) = transaction?;
        wtr.write_record(&[
            id.to_string(),
            trans_type,
            amount.to_string(),
            description,
            date,
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
