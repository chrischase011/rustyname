mod sites;
use reqwest::blocking::get;
use reqwest::Error;
use colored::*;

fn help() {
    println!("Usage: rsite [options] [site]");
    println!("Options:");
    println!("  -h, --help     Display this help message");
    println!("  -l, --list     List all available sites");
    println!("  -n, --nsfw     Include NSFW sites");
    println!("  -u --username  Username to search.");
}

fn list_sites() {
    match sites::load_json() {
        Ok(sites) => {
            for site in sites {
                println!("{}: {}", site.name, site.url);
            }
        }
        Err(e) => {
            println!("Error loading sites: {}", e);
        }
    }
}

fn search_sites(nsfw: bool) {
    match sites::load_json() {
        Ok(sites) => {
            for site in sites {
                if nsfw || site.is_nsfw == Some(false) {
                    println!("{}: {}", site.name, site.url);
                }
            }
        }
        Err(e) => {
            println!("Error loading sites: {}", e);
        }
    }
}


fn search_sites_by_username(username: &str) {
    match sites::load_json() {
        Ok(sites) => {
            for site in &sites {
                let url = site.url.replace("{username}", username);
                // println!("Checking site: {}: {}", site.name, url);
                
                // Check if the constructed URL loads successfully
                match check_site(url.clone(), username.to_string()) {
                    Ok(true) => {
                        println!("{}", format!("✔ Username found on site: {}", url).green());
                    }
                    Ok(false) => {
                        println!("{}", format!("✘ Username not found on site: {}", url).red());
                    }
                    Err(e) => {
                        println!("{}", format!("Error checking site: {}: {}", e, url).yellow());
                    }
                }

                println!("==============================\n")
            }
        }
        Err(e) => {
            println!("Error loading sites: {}", e);
        }
    }
}

fn check_site(url: String, username: String) -> Result<bool, Error> {
    let full_url = format!("{}/{}", url, username);

    println!("Checking site: {}", full_url);

    let response = get(&full_url)?;
    if response.status().is_success() {
        Ok(true)
    } else {
        Ok(false) 
    }
}


fn parse_args() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        help();
    } else {
        match args[1].as_str() {
            "-h" | "--help" => help(),
            "-l" | "--list" => list_sites(),
            "-n" | "--nsfw" => search_sites(true),
            "-u" | "--username" => {
                if args.len() == 3 {
                    search_sites_by_username(&args[2]);
                } else {
                    println!("Error: Missing username");
                }
            },
            _ => {
                println!("Error: Invalid option");
                help();
            }
        }
    }
}

fn main() {
    parse_args();  // Call parse_args() to handle the arguments
}
