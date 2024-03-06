# mina-archive sql dump comparison tool

## Prerequisites

- source.csv - csv file generated from database that was replayed by replayer.
- target.csv - csv file generated from original database that was not touched by replayer.

## Build the `sqldiff` tool

Rust toolchain needs to be installed.
For nix users: 
```bash
$ nix develop .

```
For others: follow your OS preferred way.

```bash
$ cargo run -- --help
A tool to generate postgresql patch from mina archive database diff.

Usage: sqldiff --source <source.csv> --target <target.csv> --output <patch.sql>

Options:
  -s, --source <source.csv>  csv file containing fixed mina archive balances
  -t, --target <target.csv>  csv file with unpatched mina archive balances
  -o, --output <patch.sql>   where to output an sql patch that fixes mina archive balances
  -h, --help                 Print help
```

## Getting started

In order to create an sql diff file, you can follow these steps:
1. Create target.csv from a database that was fixed by replayer.

  ```bash
  psql postgres://mina:_@localhost:5434/archive_balances_migrated -c "select balance,block_height,block_sequence_no,block_secondary_sequence_no,nonce from balances ORDER BY (block_height,block_sequence_no,block_secondary_sequence_no)" -t -A -F"," -o ./source.csv
  ```
2. Create source.csv from a database that you want to compare with the database fixed by replayer.

  ```bash
  psql postgres://mina:_@localhost:5434/archive_balances_migrated -c "select balance,block_height,block_sequence_no,block_secondary_sequence_no,nonce from balances ORDER BY (block_height,block_sequence_no,block_secondary_sequence_no)" -t -A -F"," -o ./target.csv
  ```
3. Run the `sqldiff` tool to produce sql patch file.

  ```bash
  sqldiff --source source.csv --target target.csv --output patch.sql
  ```
4. Apply the patch on the databases that you want to fix broken nonces.

  ```bash
  psql postgres://USER:PASS@HOST:5432/DBNAME -f ./patch.sql
  ```
