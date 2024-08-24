extern crate core;
use anyhow::{anyhow, Error};
use std::collections::HashMap;
use std::{env, thread};
use std::ops::DerefMut;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

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
    
    let mut handles: Vec<JoinHandle<()>> = vec![];
    
    // let mut handles: Arc<Mutex<Vec<JoinHandle<()>>>> = Arc::new(Mutex::new(vec![]));
    // let mut handle_handle = handles.clone();
    // let mut handle_handle_2 = handles.clone();
    // thread::spawn( move || {
    //     loop {
    //         if !handle_handle.lock().unwrap().is_empty() {
    //             for i in handle_handle.lock().as_mut() {
    //                 let _ = i.iter().take(1).map(JoinHandle::join);
    //             }
    //         }
    //     }
    // } );

    if let Ok(user_repos) = get_repos(user.as_str(), token.as_str()).await {
        // let mut lock = handle_handle_2.lock();
        // let handles_lock = lock.as_mut().unwrap();
        // each repo, sleeping 100ms between repo
        for repo in user_repos.iter() {
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

        // clean up handles
        // for handle in handles {
        //     if let Err(_) = handle.join() {
        //         println!("COULD NOT JOIN ON THREAD")
        //     }
        // }
        Ok(())
    } else {
        Err(anyhow!("Error Downloading Repos"))
    }
}
