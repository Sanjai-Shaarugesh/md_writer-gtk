use adw::prelude::{ActionRowExt, AdwApplicationWindowExt, NavigationPageExt, PreferencesGroupExt, PreferencesRowExt};
use gtk::{glib, prelude::*};
use gtk::{TextView, Button, SearchEntry, Label, TextBuffer, Orientation};
use adw::{Application, ApplicationWindow, NavigationSplitView, ActionRow, WindowTitle};
use std::cell::RefCell;
use std::rc::Rc;

use crate::markdown_renderer::MarkdownRenderer;
use crate::sidebar_controls::{setup_category_navigation, setup_recent_files};
use crate::file_manager::FileManager;

pub fn build_ui(app: &Application) {
    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Markdown Editor")
        .default_width(1200)
        .default_height(800)
        .build();

    // Create markdown renderer
    let markdown_renderer = Rc::new(MarkdownRenderer::new());
    let file_manager = Rc::new(RefCell::new(FileManager::new()));

    // Create split view for main layout
    let split_view = NavigationSplitView::new();
    split_view.set_min_sidebar_width(300.0);
    split_view.set_max_sidebar_width(400.0);
    split_view.set_sidebar_width_fraction(0.25);

    // Create sidebar content
    let (sidebar_page, sidebar_elements) = create_sidebar();
    split_view.set_sidebar(Some(&sidebar_page));

    // Create main content area
    let (content_page, main_elements) = create_main_content(&markdown_renderer);
    split_view.set_content(Some(&content_page));

    // Set window content
    window.set_content(Some(&split_view));

    // State management
    let preview_visible = Rc::new(RefCell::new(true));
    let current_file = Rc::new(RefCell::new(None::<std::path::PathBuf>));

    // Set up preview updates
    let update_preview = {
        let markdown_renderer = markdown_renderer.clone();
        let preview_buffer = main_elements.preview_buffer.clone();
        move |text: &str| {
            markdown_renderer.render_markdown(&preview_buffer, text);
        }
    };

    // Set up text change handlers
    setup_text_handlers(
        &main_elements.text_buffer,
        &main_elements.word_count_label,
        &main_elements.status_label,
        update_preview,
    );

    // Set up button handlers
    setup_button_handlers(
        &main_elements.new_button,
        &main_elements.open_button,
        &main_elements.save_button,
        &main_elements.format_button,
        &main_elements.view_mode_button,
        &main_elements.text_buffer,
        &main_elements.text_view,
        &main_elements.main_title,
        &main_elements.status_label,
        &main_elements.main_paned,
        &main_elements.preview_scroll,
        preview_visible,
        current_file.clone(),
        file_manager.clone(),
    );

    // Set up sidebar handlers
    setup_category_navigation(
        &sidebar_elements.documents_row,
        &sidebar_elements.projects_row,
        &sidebar_elements.favorites_row,
        &main_elements.main_title,
        &main_elements.text_buffer,
        &main_elements.status_label,
    );

    setup_recent_files(
        &sidebar_elements.recent_file_1,
        &sidebar_elements.recent_file_2,
        &sidebar_elements.recent_file_3,
        &main_elements.main_title,
        &main_elements.text_buffer,
        &main_elements.status_label,
    );

    // Set up responsive behavior
    setup_responsive_behavior(&split_view);

    // Initialize with sample content
    let sample_content = include_str!("../sample_content.md");
    main_elements.text_buffer.set_text(sample_content);

    window.present();
}

struct SidebarElements {
    documents_row: ActionRow,
    projects_row: ActionRow,
    favorites_row: ActionRow,
    recent_file_1: ActionRow,
    recent_file_2: ActionRow,
    recent_file_3: ActionRow,
}

struct MainElements {
    text_buffer: TextBuffer,
    text_view: TextView,
    preview_buffer: TextBuffer,
    preview_scroll: gtk::ScrolledWindow,
    main_paned: gtk::Paned,
    main_title: WindowTitle,
    status_label: Label,
    word_count_label: Label,
    new_button: Button,
    open_button: Button,
    save_button: Button,
    format_button: Button,
    view_mode_button: Button,
}

