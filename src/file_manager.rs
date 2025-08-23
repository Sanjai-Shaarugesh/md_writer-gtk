use gtk::prelude::*;
use gtk::{FileDialog, Window};
use std::path::PathBuf;
use std::fs::{self, read_to_string};

pub struct FileManager {
    recent_files: Vec<PathBuf>,
}

impl FileManager {
    pub fn new() -> Self {
        Self {
            recent_files: Vec::new(),
        }
    }

    pub async fn open_file_dialog(&self, parent: &Window) -> Result<Option<(PathBuf, String)>, Box<dyn std::error::Error>> {
        let dialog = FileDialog::builder()
            .title("Open Markdown File")
            .modal(true)
            .build();

        // Create file filter for markdown files
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("Markdown Files"));
        filter.add_pattern("*.md");
        filter.add_pattern("*.markdown");
        filter.add_pattern("*.txt");
        
        let filter_list = gtk::gio::ListStore::new::<gtk::FileFilter>();
        filter_list.append(&filter);
        dialog.set_filters(Some(&filter_list));
        dialog.set_default_filter(Some(&filter));

        match dialog.open_future(Some(parent)).await {
            Ok(file) => {
                if let Some(path) = file.path() {
                    let content = read_to_string(&path)?;
                    Ok(Some((path, content)))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None), // User cancelled
        }
    }

    pub async fn save_file_dialog(&self, parent: &Window, content: &str) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        let dialog = FileDialog::builder()
            .title("Save Markdown File")
            .modal(true)
            .build();

        // Set default extension
        let file = gtk::gio::File::for_path("untitled.md");
        dialog.set_initial_file(Some(&file));

        // Create file filter for markdown files
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("Markdown Files"));
        filter.add_pattern("*.md");
        filter.add_pattern("*.markdown");

        let filter_list = gtk::gio::ListStore::new::<gtk::FileFilter>();
        filter_list.append(&filter);
        dialog.set_filters(Some(&filter_list));
        dialog.set_default_filter(Some(&filter));

        match dialog.save_future(Some(parent)).await {
            Ok(file) => {
                if let Some(path) = file.path() {
                    fs::write(&path, content)?;
                    Ok(Some(path))
                } else {
                    Ok(None)
                }
            }
            Err(_) => Ok(None), // User cancelled
        }
    }

    pub async fn save_file(&self, path: &PathBuf, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(path, content)?;
        Ok(())
    }

    pub fn add_recent_file(&mut self, path: PathBuf) {
        // Remove if already exists
        self.recent_files.retain(|p| p != &path);
        
        // Add to front
        self.recent_files.insert(0, path);
        
        // Keep only last 10 files
        if self.recent_files.len() > 10 {
            self.recent_files.truncate(10);
        }
    }

    pub fn get_recent_files(&self) -> &[PathBuf] {
        &self.recent_files
    }

    pub async fn load_recent_files(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_dir) = get_config_dir() {
            let app_dir = config_dir.join("markdown-editor");
            let recent_file = app_dir.join("recent_files.json");
            
            if recent_file.exists() {
                let content = read_to_string(recent_file)?;
                if let Ok(files) = serde_json::from_str::<Vec<PathBuf>>(&content) {
                    self.recent_files = files.into_iter()
                        .filter(|path| path.exists()) // Only keep existing files
                        .collect();
                }
            }
        }
        Ok(())
    }

    pub async fn save_recent_files(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config_dir) = get_config_dir() {
            let app_dir = config_dir.join("markdown-editor");
            fs::create_dir_all(&app_dir)?;

            let recent_file = app_dir.join("recent_files.json");
            let content = serde_json::to_string_pretty(&self.recent_files)?;
            fs::write(recent_file, content)?;
        }
        Ok(())
    }
}

fn get_config_dir() -> Option<PathBuf> {
    // Try XDG_CONFIG_HOME first, then fall back to ~/.config on Unix, or %APPDATA% on Windows
    if let Some(config_home) = std::env::var_os("XDG_CONFIG_HOME") {
        Some(PathBuf::from(config_home))
    } else if let Some(home) = std::env::var_os("HOME") {
        Some(PathBuf::from(home).join(".config"))
    } else if let Some(appdata) = std::env::var_os("APPDATA") {
        Some(PathBuf::from(appdata))
    } else {
        None
    }
}