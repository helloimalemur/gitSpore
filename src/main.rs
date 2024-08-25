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

    let mut total_repos = 0;
    let mut work_count = 0;
    if let Ok(mut user_repos) = get_repos(user.as_str(), token.as_str()).await {
        total_repos = user_repos.len();
        let mut trans = vec![];
        let mut working = vec![];

        while !user_repos.is_empty() {
            let mut handles: Vec<JoinHandle<()>> = trans;

            if handles.len() < 30 && working.len() < 30 {
                for _i in 0..5 {
                    working.push(user_repos.pop())
                }
            }

            while !working.is_empty() {
                let r = working.pop().unwrap();

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

            trans = clean_up_handles(handles, &mut work_count);
        }
        println!("User: {}\nOutput Path: {}\nTotal repos: {}/{}", user, output, work_count, total_repos);
        Ok(())
    } else {
        Err(anyhow!("Error Downloading Repos"))
    }
}

fn clean_up_handles(handles: Vec<JoinHandle<()>>, mut work_count: &mut i32) -> Vec<JoinHandle<()>> {
    // clean up handles
    let mut res = vec![];
    for handle in handles {
        *work_count += 1;
        let _ = handle.join();

        // if handle.is_finished() {
        //     *work_count += 1;
        //     handle.join().unwrap()
        // } else {
        //     res.push(handle)
        // }
    }
    // println!("handles handed off: {}", res.len());
    res
}
