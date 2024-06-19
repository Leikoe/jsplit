use std::fmt::format;
use std::fs::{create_dir, File};
use std::io::{BufWriter, Seek, SeekFrom, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread::current;
use serde_json::{Deserializer, Value};
use clap::Parser;

/// JSON array file splitter
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the JSON file to split
    #[arg(short, long)]
    input_path: String,

    /// Number of entries per shard
    #[arg(short, long, default_value_t = 1024)]
    shards_size: usize,


    /// Path of the output folder
    #[arg(short, long, default_value = "./shards/")]
    output_path: String,
}

fn save_shard(shard_id: usize, shard: &Vec<Value>, output_path: &Path) {
    let current_shard_file = File::create(output_path.join(&format!("shard_{}.json", shard_id)))
        .expect(&format!("couldn't create shard {}", shard_id));
    let mut writer = BufWriter::new(current_shard_file);
    serde_json::to_writer(&mut writer, &shard).expect("couldn't serialize shard");
    writer.flush().expect(&format!("couldn't write to shard_{}.json", shard_id));
}

fn main() {
    let args = Args::parse();

    let output_path = Path::new(&args.output_path);
    create_dir(&args.output_path).expect("couldn't create output directory");

    let mut file = File::open(args.input_path).expect("couldn't open input file!");
    file.seek(SeekFrom::Start(1)).expect("Should be able to eat the first '['");
    let mut file = Arc::new(file);
    let mut stream = Deserializer::from_reader(file.clone()).into_iter::<Value>();


    let mut shard_id = 0;
    let mut shard = Vec::new();
    for value in stream {
        shard.push(value.unwrap());
        if shard.len() + 1 > args.shards_size {
            println!("saving shard_{}.json ({} objects processed)", shard_id, (shard_id + 1) * args.shards_size);
            shard_id += 1;
            save_shard(shard_id, &shard, output_path);
            shard.clear();
        }
        file.seek(SeekFrom::Current(1)).expect("Should be able to eat the ','");
    }
    if !shard.is_empty() {
        shard_id += 1;
        println!("saving last shard_{}.json ({} objects processed)", shard_id, (shard_id + 1) * args.shards_size);
        save_shard(shard_id, &shard, output_path);
        shard.clear();
    }
    println!("DONE! ({} objects processed)", (shard_id + 1) * args.shards_size);
}