fn create_sidebar() -> (adw::NavigationPage, SidebarElements) {
    let sidebar_box = gtk::Box::new(Orientation::Vertical, 12);
    sidebar_box.set_margin_top(12);
    sidebar_box.set_margin_bottom(12);
    sidebar_box.set_margin_start(12);
    sidebar_box.set_margin_end(12);

    // Create search entry
    let search_entry = SearchEntry::new();
    search_entry.set_placeholder_text(Some("Search documents..."));
    sidebar_box.append(&search_entry);

    // Create categories group
    let categories_group = adw::PreferencesGroup::new();
    categories_group.set_title("Categories");

    let documents_row = ActionRow::new();
    documents_row.set_title("Documents");
    documents_row.set_subtitle("Text files and notes");
    documents_row.add_prefix(&gtk::Image::from_icon_name("folder-documents-symbolic"));
    documents_row.set_activatable(true);

    let projects_row = ActionRow::new();
    projects_row.set_title("Projects");
    projects_row.set_subtitle("Project documentation");
    projects_row.add_prefix(&gtk::Image::from_icon_name("folder-symbolic"));
    projects_row.set_activatable(true);

    let favorites_row = ActionRow::new();
    favorites_row.set_title("Favorites");
    favorites_row.set_subtitle("Starred documents");
    favorites_row.add_prefix(&gtk::Image::from_icon_name("starred-symbolic"));
    favorites_row.set_activatable(true);

    categories_group.add(&documents_row);
    categories_group.add(&projects_row);
    categories_group.add(&favorites_row);

    // Create recent files group
    let recent_group = adw::PreferencesGroup::new();
    recent_group.set_title("Recent Files");

    let recent_file_1 = ActionRow::new();
    recent_file_1.set_title("README.md");
    recent_file_1.set_subtitle("Project documentation");
    recent_file_1.add_prefix(&gtk::Image::from_icon_name("text-markdown-symbolic"));
    recent_file_1.set_activatable(true);

    let recent_file_2 = ActionRow::new();
    recent_file_2.set_title("CHANGELOG.md");
    recent_file_2.set_subtitle("Version history");
    recent_file_2.add_prefix(&gtk::Image::from_icon_name("text-markdown-symbolic"));
    recent_file_2.set_activatable(true);

    let recent_file_3 = ActionRow::new();
    recent_file_3.set_title("TODO.md");
    recent_file_3.set_subtitle("Task list");
    recent_file_3.add_prefix(&gtk::Image::from_icon_name("text-markdown-symbolic"));
    recent_file_3.set_activatable(true);

    recent_group.add(&recent_file_1);
    recent_group.add(&recent_file_2);
    recent_group.add(&recent_file_3);

    // Add all to sidebar
    sidebar_box.append(&categories_group);
    sidebar_box.append(&recent_group);

    let sidebar_scroll = gtk::ScrolledWindow::new();
    sidebar_scroll.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Automatic);
    sidebar_scroll.set_child(Some(&sidebar_box));

    let sidebar_page = adw::NavigationPage::new(&sidebar_scroll, "sidebar");
    sidebar_page.set_title("Files");

    let elements = SidebarElements {
        documents_row,
        projects_row,
        favorites_row,
        recent_file_1,
        recent_file_2,
        recent_file_3,
    };

    (sidebar_page, elements)
}

