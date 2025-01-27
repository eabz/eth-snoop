<h1 align="center">
<strong>ETH Snoop</strong>
</h1>
<p align="center">
<strong>ETH event indexer boilerplate</strong>
</p>

![build](https://github.com/eabz/eth-snoop/actions/workflows/build.yml/badge.svg)

@TODO

## Program flags

| Flag           |  Default  | Purpose                                                                  |
| -------------- | :-------: | ------------------------------------------------------------------------ |
| `--debug`      |   false   | Enables verbose (debug-level) logging output.                            |
| `--chain`      | `mainnet` | Specifies the target chain/network to index (e.g. 'mainnet', 'testnet'). |
| `--rpc`        |  `empty`  | URL of the RPC endpoint to fetch chain data and logs.                    |
| `--database`   |  `empty`  | PostgreSQL connection URL (e.g. 'postgres://user:password@host/dbname'). |
| `--batch-size` |   `50`    | Number of blocks to fetch in each batch of logs.                         |
