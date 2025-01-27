<h1 align="center">
<strong>ETH Snoop</strong>
</h1>
<p align="center">
<strong>Simple ETH event indexer boilerplate</strong>
</p>

![build](https://github.com/eabz/eth-snoop/actions/workflows/build.yml/badge.svg)

@TODO

## Program flags

| Flag            | Default     | Purpose                                                |
| --------------- | :----------:| ------------------------------------------------------ |
| `--debug`       |  false      | Start log with debug.                                  |
| `--chain`       | `mainnet`   | String identifying the chain to sync.                  |
| `--rpc`         | `empty`     | Comma separated list of rpcs to use to fetch blocks.   |
| `--database`    | `empty`     | Postgres database string with username and password.   |
| `--batch-size`  | `50`        | Amount of blocks to fetch in parallel.                 |
