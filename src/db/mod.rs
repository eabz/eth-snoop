pub mod models;

use crate::chains::Chain;
use core::panic;
use diesel::{Connection, PgConnection};
use diesel_migrations::{
    embed_migrations, EmbeddedMigrations, MigrationHarness,
};
use futures::future::join_all;
use log::*;
use std::cmp::min;

pub const MAX_PARAM_SIZE: u16 = u16::MAX;

pub const MIGRATIONS: EmbeddedMigrations =
    embed_migrations!("migrations/");

pub enum DatabaseTables {
    Events,
    Logs,
}

impl DatabaseTables {
    pub fn as_str(&self) -> &'static str {
        match self {
            DatabaseTables::Events => "events",
            DatabaseTables::Logs => "logs",
        }
    }
}

#[derive(Clone)]
pub struct Database {
    pub chain: Chain,
    pub db_url: String,
}

impl Database {
    pub async fn new(db_url: String, chain: Chain) -> Self {
        info!("Starting database service");

        let mut db = PgConnection::establish(&db_url)
            .expect("unable to connect to the database");

        db.run_pending_migrations(MIGRATIONS).unwrap();

        Self { chain, db_url }
    }

    pub fn get_connection(&self) -> PgConnection {
        PgConnection::establish(&self.db_url)
            .expect("unable to connect to the database")
    }

    pub async fn store_data(&self) {
        let mut stores: Vec<tokio::task::JoinHandle<()>> = vec![];

        let res = join_all(stores).await;

        let errored: Vec<_> =
            res.iter().filter(|res| res.is_err()).collect();

        if !errored.is_empty() {
            panic!("failed to store all chain primitive elements")
        }

        //self.store_blocks(&data.blocks).await;

        /* info!(
            "Inserted: logs ({}) events ({}).",
            data.logs.len(),
            data.events.len(),
        ); */
    }
}

/// Ref: https://github.com/aptos-labs/aptos-core/blob/main/crates/indexer/src/database.rs#L32
/// Given diesel has a limit of how many parameters can be inserted in a single operation (u16::MAX)
/// we may need to chunk an array of items based on how many columns are in the table.
/// This function returns boundaries of chunks in the form of (start_index, end_index)
pub fn get_chunks(
    num_items_to_insert: usize,
    column_count: usize,
) -> Vec<(usize, usize)> {
    let max_item_size = MAX_PARAM_SIZE as usize / column_count;
    let mut chunk: (usize, usize) =
        (0, min(num_items_to_insert, max_item_size));
    let mut chunks = vec![chunk];
    while chunk.1 != num_items_to_insert {
        chunk = (
            chunk.0 + max_item_size,
            min(num_items_to_insert, chunk.1 + max_item_size),
        );
        chunks.push(chunk);
    }
    chunks
}
