use crate::chains::{get_chain, Chain};
use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(
    name = "ETH Snoop",
    about = "Scalable SQL indexer for the Monad Blockchain."
)]
pub struct IndexerArgs {
    #[arg(
        long,
        help = "Amount of blocks to fetch logs from.",
        default_value_t = 50
    )]
    pub batch_size: usize,
    #[arg(
        long,
        help = "String identifying the chain to sync.",
        default_value_t = String::from("mainnet")
    )]
    pub chain: String,
    #[arg(long, help = "Postgres database url")]
    pub database: String,
    #[arg(long, help = "Start log with debug.", default_value_t = false)]
    pub debug: bool,
    #[arg(long, help = "Url of the RPC endpoint")]
    pub rpc: String,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub batch_size: usize,
    pub chain: Chain,
    pub db_url: String,
    pub debug: bool,
    pub rpc: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    pub fn new() -> Self {
        let args = IndexerArgs::parse();

        let chain = get_chain(args.chain);

        Self {
            batch_size: args.batch_size,
            chain,
            db_url: args.database,
            debug: args.debug,
            rpc: args.rpc,
        }
    }
}
