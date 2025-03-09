use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MapSet {
    pub id: i32,
    pub title: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScoreSet {
    pub id: i32,
    pub accuracy: f64,
    pub unstable_rate: f64,
    pub date: String,            // "YYYY-MM-DD"等の形式で保存
    pub map_set_id: Option<i32>, // map_setとの関連付け（NULL許容）
}

#[tauri::command]
pub fn init_db() -> Result<(), String> {
    let conn = Connection::open("app.db").map_err(|e| e.to_string())?;

    // map_setテーブルの作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS map_set (
        id      INTEGER PRIMARY KEY AUTOINCREMENT,
        title   TEXT NOT NULL,
        version TEXT NOT NULL
    )",
        [],
    )
    .map_err(|e| e.to_string())?;

    // score_setテーブルの作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS score_set (
        id         INTEGER PRIMARY KEY AUTOINCREMENT,
        accuracy   REAL NOT NULL,
        unstable_rate REAL NOT NULL,
        date       TEXT NOT NULL,
        map_set_id INTEGER,
        FOREIGN KEY(map_set_id) REFERENCES map_set(id)
    )",
        [],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub fn add_map_set(title: String, version: String) -> Result<(), String> {
    let conn = Connection::open("app.db").map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO map_set (title, version) VALUES (?1, ?2)",
        params![title, version],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_map_set(id: i32) -> Result<(), String> {
    let conn = Connection::open("app.db").map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM map_set WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_map_sets() -> Result<Vec<MapSet>, String> {
    let conn = Connection::open("app.db").map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, title, version FROM map_set")
        .map_err(|e| e.to_string())?;
    let map_set_iter = stmt
        .query_map([], |row| {
            Ok(MapSet {
                id: row.get(0)?,
                title: row.get(1)?,
                version: row.get(2)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut map_sets = Vec::new();
    for map_set in map_set_iter {
        map_sets.push(map_set.map_err(|e| e.to_string())?);
    }
    Ok(map_sets)
}

#[tauri::command]
pub fn add_score_set(
    accuracy: f64,
    unstable_rate: f64,
    date: String,
    map_set_id: Option<i32>,
) -> Result<(), String> {
    let conn = Connection::open("app.db").map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO score_set (hits, accuracy, date, map_set_id) VALUES (?1, ?2, ?3, ?4)",
        params![accuracy, unstable_rate, date, map_set_id],
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_score_sets() -> Result<Vec<ScoreSet>, String> {
    let conn = Connection::open("app.db").map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, hits, accuracy, date, map_set_id FROM score_set")
        .map_err(|e| e.to_string())?;
    let score_set_iter = stmt
        .query_map([], |row| {
            Ok(ScoreSet {
                id: row.get(0)?,
                accuracy: row.get(1)?,
                unstable_rate: row.get(2)?,
                date: row.get(3)?,
                map_set_id: row.get(4)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut score_sets = Vec::new();
    for score_set in score_set_iter {
        score_sets.push(score_set.map_err(|e| e.to_string())?);
    }
    Ok(score_sets)
}
