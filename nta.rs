use reqwest::blocking::Client;
use reqwest::StatusCode;
use regex::Regex;
use std::error::Error;

struct NtaUpgrade {
    _repo: &'static str,
    location: String,
}

impl NtaUpgrade {
    fn new() -> NtaUpgrade {
        let _repo = "https://raw.githubusercontent.com/l1nux-th1ngz/nta/main/";
        let location = find_nta();
        NtaUpgrade {
            _repo,
            location,
        }
    }

    fn find_nta() -> String {
        // Implement find_nta logic here and return the location as a String
        // For example:
        // let location = "/path/to/nta_folder";
        // location.to_string()
        // In your actual code, you should implement this function.
        // Replace this example with your actual logic.
        "/path/to/nta_folder".to_string()
    }

    fn download_latest_version(&self) -> Result<(), Box<dyn Error>> {
        println!("Downloading the latest version of Nta...");
        let client = Client::new();
        let latest_version_url = format!("{}/nta.py", self._repo);
        let response = client.get(&latest_version_url).send()?;

        if response.status() == StatusCode::OK {
            let mut latest_version_file = std::fs::File::create(&self.location)?;

            response.copy_to(&mut latest_version_file)?;

            println!("Download complete.");
            return Ok(());
        }

        Err("Download failed".into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Nta Upgrader");
    let nta_upgrade = NtaUpgrade::new();
    nta_upgrade.download_latest_version()?;

    // Additional code for the rest of your script goes here.

    Ok(())
}
