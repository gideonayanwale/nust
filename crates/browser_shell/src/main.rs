use browser_shell::automation_registry::AutomationRegistry;
use browser_shell::bookmark_manager::BookmarkManagerService;
use browser_shell::capability_matrix::CapabilityMatrix;
use browser_shell::download_manager::DownloadManagerService;
use browser_shell::extension_library::ExtensionLibraryService;
use browser_shell::history_manager::HistoryManagerService;
use browser_shell::home_page::HomePage;
use browser_shell::new_tab_page::NewTabPage;
use browser_shell::pipeline::run_pipeline;
use browser_shell::session_manager::{SessionManagerService, SessionMode};
use browser_shell::settings_system::{BrowserSettings, PerformanceMode, SkinMode};
use browser_shell::tab_manager::TabManagerService;
use browser_shell::tab_process_registry::TabProcessRegistry;
use browser_shell::task_manager::TaskManagerService;
use browser_shell::upload_manager::UploadManagerService;

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
    if let Some(idx) = args.iter().position(|arg| arg == "--disable-downloads") {
        settings.downloads_enabled = false;
        args.remove(idx);
    }
    if let Some(idx) = args.iter().position(|arg| arg == "--disable-extensions") {
        settings.extension_library_enabled = false;
        settings.global_extension_runtime_enabled = false;
        args.remove(idx);
    }
    if let Some(idx) = args.iter().position(|arg| arg == "--disable-automation") {
        settings.automation_registry_enabled = false;
        settings.automation_autopilot_enabled = false;
        args.remove(idx);
    }
    if let Some(idx) = args.iter().position(|arg| arg == "--disable-task-manager") {
        settings.task_manager_enabled = false;
        args.remove(idx);
    }
    if let Some(idx) = args.iter().position(|arg| arg == "--disable-uploads") {
        settings.upload_on_demand_enabled = false;
        args.remove(idx);
    }
    if let Some(idx) = args.iter().position(|arg| arg == "--upload-latency-ms") {
        if let Some(raw) = args.get(idx + 1) {
            if let Ok(latency) = raw.parse::<u64>() {
                settings.upload_latency_ms = latency;
            }
        }
        args.drain(idx..=(idx + 1).min(args.len() - 1));
    }

    let mut args = args.into_iter();

    match args.next().as_deref() {
        Some("--home") => {
            println!("{}", HomePage::with_settings(&settings).render_html());
            Ok(())
        }
        Some("--new-tab") => {
            let query = args.next().unwrap_or_default();
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
            showcase_modern_features(&settings);
            Ok(())
        }
        Some("--download-tray") => {
            println!("{}", run_download_tray_demo(&settings));
            Ok(())
        }
        Some("--extensions") => {
            println!("{}", run_extension_library_demo(&settings));
            Ok(())
        }
        Some("--task-manager") => {
            println!("{}", run_task_manager_demo(&settings));
            Ok(())
        }
        Some("--upload-tray") => {
            println!("{}", run_upload_tray_demo(&settings));
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
}

fn showcase_modern_features(settings: &BrowserSettings) {
    let mut tabs = TabManagerService::default();
    let mut process_registry = TabProcessRegistry::default();
    let mut automation_registry = AutomationRegistry::default();
    let mut downloads = DownloadManagerService::default();
    let mut uploads = UploadManagerService::default();
    let mut extensions = ExtensionLibraryService::default();
    let task_manager = TaskManagerService;
    let tab = tabs.open_new_tab("browser features");
    let process = process_registry.register_thread_process_for_tab(tab.id);
    tabs.attach_process(tab.id, process);
    tabs.pin_tab(tab.id);
    tabs.mute_tab(tab.id);
    if settings.automation_registry_enabled {
        automation_registry.register_default_workflow(tab.id, "https://example.com");
    }
    downloads.enqueue(
        "browser-shell-preview.zip",
        "https://cdn.nust.dev/preview.zip",
    );
    downloads.start_all(settings);
    uploads.queue_on_demand(
        "session-recording.webm",
        "video/webm",
        "https://upload.nust.dev/session",
    );
    uploads.queue_on_demand(
        "project-archive.tar.zst",
        "application/zstd",
        "https://upload.nust.dev/archive",
    );
    uploads.run_on_demand(settings);
    extensions.install("privacy-guard", "Privacy Guard");
    extensions.install("dev-hud", "Developer HUD");
    extensions.activate_globally(settings);

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
    println!("- {}", downloads.tray_summary());
    println!("- {}", uploads.tray_summary());
    println!("- {}", extensions.summary());
    if let Some(snapshot) = task_manager.snapshot(&tabs, &downloads, settings) {
        println!(
            "- {}",
            TaskManagerService::render(&snapshot).replace('\n', " | ")
        );
    }
}

fn run_download_tray_demo(settings: &BrowserSettings) -> String {
    let mut downloads = DownloadManagerService::default();
    downloads.enqueue(
        "nust-installer.exe",
        "https://cdn.nust.dev/nust-installer.exe",
    );
    downloads.enqueue(
        "design-assets.pack",
        "https://cdn.nust.dev/design-assets.pack",
    );
    downloads.start_all(settings);
    downloads.tray_summary()
}

fn run_extension_library_demo(settings: &BrowserSettings) -> String {
    let mut extensions = ExtensionLibraryService::default();
    extensions.install("password-vault", "Password Vault");
    extensions.install("workspace-sync", "Workspace Sync");
    extensions.activate_globally(settings);
    extensions.summary()
}

fn run_task_manager_demo(settings: &BrowserSettings) -> String {
    let mut tabs = TabManagerService::default();
    let mut registry = TabProcessRegistry::default();
    let mut downloads = DownloadManagerService::default();
    let task_manager = TaskManagerService;

    let first = tabs.open_new_tab("nust");
    tabs.attach_process(first.id, registry.register_thread_process_for_tab(first.id));
    let second = tabs.open_new_tab("extensions");
    tabs.attach_process(
        second.id,
        registry.register_thread_process_for_tab(second.id),
    );

    downloads.enqueue("core-update.bin", "https://cdn.nust.dev/core-update.bin");
    downloads.start_all(settings);

    task_manager
        .snapshot(&tabs, &downloads, settings)
        .map(|snapshot| TaskManagerService::render(&snapshot))
        .unwrap_or_else(|| "Task manager is disabled by settings".to_string())
}

fn run_upload_tray_demo(settings: &BrowserSettings) -> String {
    let mut uploads = UploadManagerService::default();
    uploads.queue_on_demand(
        "design.psd",
        "image/vnd.adobe.photoshop",
        "https://upload.nust.dev/design",
    );
    uploads.queue_on_demand(
        "analytics.parquet",
        "application/vnd.apache.parquet",
        "https://upload.nust.dev/data",
    );
    uploads.queue_on_demand(
        "backup.iso",
        "application/x-iso9660-image",
        "https://upload.nust.dev/system",
    );
    uploads.run_on_demand(settings);
    uploads.tray_summary()
}
