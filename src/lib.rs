use globwalk::*;
use rayon::prelude::*;
use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::{prelude::*, BufReader, Seek, SeekFrom};
use std::result::Result;
use std::time::{Instant, SystemTime};

pub struct Config {
    pub base_dir: String,
    pub glob_pattern: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let base_dir = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a base direction"),
        };

        let glob_pattern = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a glob pattern"),
        };

        let case_sensitive: bool = env::var("CASE_SENSITIVE")
            .unwrap_or_default()
            .parse()
            .unwrap_or(true);

        Ok(Config {
            base_dir,
            glob_pattern,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) {
    let base_dir = "/Users/KT/Dev/personal/grepper/src";
    let patterns = ["*.{rs}"];
    let now = Instant::now();

    GlobWalkerBuilder::from_patterns(base_dir, &patterns)
        .case_insensitive(true)
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .collect::<Vec<DirEntry>>()
        .par_iter()
        .for_each(move |e| apply(e));

    println!("{:?}", now.elapsed())
}

fn apply(dir_entry: &DirEntry) {
    if let Some(path) = dir_entry.path().to_str() {
        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(path)
            .unwrap();

        let mut reader = BufReader::new(&file);
        let mut contents = String::new();

        match reader.read_to_string(&mut contents) {
            Ok(len) => println!("finish reading {} - {} bytes", path, len),
            Err(e) => println!("failure reading {}: \n{}", path, e),
        };

        let result = [APACHE_2_0, contents.as_str()].join("\n");

        file.seek(SeekFrom::Start(0)).unwrap();
        file.write_all(result.as_bytes()).unwrap();
        file.sync_all().unwrap();
    } else {
        println!(
            "{:?} - Unable to parse file {:?}",
            SystemTime::now(),
            dir_entry.path()
        );
    }
}

const APACHE_2_0: &'static str = "/**
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * 'License'); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * 'AS IS' BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
";
