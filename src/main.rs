use gtk::prelude::*;
use gtk::{Builder, TextView, ApplicationWindow, Button, ToggleButton, SearchEntry, Label, TextBuffer};
use adw::{Application, NavigationSplitView, ActionRow, WindowTitle};
use std::cell::RefCell;
use std::rc::Rc;

fn build_ui(app: &Application) {
    let builder = Builder::from_file("src/ui/main-window.ui");
    let window: ApplicationWindow = builder.object("main_window")
        .expect("Couldn't get main_window");
    window.set_application(Some(app));
    
    // Get main components
    let text_view: TextView = builder.object("text_view")
        .expect("Couldn't get text_view");
    let text_buffer: TextBuffer = builder.object("text_buffer")
        .expect("Couldn't get text_buffer");
    let split_view: NavigationSplitView = builder.object("split_view")
        .expect("Couldn't get split_view");
    
    // Get UI elements
    let sidebar_toggle_button: ToggleButton = builder.object("sidebar_toggle_button")
        .expect("Couldn't get sidebar_toggle_button");
    let show_sidebar_button: Button = builder.object("show_sidebar_button")
        .expect("Couldn't get show_sidebar_button");
    let search_entry: SearchEntry = builder.object("search_entry")
        .expect("Couldn't get search_entry");
    let word_count_label: Label = builder.object("word_count_label")
        .expect("Couldn't get word_count_label");
    let status_label: Label = builder.object("status_label")
        .expect("Couldn't get status_label");
    let main_title: WindowTitle = builder.object("main_title")
        .expect("Couldn't get main_title");
    
    // Get category rows
    let category_documents: ActionRow = builder.object("category_documents")
        .expect("Couldn't get category_documents");
    let category_projects: ActionRow = builder.object("category_projects")
        .expect("Couldn't get category_projects");
    let category_favorites: ActionRow = builder.object("category_favorites")
        .expect("Couldn't get category_favorites");
    
    // Get recent file rows
    let recent_file_1: ActionRow = builder.object("recent_file_1")
        .expect("Couldn't get recent_file_1");
    let recent_file_2: ActionRow = builder.object("recent_file_2")
        .expect("Couldn't get recent_file_2");
    let recent_file_3: ActionRow = builder.object("recent_file_3")
        .expect("Couldn't get recent_file_3");
    
    // Get action buttons
    let new_document_btn: Button = builder.object("new_document_btn")
        .expect("Couldn't get new_document_btn");
    let open_document_btn: Button = builder.object("open_document_btn")
        .expect("Couldn't get open_document_btn");
    let save_button: Button = builder.object("save_button")
        .expect("Couldn't get save_button");
    let format_button: Button = builder.object("format_button")
        .expect("Couldn't get format_button");
    let view_mode_button: Button = builder.object("view_mode_button")
        .expect("Couldn't get view_mode_button");
    
    // Set up word count tracking
    let word_count = Rc::new(RefCell::new(0));
    let word_count_clone = word_count.clone();
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
            
            *word_count_clone.borrow_mut() = words;
            word_count_label.set_text(&format!("{} words", words));
            
            if words == 0 {
                status_label.set_text("Ready");
            } else {
                status_label.set_text("Editing");
            }
        }
    });
    
    // Initialize sidebar state and responsiveness
    setup_sidebar_controls(&split_view, &sidebar_toggle_button, &show_sidebar_button);
    
    // Set up category navigation
    setup_category_navigation(
        &category_documents,
        &category_projects, 
        &category_favorites,
        &main_title,
        &text_buffer,
        &status_label
    );
    
    // Set up recent files
    setup_recent_files(
        &recent_file_1,
        &recent_file_2,
        &recent_file_3,
        &main_title,
        &text_buffer,
        &status_label
    );
    
    // Set up search functionality
    search_entry.connect_search_changed({
        let status_label = status_label.clone();
        move |entry| {
            let query = entry.text();
            if query.is_empty() {
                status_label.set_text("Ready");
            } else {
                status_label.set_text(&format!("Searching: {}", query));
            }
        }
    });
    
    // Set up action buttons
    new_document_btn.connect_clicked({
        let text_buffer = text_buffer.clone();
        let main_title = main_title.clone();
        let status_label = status_label.clone();
        move |_| {
            text_buffer.set_text("");
            main_title.set_title("New Document");
            main_title.set_subtitle("Untitled");
            status_label.set_text("Ready");
        }
    });
    
    open_document_btn.connect_clicked({
        let status_label = status_label.clone();
        move |_| {
            status_label.set_text("Open dialog would appear here");
        }
    });
    
    save_button.connect_clicked({
        let status_label = status_label.clone();
        let word_count = word_count.clone();
        move |_| {
            let words = *word_count.borrow();
            status_label.set_text(&format!("Saved {} words", words));
        }
    });
    
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
    
    view_mode_button.connect_clicked({
        let text_view = text_view.clone();
        let status_label = status_label.clone();
        move |_| {
            let wrap_mode = text_view.wrap_mode();
            match wrap_mode {
                gtk::WrapMode::Word => {
                    text_view.set_wrap_mode(gtk::WrapMode::None);
                    status_label.set_text("No text wrapping");
                },
                _ => {
                    text_view.set_wrap_mode(gtk::WrapMode::Word);
                    status_label.set_text("Word wrapping enabled");
                }
            }
        }
    });
    
    window.present();
}

