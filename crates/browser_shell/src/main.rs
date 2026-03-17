use browser_shell::home_page::HomePage;
use browser_shell::new_tab_page::NewTabPage;
use browser_shell::pipeline::run_pipeline;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("--home") => {
            println!("{}", HomePage::default().render_html());
            Ok(())
        }
        Some("--new-tab") => {
            let query = args.next().unwrap_or_default();
            println!("{}", NewTabPage::new(query).render_html());
            Ok(())
        }
        Some(url) => {
            let output = run_pipeline(url)?;
            println!("NUST minimal renderer output for {url}:");
            for command in output.commands {
                println!("{command:?}");
            }
            Ok(())
        }
        None => {
            let default_url = "https://example.com";
            let output = run_pipeline(default_url)?;
            println!("NUST minimal renderer output for {default_url}:");
            for command in output.commands {
                println!("{command:?}");
            }
            Ok(())
        }
    }
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
