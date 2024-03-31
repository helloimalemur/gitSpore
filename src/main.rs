extern crate core;


use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::thread::JoinHandle;
use std::time::Duration;
use clap::Parser;

mod get_repos;
mod options;

use crate::get_repos::download_repo;
use get_repos::*;
use crate::options::{Arguments, load_from_config_file};

#[tokio::main]
async fn main() {
    let mut settings_map = HashMap::<String, String>::new();

    let mut user = String::new();
    let mut output = String::new();
    let mut token = String::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args.get(1).unwrap().eq_ignore_ascii_case("config-file") {
        load_from_config_file(&mut settings_map);
        user = settings_map.get("user").expect("invalid user argument").to_string();
        output = settings_map.get("output").expect("invalid output argument").to_string();
        token = settings_map.get("token").expect("invalid token argument").to_string();
    } else {
        let options = Arguments::parse();
        user = options.user.to_string();
        output = options.output_folder.to_string();
        token = options.token.to_string();
    }

    println!("User: {}\nOutput Path: {}\n", user, output);

    let user_repos = get_repos(user.as_str(), token.as_str()).await;

    // let pb = indicatif::ProgressBar::new(user_repos.len() as u64);

    let mut handles: Vec<JoinHandle<()>> = vec![];

    // each repo, sleeping 1s between repo
    for (_int, repo) in user_repos.iter().enumerate() {
        // println!("{}", repo.clone().html_url);
        let repo_name = repo.html_url.as_str().split('/').last().unwrap();
        let final_output_path = format!("{}{}/", output, repo_name);

        if Path::new(final_output_path.as_str()).exists() {
            let handle = update_repo(final_output_path);
            handles.push(handle);
        } else {
            let handle = download_repo(
                String::from(repo.html_url.as_str()),
                String::from(repo_name),
                String::from(final_output_path),
                String::from(&token),
            );
            handles.push(handle);
            // pb.println(format!("[+] #{}/{}", int, user_repos.len()));
            // pb.inc(1);
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    // clean up handles
    for handle in handles {
        handle.join().unwrap()
    }
}
