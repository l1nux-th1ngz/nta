use std::fs;
use std::fs::File;
use std::io::prelude::*;
use reqwest;
use reqwest::blocking::Client;
use reqwest::StatusCode;
use std::io::Result;
use regex::Regex;
use std::path::Path;
use std::env;

struct NtaUpgrade {
    // Status Constants
    STATUS_ENABLED: i32 = 1;
    STATUS_DISABLED: i32 = 2;
    STATUS_PAUSED: i32 = 4;
    STATUS_INCOGNITO: i32 = 8;
    STATUS_ERROR: i32 = 128,
    _repo: &'static str,
    _webowner: u32,
    location: String,
    username: String,
}

impl NtaUpgrade {
    fn new() -> NtaUpgrade {
        let _repo = "https://raw.githubusercontent.com/l1nux-th1ngz/nta/main/";
        let _git_download = "https://github.com/l1nux-th1ngz/nta/archive/master.zip";
        let _webowner = 0; // Fill with the appropriate owner value

        let location = find_nta();
        let username = find_username(&location);

        NtaUpgrade {
            _repo,
            _git_download,
            _webowner,
            location,
            username,
        }
    }

    fn find_latest_version(&self) {
        // Get the latest version from bl_nta.txt
        let tempbl = format!("{}/bl_nta.txt", &folders::tempdir);
        let filelines = load_file(&tempbl);

        if filelines.is_empty() {
            logger.warning(format!(
                "{} is missing. This could be due to a recent reboot, or Nta block list is not enabled",
                &tempbl
            ));
            return;
        }

        for line in filelines {
            let re = Regex::new(r"").unwrap(); // Fill in the regex pattern
            if let Some(captures) = re.captures(&line) {
                let latest_version = captures.get(1).unwrap().as_str();
                logger.info(format!("Latest version of Nta is: {}", latest_version));
                break;
            }
        }
    }

    fn notification_create(&self) {
        // Create latestversion.php with the necessary PHP code
        let latestversionphp = format!("{}/latestversion.php", &folders::webconfigdir);
        let mut file = File::create(&latestversionphp).unwrap();

        write!(
            file,
            "<?php\n$upgradenotifier->latestversion = '{}';\n?>",
            &self._latestversion
        )
        .unwrap();

        let metadata = fs::metadata(&latestversionphp).unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o664);
        fs::set_permissions(&latestversionphp, permissions).unwrap();
    }

    fn notification_delete(&self) {
        // Delete latestversion.php
        let filename = format!("{}/latestversion.php", &folders::webconfigdir);
        logger.info("Deleting upgrade notification");
        fs::remove_file(filename).unwrap();
    }

    fn download_latest_version(&self) -> Result<()> {
        // Download the latest version from the provided URL
        logger.info("Downloading the latest version of Nta...");

        let client = Client::new();
        let latest_version_url = format!("{}/nta.py", &self._repo);
        let mut response = client.get(&latest_version_url).send()?;

        if response.status() == StatusCode::OK {
            let mut latest_version_file = File::create(&self.location)?;

            response.copy_to(&mut latest_version_file)?;

            return Ok(());
        }

        Err(std::io::Error::from(std::io::ErrorKind::Other))
    }

    // The rest of the struct methods remain the same
}

fn main() {
    // Change log level to INFO
    // Set the logger's level to INFO if applicable

    println!("Nta Upgrader");
    let services = Services::new();
    let nta_upgrade = NtaUpgrade::new();

    // Check for root privileges if needed

    nta_upgrade.find_latest_version();
    nta_upgrade.download_latest_version();

    // Perform the upgrade steps

    println!("Restarting Nta...");
    // Restart Nta if necessary

    println!("Nta upgrade complete :-)");
}