fn create_main_content(markdown_renderer: &Rc<MarkdownRenderer>) -> (adw::NavigationPage, MainElements) {
    // Create toolbar view
    let toolbar_view = adw::ToolbarView::new();

    // Create header bar
    let header_bar = adw::HeaderBar::new();
    
    // Create window title
    let main_title = WindowTitle::new("Markdown Editor", "Ready");
    header_bar.set_title_widget(Some(&main_title));

    // Header buttons
    let new_button = Button::from_icon_name("document-new-symbolic");
    new_button.set_tooltip_text(Some("New Document (Ctrl+N)"));
    new_button.add_css_class("flat");

    let open_button = Button::from_icon_name("document-open-symbolic");
    open_button.set_tooltip_text(Some("Open Document (Ctrl+O)"));
    open_button.add_css_class("flat");

    let save_button = Button::from_icon_name("document-save-symbolic");
    save_button.set_tooltip_text(Some("Save Document (Ctrl+S)"));
    save_button.add_css_class("flat");

    let view_mode_button = Button::from_icon_name("sidebar-show-right-symbolic");
    view_mode_button.set_tooltip_text(Some("Toggle Preview"));
    view_mode_button.add_css_class("flat");

    let format_button = Button::from_icon_name("format-text-bold-symbolic");
    format_button.set_tooltip_text(Some("Toggle Font Style"));
    format_button.add_css_class("flat");

    // Add buttons to header
    header_bar.pack_start(&new_button);
    header_bar.pack_start(&open_button);
    header_bar.pack_start(&save_button);
    header_bar.pack_end(&view_mode_button);
    header_bar.pack_end(&format_button);

    toolbar_view.add_top_bar(&header_bar);

    // Create main content area with paned view
    let main_paned = gtk::Paned::new(Orientation::Horizontal);
    main_paned.set_position(600);
    main_paned.set_resize_start_child(true);
    main_paned.set_resize_end_child(true);

    // Create text editor
    let text_buffer = TextBuffer::new(None::<&gtk::TextTagTable>);
    let text_view = TextView::with_buffer(&text_buffer);
    text_view.set_wrap_mode(gtk::WrapMode::Word);
    text_view.set_left_margin(20);
    text_view.set_right_margin(20);
    text_view.set_top_margin(20);
    text_view.set_bottom_margin(20);
    text_view.set_accepts_tab(true);
    text_view.set_monospace(true);

    // Create editor scroll window
    let editor_scroll = gtk::ScrolledWindow::new();
    editor_scroll.set_hscrollbar_policy(gtk::PolicyType::Never);
    editor_scroll.set_vscrollbar_policy(gtk::PolicyType::Automatic);
    editor_scroll.set_child(Some(&text_view));
    editor_scroll.add_css_class("editor-pane");

    // Create preview components
    let preview_buffer = TextBuffer::new(Some(&markdown_renderer.tag_table));
    let preview_view = TextView::with_buffer(&preview_buffer);
    preview_view.set_editable(false);
    preview_view.set_cursor_visible(false);
    preview_view.set_wrap_mode(gtk::WrapMode::Word);
    preview_view.set_left_margin(20);
    preview_view.set_right_margin(20);
    preview_view.set_top_margin(20);
    preview_view.set_bottom_margin(20);
    preview_view.add_css_class("preview");

    let preview_scroll = gtk::ScrolledWindow::new();
    preview_scroll.set_hscrollbar_policy(gtk::PolicyType::Never);
    preview_scroll.set_vscrollbar_policy(gtk::PolicyType::Automatic);
    preview_scroll.set_child(Some(&preview_view));
    preview_scroll.add_css_class("preview-pane");

    // Add editor and preview to paned view
    main_paned.set_start_child(Some(&editor_scroll));
    main_paned.set_end_child(Some(&preview_scroll));

    toolbar_view.set_content(Some(&main_paned));

    // Create status bar
    let status_bar = gtk::Box::new(Orientation::Horizontal, 0);
    status_bar.add_css_class("statusbar");

    let status_label = Label::new(Some("Ready"));
    status_label.set_margin_start(10);
    status_label.set_halign(gtk::Align::Start);

    let word_count_label = Label::new(Some("0 words"));
    word_count_label.set_margin_end(10);
    word_count_label.set_halign(gtk::Align::End);
    word_count_label.set_hexpand(true);

    status_bar.append(&status_label);
    status_bar.append(&word_count_label);

    toolbar_view.add_bottom_bar(&status_bar);

    // Create main content page
    let content_page = adw::NavigationPage::new(&toolbar_view, "editor");

    let elements = MainElements {
        text_buffer,
        text_view,
        preview_buffer,
        preview_scroll,
        main_paned,
        main_title,
        status_label,
        word_count_label,
        new_button,
        open_button,
        save_button,
        format_button,
        view_mode_button,
    };

    (content_page, elements)
}

fn setup_text_handlers(
    text_buffer: &TextBuffer,
    word_count_label: &Label,
    status_label: &Label,
    update_preview: impl Fn(&str) + 'static,
) {
    text_buffer.connect_changed({
        let word_count_label = word_count_label.clone();
        let status_label = status_label.clone();
        move |buffer| {
            let start = buffer.start_iter();
            let end = buffer.end_iter();
            let text = buffer.text(&start, &end, false);
            
            let words = if text.trim().is_empty() {
                0
            } else {
                text.split_whitespace().count()
            };

            word_count_label.set_text(&format!("{} words", words));
            
            if words == 0 {
                status_label.set_text("Ready");
            } else {
                status_label.set_text("Editing");
            }

            update_preview(&text);
        }
    });
}

