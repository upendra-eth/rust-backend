use std::env;

fn main() {
    // Access the MY_APP_ENV environment variable
    let my_app_env = env::var("MY_APP_ENV").unwrap_or("development".to_string());

    if my_app_env == "development" {
        // Development-specific code
        println!("Running in development environment.");
    } else if my_app_env == "production" {
        // Production-specific code
        println!("Running in production environment.");
    } else {
        // Default behavior for unknown environment
        println!("Unknown or unspecified environment.");
    }
}
