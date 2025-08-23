use gtk::prelude::*;
use gtk::{Button, ToggleButton, TextBuffer, Label};
use adw::{NavigationSplitView, ActionRow, WindowTitle};

pub fn setup_sidebar_controls(
    split_view: &NavigationSplitView,
    sidebar_toggle_button: &ToggleButton,
    show_sidebar_button: &Button,
) {
    // Handle sidebar toggle button
    sidebar_toggle_button.connect_toggled({
        let split_view = split_view.clone();
        move |button| {
            let active = button.is_active();
            split_view.set_show_content(!active);
        }
    });

    // Handle show sidebar button (for when sidebar is collapsed)
    show_sidebar_button.connect_clicked({
        let split_view = split_view.clone();
        move |_| {
            split_view.set_show_content(false);
        }
    });

    // Handle responsive behavior
    split_view.connect_collapsed_notify({
        let split_view = split_view.clone();
        let sidebar_toggle_button = sidebar_toggle_button.clone();
        let show_sidebar_button = show_sidebar_button.clone();
        move |_| {
            let collapsed = split_view.is_collapsed();
            sidebar_toggle_button.set_visible(!collapsed);
            show_sidebar_button.set_visible(collapsed);
        }
    });
}

pub fn setup_category_navigation(
    category_documents: &ActionRow,
    category_projects: &ActionRow,
    category_favorites: &ActionRow,
    main_title: &WindowTitle,
    text_buffer: &TextBuffer,
    status_label: &Label,
) {
    // Documents category
    category_documents.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("Documents");
            main_title.set_subtitle("Browse your documents");
            text_buffer.set_text("# Documents\n\nYour document files will appear here.\n\nThis is a placeholder for the documents category.");
            status_label.set_text("Documents category selected");
        }
    });

    // Projects category
    category_projects.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("Projects");
            main_title.set_subtitle("Browse your projects");
            text_buffer.set_text("# Projects\n\nYour project documentation will appear here.\n\n## Current Projects\n\n- Project Alpha\n- Project Beta\n- Project Gamma");
            status_label.set_text("Projects category selected");
        }
    });

    // Favorites category
    category_favorites.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("Favorites");
            main_title.set_subtitle("Your starred documents");
            text_buffer.set_text("# Favorites\n\nYour favorite and starred documents will appear here.\n\n⭐ Add documents to favorites by clicking the star icon.");
            status_label.set_text("Favorites category selected");
        }
    });
}

pub fn setup_recent_files(
    recent_file_1: &ActionRow,
    recent_file_2: &ActionRow,
    recent_file_3: &ActionRow,
    main_title: &WindowTitle,
    text_buffer: &TextBuffer,
    status_label: &Label,
) {
    // Recent file 1 - README.md
    recent_file_1.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("README.md");
            main_title.set_subtitle("Project documentation");
            text_buffer.set_text("# README\n\nThis is a sample README file.\n\n## Installation\n\n1. Download the application\n2. Follow the setup instructions\n\n## Usage\n\nRun the application and enjoy!");
            status_label.set_text("README.md opened");
        }
    });

    // Recent file 2 - CHANGELOG.md
    recent_file_2.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("CHANGELOG.md");
            main_title.set_subtitle("Version history");
            text_buffer.set_text("# Changelog\n\n## [1.0.0] - 2024-01-15\n\n### Added\n- Initial release\n- Basic functionality\n- User interface\n\n### Fixed\n- Various bug fixes");
            status_label.set_text("CHANGELOG.md opened");
        }
    });

    // Recent file 3 - TODO.md
    recent_file_3.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("TODO.md");
            main_title.set_subtitle("Task list");
            text_buffer.set_text("# TODO List\n\n## High Priority\n\n- [ ] Fix critical bug in main module\n- [ ] Update documentation\n- [ ] Add unit tests\n\n## Medium Priority\n\n- [ ] Improve UI design\n- [ ] Add new features\n- [ ] Optimize performance");
            status_label.set_text("TODO.md opened");
        }
    });
}