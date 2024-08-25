extern crate core;
use anyhow::{anyhow, Error};
use std::collections::HashMap;
use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{env, thread};

mod get_repos;
mod options;

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
            .expect("invalid output argument")
            .to_string();
        if let Some(tk) = settings_map.get("token") {
            token = tk.to_string()
        } else {
            token = "".to_string()
        }
    } else {
        (user, output, token) = load_from_clap()
    }

    println!("User: {}\nOutput Path: {}\n", user, output);


    if let Ok(mut user_repos) = get_repos(user.as_str(), token.as_str()).await {

        while !user_repos.is_empty() {
            let mut working = vec![];
            for _i in 0..12 {
                working.push(user_repos.pop())
            }
            let mut handles: Vec<JoinHandle<()>> = vec![];
            for r in working.iter() {
                if let Some(repo) = r.clone() {
                    let repo_name = repo
                        .html_url
                        .as_str()
                        .split('/')
                        .last()
                        .expect("Could not parse url");
                    if !output.ends_with('/') {
                        output.push('/')
                    }

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
                    }

                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }

            clean_up_handles(handles);
        }
        Ok(())
    } else {
        Err(anyhow!("Error Downloading Repos"))
    }
}

fn clean_up_handles(handles: Vec<JoinHandle<()>>) {
    // clean up handles
    for handle in handles {
        if let Err(_) = handle.join() {
            println!("COULD NOT JOIN ON THREAD")
        }
    }
}
