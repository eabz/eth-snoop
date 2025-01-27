<h1 align="center">
<strong>ETH Snoop</strong>
</h1>
<p align="center">
<strong>ETH logs and event indexer</strong>
</p>

![build](https://github.com/eabz/eth-snoop/actions/workflows/build.yml/badge.svg)

ETH Snoop is a minimal, rust-based event and logs indexer for Ethereum-compatible blockchains, intended as a boilerplate that you can fork and adapt to your own contracts. It provides just the essential scaffolding—listening to on-chain logs, organizing smart contract events, and storing them in a structured database—so that you can quickly integrate and expand it to match the specific needs of your protocol.


## Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/)

## How to use

@TODO

## Build

You can try the indexer locally or using Docker.

1. Clone the repository

```
git clone https://github.com/eabz/eth-snoop && cd eth-snoop
```

2. Build the program

```
cargo build --release
```

## Docker

Build the docker image
```
docker build . -t snoop
```

Copy the `.env.example` and rename it to `.env `

Start the indexer
```
docker compose -f docker/docker-compose.yml up
```

## Program flags

| Flag           |  Default  | Purpose                                                                  |
| -------------- | :-------: | ------------------------------------------------------------------------ |
| `--debug`      |   false   | Enables verbose (debug-level) logging output.                            |
| `--chain`      | `mainnet` | Specifies the target chain/network to index (e.g. 'mainnet', 'testnet'). |
| `--rpc`        |  `empty`  | URL of the RPC endpoint to fetch chain data and logs.                    |
| `--database`   |  `empty`  | PostgreSQL connection URL (e.g. 'postgres://user:password@host/dbname'). |
| `--batch-size` |   `50`    | Number of blocks to fetch in each batch of logs.                         |
