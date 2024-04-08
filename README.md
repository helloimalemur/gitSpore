# gitSpore
Github backup tool that concurrently downloads all public and private github repo's of the specified user.

### Install from Crates.io
    cargo install gitspore

## Running gitSpore from cli
    Usage: gitspore [OPTIONS] -u <USER>
    
    Options:
    -u <USER>               Github username
    -o <OUTPUT_FOLDER>      folder to save github repo [default: ./]
    -t <TOKEN>              Github token - If no token is provided only the public repos will be downloaded [default: None]
    -h, --help              Print help

## Running gitSpore from config file
    Usage: gitspore config-file
#### config/Settings.toml
```toml
github_username = "helloimalemur"
output_folder = "/tmp/"
github_personal_access_token = "github_xxxxx"
```

## Development and Collaboration
#### Feel free to open a pull gitsporest, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check
####