fn setup_sidebar_controls(
    split_view: &NavigationSplitView,
    sidebar_toggle_button: &ToggleButton,
    show_sidebar_button: &Button,
) {
    // Set initial state
    let initial_collapsed = split_view.is_collapsed();
    show_sidebar_button.set_visible(initial_collapsed);
    sidebar_toggle_button.set_active(!initial_collapsed);
    update_sidebar_button_icon(sidebar_toggle_button, initial_collapsed);
    
    // Connect the main sidebar toggle button
    sidebar_toggle_button.connect_toggled({
        let split_view = split_view.clone();
        let show_sidebar_button = show_sidebar_button.clone();
        move |button| {
            let is_active = button.is_active();
            split_view.set_collapsed(!is_active);
            show_sidebar_button.set_visible(!is_active);
            update_sidebar_button_icon(button, !is_active);
        }
    });
    
    // Connect show sidebar button (appears when sidebar is hidden)
    show_sidebar_button.connect_clicked({
        let split_view = split_view.clone();
        let show_sidebar_button = show_sidebar_button.clone();
        let sidebar_toggle_button = sidebar_toggle_button.clone();
        move |_| {
            split_view.set_collapsed(false);
            show_sidebar_button.set_visible(false);
            sidebar_toggle_button.set_active(true);
        }
    });
    
    // Listen to split view property changes for responsive behavior
    split_view.connect_collapsed_notify({
        let show_sidebar_button = show_sidebar_button.clone();
        let sidebar_toggle_button = sidebar_toggle_button.clone();
        move |split_view| {
            let is_collapsed = split_view.is_collapsed();
            show_sidebar_button.set_visible(is_collapsed);
            sidebar_toggle_button.set_active(!is_collapsed);
            update_sidebar_button_icon(&sidebar_toggle_button, is_collapsed);
        }
    });
}

fn update_sidebar_button_icon(button: &ToggleButton, is_collapsed: bool) {
    if is_collapsed {
        button.set_icon_name("sidebar-show-symbolic-rtl");
        button.set_tooltip_text(Some("Show Sidebar"));
    } else {
        button.set_icon_name("sidebar-show-right-symbolic-rtl");
        button.set_tooltip_text(Some("Hide Sidebar"));
    }
}

fn setup_category_navigation(
    documents: &ActionRow,
    projects: &ActionRow,
    favorites: &ActionRow,
    main_title: &WindowTitle,
    text_buffer: &TextBuffer,
    status_label: &Label,
) {
    documents.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("Documents");
            main_title.set_subtitle("Text files and notes");
            text_buffer.set_text("# Documents Category\n\nShowing all documents in your collection.\n\n- README.md\n- project-notes.txt\n- documentation.md\n- ideas.txt");
            status_label.set_text("Browsing documents");
        }
    });
    
    projects.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("Projects");
            main_title.set_subtitle("Active work items");
            text_buffer.set_text("# Projects Category\n\nYour active projects:\n\n## Current Projects\n- Website Redesign\n- Mobile App Development\n- Documentation Update\n- Code Refactoring\n- User Research");
            status_label.set_text("Browsing projects");
        }
    });
    
    favorites.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("Favorites");
            main_title.set_subtitle("Bookmarked items");
            text_buffer.set_text("# Favorites Category\n\nYour bookmarked and starred items:\n\n⭐ Important Meeting Notes\n⭐ Code Snippets Collection\n⭐ Research References\n⭐ Quick Ideas\n⭐ Templates\n⭐ Useful Links\n⭐ Draft Articles\n⭐ Configuration Files");
            status_label.set_text("Browsing favorites");
        }
    });
}

