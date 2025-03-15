use chrono::Local;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::sqlite::SqliteConnection;
use diesel::{allow_tables_to_appear_in_same_query, joinable, table};
use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

table! {
    map_sets (id) {
        id -> Integer,
        title -> Text,
        version -> Text,
        bpm -> Integer,
    }
}

table! {
    score_sets (id) {
        id -> Integer,
        accuracy -> Double,
        unstable_rate -> Double,
        date -> Text,
        map_set_id -> Integer,
    }
}

joinable!(score_sets -> map_sets (map_set_id));
allow_tables_to_appear_in_same_query!(map_sets, score_sets,);

/// used for map_set table
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, AsChangeset)]
#[table_name = "map_sets"]
struct MapSet {
    /// primary key
    id: i32,
    /// title
    title: String,
    /// map's difficulty
    version: String,
    /// map's bpm
    bpm: i32,
}

#[derive(Insertable)]
#[table_name = "map_sets"]
pub struct NewMapSet {
    /// title
    pub title: String,
    /// map's difficulty
    pub version: String,
    /// map's bpm
    pub bpm: i32,
}

/// used for score_set table, stores user's score
#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, Associations, AsChangeset)]
#[table_name = "score_sets"]
#[belongs_to(MapSet)]
struct ScoreSet {
    /// primary key
    id: i32,
    /// accuracy
    accuracy: f64,
    /// unstable rate
    unstable_rate: f64,
    /// when played. %Y-%m-%d %H:%M:%S
    date: String,
    /// played map. related to map_set's id
    map_set_id: i32,
}

#[derive(Insertable, Associations)]
#[table_name = "score_sets"]
#[belongs_to(MapSet)]
pub struct NewScoreSet {
    /// accuracy
    pub accuracy: f64,
    /// unstable rate
    pub unstable_rate: f64,
    /// when played. %Y-%m-%d %H:%M:%S
    pub date: String,
    /// played map. related to map_set's id
    pub map_set_id: i32,
}

type DbPool = Pool<ConnectionManager<SqliteConnection>>;
type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

static POOL: LazyLock<DbPool> = LazyLock::new(|| {
    let manager = ConnectionManager::<SqliteConnection>::new("./tapper.db");
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
});

pub fn create_map_set(
    title: String,
    version: String,
    bpm: i32,
) -> Result<MapSet, diesel::result::Error> {
    let new_map_set = NewMapSet {
        title,
        version,
        bpm,
    };

    let mut conn = POOL.get().expect("failed to get connection");

    diesel::insert_into(map_sets::table)
        .values(&new_map_set)
        .execute(&mut conn)?;

    map_sets::table.order(map_sets::id.desc()).first(&mut conn)
}

pub fn get_map_set(id: i32) -> Result<MapSet, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    map_sets::table.find(id).first::<MapSet>(&mut conn)
}

pub fn get_all_map_sets() -> Result<Vec<MapSet>, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    map_sets::table.load::<MapSet>(&mut conn)
}

pub fn update_map_set(
    id: i32,
    title: String,
    version: String,
    bpm: i32,
) -> Result<MapSet, diesel::result::Error> {
    let updated_map_set = MapSet {
        id,
        title,
        version,
        bpm,
    };

    let mut conn = POOL.get().expect("failed to get connection");

    diesel::update(map_sets::table.find(id))
        .set(&updated_map_set)
        .execute(&mut conn)?;

    map_sets::table.find(id).first(&mut conn)
}

pub fn delete_map_set(
    id: i32,
) -> Result<usize, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    diesel::delete(map_sets::table.find(id)).execute(&mut conn)
}

pub fn create_score_set(
    accuracy: f64,
    unstable_rate: f64,
    map_set_id: i32,
) -> Result<ScoreSet, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let new_score_set = NewScoreSet {
        accuracy,
        unstable_rate,
        date,
        map_set_id,
    };

    diesel::insert_into(score_sets::table)
        .values(&new_score_set)
        .execute(&mut conn)?;

    score_sets::table.order(score_sets::id.desc()).first(&mut conn)
}

pub fn get_score_set(
    id: i32,
) -> Result<ScoreSet, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    score_sets::table.find(id).first::<ScoreSet>(&mut conn)
}

pub fn get_all_score_sets(
) -> Result<Vec<ScoreSet>, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    score_sets::table.load::<ScoreSet>(&mut conn)
}

pub fn get_score_sets_by_map_set(
    map_id: i32,
) -> Result<Vec<ScoreSet>, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    score_sets::table
        .filter(score_sets::map_set_id.eq(map_id))
        .load::<ScoreSet>(&mut conn)
}

pub fn update_score_set(
    id: i32,
    accuracy: f64,
    unstable_rate: f64,
    map_set_id: i32,
) -> Result<ScoreSet, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    let date = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let updated_score_set = ScoreSet {
        id,
        accuracy,
        unstable_rate,
        date,
        map_set_id,
    };

    diesel::update(score_sets::table.find(id))
        .set(&updated_score_set)
        .execute(&mut conn)?;

    score_sets::table.find(id).first(&mut conn)
}

pub fn delete_score_set(
    id: i32,
) -> Result<usize, diesel::result::Error> {
    let mut conn = POOL.get().expect("failed to get connection");
    diesel::delete(score_sets::table.find(id)).execute(&mut conn)
}
