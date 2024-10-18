//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we set up an environment variable called `TEST_FOO`
    // and assign it the current Unix timestamp.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Print the cargo command to set the environment variable
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // In tests8, we enable the "pass" feature to make the testcase return early
    // We can tell Cargo to enable this feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

