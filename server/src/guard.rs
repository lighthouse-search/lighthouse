use std::process::Command;
use std::env;
use std::path::PathBuf;

use crate::{network::port::find_available_port, CONFIG_VALUE};
use crate::{structs::*, GUARD_HOSTNAME_TO_USE};

pub fn build_guard_hostname_to_use() -> Result<Guard_hostname_to_use, String> {
    // TODO: This needs to be a config struct.

    let mut guard_hostname: Option<String> = None;
    let mut use_local_guard = false; // Signal to future pipelines that Lighthouse must maintain a Guard executable.
    let mut guard_port: Option<u16> = None;

    if (CONFIG_VALUE.authentication.is_none() == true || CONFIG_VALUE.authentication.clone().unwrap().guard.is_none() == true || CONFIG_VALUE.authentication.clone().unwrap().guard.unwrap().hostname.is_none() == true) {
        log::info!("authentication.guard.hostname not specified on configuration, using local Guard instance...");

        guard_port = Some(find_available_port().expect("Failed to find available port"));

        use_local_guard = true;
    } else {
        log::info!("authentication.guard.hostname IS specified, using external Guard instance...");
        guard_hostname = Some(CONFIG_VALUE.authentication.clone().unwrap().guard.unwrap().hostname.expect("Failed to parse hostname"));
    }

    Ok(Guard_hostname_to_use {
        hostname: guard_hostname,
        use_local_guard: use_local_guard,
        local_port: guard_port
    })
}

pub fn guard_hostname_for_host(headers: &Headers) -> Result<String, String> {
    // If using the local Guard instance, we should change the Guard pathname based on the server host. E.g., if the server host is https://lighthouse.example.com, the Guard URL would be https://lighthouse.example.com/guard, as https://127.0.0.1/guard obviously doesn't work.
    let mut host: String = headers.headers_map.get("host").unwrap().to_string();
    if (host.starts_with("https://") == false && host.starts_with("http://") == false) {
        // Firefox does not include protocol in its Host header (e.g. example.lighthouse.com, 127.0.0.1:5454, www.google.com). We need to add an artificial protocol so the URL parser will operate.
        host = format!("https://{}", host);
    }

    let host_parsed = url::Url::parse(&host).unwrap();
    let hostname = host_parsed.host_str().expect("Failed to parse host string").to_string();

    if (GUARD_HOSTNAME_TO_USE.use_local_guard == true && // If we're using a local Guard instance, we need to check the relevant Config.metadata is provided.
        CONFIG_VALUE.metadata.is_none() == true && // Check Config.metadata is provided.
        CONFIG_VALUE.metadata.clone().expect("Missing Config.metadata").hostname.is_none() == true) { // Check Config.metadata.hostname is provided.
        
        return Err("Config.metadata.hostname is unspecified while using local Guard instance. Please specify Config.metadata.hostname in your configuration.".into());
    }

    log::debug!("hostname: {} || config.metadata: {:?}", hostname, CONFIG_VALUE.metadata);
    if (GUARD_HOSTNAME_TO_USE.use_local_guard == true && // Ensure we're meant to be using the local Guard instance.
        CONFIG_VALUE.metadata.clone().unwrap().hostname.unwrap().contains(&hostname) == true) { // Check the untrusted host string is a legitimate URL within the configuration.
        
        let mut url = url::Url::parse(&host).unwrap();
        url.set_path("/guard");
        url.set_scheme("https").expect("Failed to set HTTPS");
        url.set_port(Some(GUARD_HOSTNAME_TO_USE.local_port.unwrap())).expect("Failed to set port");
        Ok(url.as_str().to_string())
    } else {
        Ok(GUARD_HOSTNAME_TO_USE.hostname.clone().unwrap())
    }
}

pub async fn start_guard(port: u16) {
    let guard_executable_dir = guard_executable_directory();

    std::thread::spawn(move || {
        // TODO: Add startup/error handling here.
        log::info!("Starting guard on port {}... ({})", port, guard_executable_dir);

        Command::new(guard_executable_directory())
            .args(["--port", &port.to_string()])
            .output()
            .expect("Failed to execute Guard process");

        log::info!("Started Guard ({}) on port {}", guard_executable_dir, port);
    });
}

pub fn guard_executable_directory() -> String {
    let mut path: PathBuf = env::current_dir().expect("Failed to get current directory");
    path.push("../");
    path.push(".guard");
    path.push("guard-server");

    return path.display().to_string();
}