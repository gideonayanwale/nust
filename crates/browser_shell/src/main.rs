use browser_shell::pipeline::run_pipeline;

fn main() -> Result<(), String> {
    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "https://example.com".to_string());

    let output = run_pipeline(&url)?;

    println!("NUST minimal renderer output for {url}:");
    for command in output.commands {
        println!("{command:?}");
    }

    if std::env::var("NUST_SHOW_WINDOW").ok().as_deref() == Some("1") {
        println!("Window bootstrap placeholder enabled.");
    }

    Ok(())
}
