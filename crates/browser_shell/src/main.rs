<<<<<<< HEAD
<<<<<<< HEAD
use browser_shell::automation_registry::AutomationRegistry;
use browser_shell::bookmark_manager::BookmarkManagerService;
use browser_shell::capability_matrix::CapabilityMatrix;
use browser_shell::history_manager::HistoryManagerService;
use browser_shell::home_page::HomePage;
use browser_shell::new_tab_page::NewTabPage;
use browser_shell::pipeline::run_pipeline;
use browser_shell::session_manager::{SessionManagerService, SessionMode};
use browser_shell::settings_system::{BrowserSettings, PerformanceMode, SkinMode};
use browser_shell::tab_manager::TabManagerService;
use browser_shell::tab_process_registry::TabProcessRegistry;

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1).collect::<Vec<_>>();
    let mut settings = BrowserSettings::default();

    if let Some(idx) = args.iter().position(|arg| arg == "--skin") {
        if let Some(skin) = args.get(idx + 1) {
            settings.skin_mode = match skin.as_str() {
                "aurora" => SkinMode::NustAurora,
                _ => SkinMode::EdgeChromeBraveFusion,
            };
        }
        args.drain(idx..=(idx + 1).min(args.len() - 1));
    }

    let mut args = args.into_iter();

    match args.next().as_deref() {
        Some("--home") => {
            println!("{}", HomePage::with_settings(&settings).render_html());
=======
=======
use browser_shell::bookmark_manager::BookmarkManagerService;
use browser_shell::capability_matrix::CapabilityMatrix;
use browser_shell::history_manager::HistoryManagerService;
>>>>>>> main
use browser_shell::home_page::HomePage;
use browser_shell::new_tab_page::NewTabPage;
use browser_shell::pipeline::run_pipeline;
use browser_shell::session_manager::{SessionManagerService, SessionMode};
use browser_shell::settings_system::{BrowserSettings, PerformanceMode};
use browser_shell::tab_manager::TabManagerService;
=======
use browser_shell::home_page::HomePage;
use browser_shell::new_tab_page::NewTabPage;
use browser_shell::pipeline::run_pipeline;
>>>>>>> main

fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("--home") => {
            println!("{}", HomePage::default().render_html());
>>>>>>> main
            Ok(())
        }
        Some("--new-tab") => {
            let query = args.next().unwrap_or_default();
<<<<<<< HEAD
            println!(
                "{}",
                NewTabPage::with_settings(query, &settings).render_html()
            );
            Ok(())
        }
        Some("--incognito") => {
            let mut session = SessionManagerService::default();
            session.set_mode(SessionMode::Incognito);
            println!("Session mode: {:?}", session.state().mode);
            println!("Restore on startup: {}", session.state().restore_on_startup);
            Ok(())
        }
        Some("--feature-report") => {
            if let Some(mode_flag) = args.next().as_deref() {
                match mode_flag {
                    "balanced" => settings.apply_mode(PerformanceMode::Balanced),
                    "compat" => settings.apply_mode(PerformanceMode::MaximumCompatibility),
                    _ => settings.apply_mode(PerformanceMode::Lightweight),
                }
            }
            let report = CapabilityMatrix::baseline(&settings).to_report();
            println!("{report}");
            Ok(())
        }
        Some("--showcase-modern-features") => {
            showcase_modern_features();
=======
            println!("{}", NewTabPage::new(query).render_html());
>>>>>>> main
            Ok(())
        }
<<<<<<< HEAD
        Some("--incognito") => {
            let mut session = SessionManagerService::default();
            session.set_mode(SessionMode::Incognito);
            println!("Session mode: {:?}", session.state().mode);
            println!("Restore on startup: {}", session.state().restore_on_startup);
            Ok(())
        }
        Some("--feature-report") => {
            let mut settings = BrowserSettings::default();
            if let Some(mode_flag) = args.next().as_deref() {
                match mode_flag {
                    "balanced" => settings.apply_mode(PerformanceMode::Balanced),
                    "compat" => settings.apply_mode(PerformanceMode::MaximumCompatibility),
                    _ => settings.apply_mode(PerformanceMode::Lightweight),
                }
            }
            let report = CapabilityMatrix::baseline(&settings).to_report();
            println!("{report}");
            Ok(())
        }
        Some("--showcase-modern-features") => {
            showcase_modern_features();
            Ok(())
        }
=======
>>>>>>> main
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
<<<<<<< HEAD
}

fn showcase_modern_features() {
    let mut tabs = TabManagerService::default();
<<<<<<< HEAD
    let mut process_registry = TabProcessRegistry::default();
    let mut automation_registry = AutomationRegistry::default();
    let tab = tabs.open_new_tab("browser features");
    let process = process_registry.register_thread_process_for_tab(tab.id);
    tabs.attach_process(tab.id, process);
    tabs.pin_tab(tab.id);
    tabs.mute_tab(tab.id);
    automation_registry.register_default_workflow(tab.id, "https://example.com");
=======
    let tab = tabs.open_new_tab("browser features");
    tabs.pin_tab(tab.id);
    tabs.mute_tab(tab.id);
>>>>>>> main

    let mut bookmarks = BookmarkManagerService::default();
    bookmarks.add("https://example.com", "Example", Some("Work".to_string()));

    let mut history = HistoryManagerService::default();
    history.visit("https://example.com", "Example", 1_700_000_000_000);

    println!("Modern browser-style features snapshot:");
    println!(
        "- tabs: {} (first pinned: {}, muted: {})",
        tabs.tabs().len(),
        tabs.tabs()[0].pinned,
        tabs.tabs()[0].muted
    );
    println!("- bookmarks: {}", bookmarks.all().len());
    println!("- history entries: {}", history.recent(10).len());
    println!("- modes: normal + incognito supported");
<<<<<<< HEAD
    println!(
        "- tab thread-process registry: {} active entry",
        tabs.tabs()
            .iter()
            .filter(|tab| tab.process.is_some())
            .count()
    );
    println!(
        "- automation workflows attached: {}",
        automation_registry.workflow_count()
    );
=======
>>>>>>> main
=======
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
>>>>>>> main
}
