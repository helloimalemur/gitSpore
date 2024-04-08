extern crate core;
use anyhow::{anyhow, Error};
use std::collections::HashMap;
use std::env;
use std::path::Path;
use std::thread::JoinHandle;
use std::time::Duration;

mod dump_repos;
mod get_repos;
mod options;

use crate::dump_repos::clone_all;
use crate::get_repos::download_repo;
use crate::options::{load_from_clap, load_from_config_file};
use get_repos::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut settings_map = HashMap::<String, String>::new();

    #[allow(unused)]
    let mut user = String::new();
    #[allow(unused)]
    let mut output = String::new();
    #[allow(unused)]
    let mut token = String::new();

    let args: Vec<String> = env::args().collect();

    if args.len() > 1
        && match args.get(1) {
            None => {
                panic!("Invalid argument")
            }
            Some(e) => e.eq_ignore_ascii_case("config-file"),
        }
    {
        load_from_config_file(&mut settings_map);
        user = settings_map
            .get("user")
            .expect("invalid user argument")
            .to_string();
        output = settings_map
            .get("output")
            .unwrap_or(&"./".to_string())
            .to_string();
        token = settings_map
            .get("token")
            .unwrap_or(&"None".to_string())
            .to_string();
    } else {
        (user, output, token) = load_from_clap()
    }

    println!("User: {}\nOutput Path: {}\n", user, output);

    if token.eq("None") {
        let repos = dump_repos::dump_repos(user.clone()).await;
        clone_all(user, &repos);
        println!("Dump OK. \n{:#?}", repos);
        return Ok(());
    }

    if let Ok(user_repos) = get_repos(user.as_str(), token.as_str()).await {
        let mut handles: Vec<JoinHandle<()>> = vec![];

        // each repo, sleeping 1s between repo
        for repo in user_repos.iter() {
            // println!("{}", repo.clone().html_url);
            let repo_name = repo
                .html_url
                .as_str()
                .split('/')
                .last()
                .expect("Could not parse url");
            let final_output_path = format!("{}{}/", output, repo_name);

            if Path::new(final_output_path.as_str()).exists() {
                let handle = update_repo(final_output_path);
                handles.push(handle);
            } else {
                let handle = download_repo(
                    String::from(repo.html_url.as_str()),
                    String::from(repo_name),
                    final_output_path,
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
        Ok(())
    } else {
        Err(anyhow!("Error Downloading Repos"))
    }
}
