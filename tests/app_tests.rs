use std::env;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[test]
fn hurl_tests() {
    // Set the HURL_url environment variable
    env::set_var("HURL_url", "http://127.0.0.1:8000");

    // Start cargo in the background
    let mut cargo_process = Command::new("cargo")
        .arg("run")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start cargo");

    // capture the PID
    let cargo_pid = cargo_process.id();

    // wait for rocket
    if let Some(ref mut stdout) = cargo_process.stdout {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    print!("{}", line);
                    if line.contains("Rocket has launched") {
                        break;
                    }
                }
                Err(e) => {
                    cargo_process
                        .kill()
                        .unwrap_or_else(|_| panic!("Failed to kill cargo: pid = {}", cargo_pid));
                    panic!("Failed to read line: {}", e);
                }
            }
        }
    } else {
        cargo_process
            .kill()
            .unwrap_or_else(|_| panic!("Failed to kill cargo: pid = {}", cargo_pid));
        panic!("No stdout to parse...");
    }

    // Define the test files
    let tests = vec![
        "tests/users.hurl",
        "tests/invites.hurl",
        "tests/permissions.hurl",
        "tests/sessions.hurl",
        "tests/genres.hurl",
        "tests/artists.hurl",
    ];

    // Run all application tests
    for test in tests {
        let status = Command::new("hurl")
            .arg("--very-verbose")
            .arg(test)
            .status()
            .expect("Failed to execute hurl command");

        // Check if the test failed
        if !status.success() {
            cargo_process
                .kill()
                .unwrap_or_else(|_| panic!("Failed to kill cargo: pid = {}", cargo_pid));
            panic!("Test failed: {}", test);
        }
    }

    // Kill the cargo process
    cargo_process
        .kill()
        .unwrap_or_else(|_| panic!("Failed to kill cargo: pid = {}", cargo_pid));
}