fn setup_recent_files(
    recent_1: &ActionRow,
    recent_2: &ActionRow,
    recent_3: &ActionRow,
    main_title: &WindowTitle,
    text_buffer: &TextBuffer,
    status_label: &Label,
) {
    recent_1.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("README.md");
            main_title.set_subtitle("Last modified 2 hours ago");
            text_buffer.set_text("# Project README\n\nWelcome to our awesome project!\n\n## Getting Started\n\nThis project is built with modern technologies and follows best practices.\n\n### Prerequisites\n\n- Rust 1.70+\n- GTK4 development libraries\n- Git\n\n### Installation\n\n```bash\ngit clone https://github.com/user/project.git\ncd project\ncargo build --release\n```\n\n### Usage\n\nRun the application with:\n\n```bash\ncargo run\n```\n\n## Features\n\n- Responsive design\n- Cross-platform compatibility\n- Modern UI with Libadwaita\n- Efficient performance\n\n## Contributing\n\nWe welcome contributions! Please read our contributing guidelines.");
            status_label.set_text("Opened README.md");
        }
    });
    
    recent_2.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("project-notes.txt");
            main_title.set_subtitle("Last modified yesterday");
            text_buffer.set_text("# Project Development Notes\n\n## Daily Standup - Week 1\n\n### Monday\n- Set up development environment\n- Created initial project structure\n- Implemented basic UI layout\n\n### Tuesday\n- Added responsive design features\n- Implemented sidebar navigation\n- Fixed layout issues on mobile devices\n\n### Wednesday\n- Enhanced text editor functionality\n- Added word count feature\n- Improved user experience\n\n### Thursday\n- Code refactoring and optimization\n- Added search functionality\n- Updated documentation\n\n### Friday\n- Testing and bug fixes\n- Performance improvements\n- Prepared for deployment\n\n## Next Week Goals\n\n- Add more editor features\n- Implement file operations\n- Add syntax highlighting\n- Create user preferences");
            status_label.set_text("Opened project-notes.txt");
        }
    });
    
    recent_3.connect_activate({
        let main_title = main_title.clone();
        let text_buffer = text_buffer.clone();
        let status_label = status_label.clone();
        move |_| {
            main_title.set_title("meeting-agenda.md");
            main_title.set_subtitle("Last modified 3 days ago");
            text_buffer.set_text("# Team Meeting Agenda\n\n**Date:** March 20, 2024\n**Time:** 10:00 AM - 11:30 AM\n**Location:** Conference Room A / Zoom\n\n## Attendees\n\n- Alice Johnson (Product Manager)\n- Bob Smith (Lead Developer)\n- Carol White (UI/UX Designer)\n- David Brown (QA Engineer)\n- Emma Davis (DevOps)\n\n## Agenda Items\n\n### 1. Project Status Update (20 min)\n- Development progress review\n- Current milestone achievements\n- Upcoming deliverables\n\n### 2. Technical Discussions (30 min)\n- Architecture decisions\n- Performance optimization\n- Code review process\n\n### 3. Design Review (20 min)\n- UI mockups presentation\n- User feedback integration\n- Accessibility improvements\n\n### 4. Quality Assurance (15 min)\n- Testing strategy\n- Bug reports and fixes\n- Release criteria\n\n### 5. Next Steps (5 min)\n- Action items assignment\n- Next meeting schedule\n- Blockers and dependencies\n\n## Action Items\n\n- [ ] Finalize API documentation (Bob)\n- [ ] Update design system (Carol)\n- [ ] Set up CI/CD pipeline (Emma)\n- [ ] Create test cases (David)");
            status_label.set_text("Opened meeting-agenda.md");
        }
    });
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.adaptiveeditor")
        .build();
    
    app.connect_activate(build_ui);
    app.run();
}