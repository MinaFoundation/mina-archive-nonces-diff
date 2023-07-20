use clap::Parser;
use csv::ReaderBuilder;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Write},
};

#[derive(Parser, Debug)]
#[clap(
    name = "sqldiff",
    about = "A tool to generate postgresql patch from mina archive database diff."
)]
/// Configuration options for the program.
struct Args {
    /// csv file containing fixed mina archive balances
    #[clap(short, long, value_name = "source.csv")]
    source: String,
    /// csv file with unpatched mina archive balances
    #[clap(short, long, value_name = "target.csv")]
    target: String,
    /// where to output an sql patch that fixes mina archive balances
    #[clap(short, long, value_name = "patch.sql")]
    output: String,
}

// struct `balance` of integers for `balance`, `block_height`, `block_sequence_no`, `block_secondary_sequence_no`, `nonce` values.
#[derive(Debug)]
struct Balance {
    balance: i64,
    block_height: i64,
    block_sequence_no: i64,
    block_secondary_sequence_no: i64,
    nonce: Option<i64>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let source = File::open(args.source)?;
    let target = File::open(args.target)?;
    let reader_src = io::BufReader::new(source);
    let reader_dst = io::BufReader::new(target);

    let lines_src: Vec<String> = reader_src.lines().map(|line| line.unwrap()).collect();
    let lines_dst: Vec<String> = reader_dst.lines().map(|line| line.unwrap()).collect();
    // initialize empty vec of strings
    let mut lines_diff: Vec<String> = Vec::new();

    let mut lines_dst_map: HashMap<String, bool> = HashMap::new();

    for line in lines_dst {
        lines_dst_map.insert(line, true);
    }
    for line in lines_src {
        if !lines_dst_map.contains_key(&line) {
            lines_diff.push(line);
        }
    }
    // parce lines_diff to balance struct
    let mut balance_vec: Vec<Balance> = Vec::new();
    for line in lines_diff {
        let mut rdr = ReaderBuilder::new()
            .has_headers(false)
            .from_reader(line.as_bytes());
        for result in rdr.records() {
            let record = result?;
            let balance = Balance {
                balance: record[0].parse::<i64>().unwrap(),
                block_height: record[1].parse::<i64>().unwrap(),
                block_sequence_no: record[2].parse::<i64>().unwrap(),
                block_secondary_sequence_no: record[3].parse::<i64>().unwrap(),
                nonce: if record[4] == "".to_string() {
                    None
                } else {
                    Some(record[4].parse::<i64>().unwrap())
                },
            };
            balance_vec.push(balance);
        }
    }
    let mut diff_sql: Vec<String> = Vec::new();
    diff_sql.push("BEGIN;".to_string());
    for balance in balance_vec {
        let sql = format!(
            "UPDATE balances SET nonce = {} WHERE balance = {} AND block_height = {} AND block_sequence_no = {} AND block_secondary_sequence_no = {};",
            match balance.nonce {
                Some(n) => n.to_string(),
                None => "NULL".to_string(),
            },
            balance.balance,
            balance.block_height,
            balance.block_sequence_no,
            balance.block_secondary_sequence_no,
        );
        diff_sql.push(sql);
    }
    diff_sql.push("COMMIT;".to_string());
    // save lines_diff to File
    let mut file = File::create(args.output)?;
    for line in diff_sql {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