#[allow(clippy::too_many_arguments)]
fn setup_button_handlers(
    new_button: &Button,
    open_button: &Button,
    save_button: &Button,
    format_button: &Button,
    view_mode_button: &Button,
    text_buffer: &TextBuffer,
    text_view: &TextView,
    main_title: &WindowTitle,
    status_label: &Label,
    main_paned: &gtk::Paned,
    preview_scroll: &gtk::ScrolledWindow,
    preview_visible: Rc<RefCell<bool>>,
    current_file: Rc<RefCell<Option<std::path::PathBuf>>>,
    file_manager: Rc<RefCell<crate::file_manager::FileManager>>,
) {
    // New document
    new_button.connect_clicked({
        let text_buffer = text_buffer.clone();
        let main_title = main_title.clone();
        let status_label = status_label.clone();
        let current_file = current_file.clone();
        move |_| {
            text_buffer.set_text("");
            main_title.set_title("New Document");
            main_title.set_subtitle("Untitled");
            status_label.set_text("Ready");
            *current_file.borrow_mut() = None;
        }
    });

    // Open document
    open_button.connect_clicked({
        let text_buffer = text_buffer.clone();
        let main_title = main_title.clone();
        let status_label = status_label.clone();
        let current_file = current_file.clone();
        let file_manager = file_manager.clone();
        move |button| {
            if let Some(window) = button.root().and_downcast::<gtk::Window>() {
                let file_manager = file_manager.clone();
                let text_buffer = text_buffer.clone();
                let main_title = main_title.clone();
                let status_label = status_label.clone();
                let current_file = current_file.clone();

                glib::spawn_future_local(async move {
                    if let Ok(Some((path, text))) = file_manager.borrow().open_file_dialog(&window).await {
                        text_buffer.set_text(&text);
                        if let Some(name) = path.file_name() {
                            main_title.set_title(&name.to_string_lossy());
                            main_title.set_subtitle(&path.to_string_lossy());
                        }
                        status_label.set_text("File opened");
                        *current_file.borrow_mut() = Some(path);
                    }
                });
            }
        }
    });

    // Save document
    save_button.connect_clicked({
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        let current_file = current_file.clone();
        let file_manager = file_manager.clone();
        move |button| {
            if let Some(window) = button.root().and_downcast::<gtk::Window>() {
                let text_buffer = text_buffer.clone();
                let status_label = status_label.clone();
                let current_file = current_file.clone();
                let file_manager = file_manager.clone();

                glib::spawn_future_local(async move {
                    let start = text_buffer.start_iter();
                    let end = text_buffer.end_iter();
                    let text = text_buffer.text(&start, &end, false);

                    let result = if let Some(path) = current_file.borrow().as_ref() {
                        file_manager.borrow().save_file(path, &text).await
                    } else {
                        match file_manager.borrow().save_file_dialog(&window, &text).await {
                            Ok(Some(path)) => {
                                *current_file.borrow_mut() = Some(path);
                                Ok(())
                            }
                            Ok(None) => return,
                            Err(e) => Err(e),
                        }
                    };

                    match result {
                        Ok(_) => status_label.set_text("File saved"),
                        Err(_) => status_label.set_text("Save failed"),
                    }
                });
            }
        }
    });

    // Format toggle
    format_button.connect_clicked({
        let text_view = text_view.clone();
        let status_label = status_label.clone();
        move |_| {
            let monospace = text_view.is_monospace();
            text_view.set_monospace(!monospace);
            if monospace {
                status_label.set_text("Switched to proportional font");
            } else {
                status_label.set_text("Switched to monospace font");
            }
        }
    });

    // View mode toggle
    view_mode_button.connect_clicked({
        let main_paned = main_paned.clone();
        let preview_scroll = preview_scroll.clone();
        let status_label = status_label.clone();
        move |button| {
            let visible = *preview_visible.borrow();
            if visible {
                main_paned.set_end_child(gtk::Widget::NONE);
                button.set_icon_name("view-dual-symbolic");
                button.set_tooltip_text(Some("Show Preview"));
                status_label.set_text("Preview hidden");
            } else {
                main_paned.set_end_child(Some(&preview_scroll));
                button.set_icon_name("sidebar-show-right-symbolic");
                button.set_tooltip_text(Some("Hide Preview"));
                status_label.set_text("Preview shown");
            }
            *preview_visible.borrow_mut() = !visible;
        }
    });
}

fn setup_responsive_behavior(split_view: &NavigationSplitView) {
    split_view.connect_collapsed_notify({
        let split_view = split_view.clone();
        move |_| {
            let collapsed = split_view.is_collapsed();
            if collapsed {
                split_view.set_show_content(true);
            }
        }
    });
}