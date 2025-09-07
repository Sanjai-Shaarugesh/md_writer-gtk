use gtk::{gdk, gio, glib, prelude::*};
use gtk::{TextBuffer, TextTag, TextTagTable, gdk_pixbuf::Pixbuf, TextChildAnchor};
use std::collections::HashMap;
use pulldown_cmark::{Parser, Event, Tag, Options};
use std::path::{Path, PathBuf};
use reqwest::get;
use std::fs;
use std::rc::Rc;
use std::cell::RefCell;

pub struct MarkdownRenderer {
    pub tag_table: TextTagTable,
    tags: HashMap<String, TextTag>,
    base_path: Option<PathBuf>,
    runtime: Option<tokio::runtime::Runtime>, // For async operations
}

impl MarkdownRenderer {
    pub fn new(base_path: Option<&str>) -> Self {
        let tag_table = TextTagTable::new();
        let mut tags = HashMap::new();

        // Create comprehensive text formatting tags
        let heading1_tag = TextTag::new(Some("heading1"));
        heading1_tag.set_scale(2.2);
        heading1_tag.set_weight(800);
        heading1_tag.set_foreground(Some("#1f2937"));
        heading1_tag.set_pixels_below_lines(8);
        tag_table.add(&heading1_tag);
        tags.insert("heading1".to_string(), heading1_tag);

        let heading2_tag = TextTag::new(Some("heading2"));
        heading2_tag.set_scale(1.8);
        heading2_tag.set_weight(700);
        heading2_tag.set_foreground(Some("#374151"));
        heading2_tag.set_pixels_below_lines(6);
        tag_table.add(&heading2_tag);
        tags.insert("heading2".to_string(), heading2_tag);

        let heading3_tag = TextTag::new(Some("heading3"));
        heading3_tag.set_scale(1.5);
        heading3_tag.set_weight(600);
        heading3_tag.set_foreground(Some("#4b5563"));
        heading3_tag.set_pixels_below_lines(4);
        tag_table.add(&heading3_tag);
        tags.insert("heading3".to_string(), heading3_tag);

        let bold_tag = TextTag::new(Some("bold"));
        bold_tag.set_weight(700);
        tag_table.add(&bold_tag);
        tags.insert("bold".to_string(), bold_tag);

        let italic_tag = TextTag::new(Some("italic"));
        italic_tag.set_style(gtk::pango::Style::Italic);
        tag_table.add(&italic_tag);
        tags.insert("italic".to_string(), italic_tag);

        let code_tag = TextTag::new(Some("code"));
        code_tag.set_family(Some("monospace"));
        code_tag.set_background(Some("#f3f4f6"));
        code_tag.set_foreground(Some("#dc2626"));
        tag_table.add(&code_tag);
        tags.insert("code".to_string(), code_tag);

        let code_block_tag = TextTag::new(Some("code_block"));
        code_block_tag.set_family(Some("monospace"));
        code_block_tag.set_background(Some("#1f2937"));
        code_block_tag.set_foreground(Some("#f9fafb"));
        code_block_tag.set_pixels_above_lines(8);
        code_block_tag.set_pixels_below_lines(8);
        code_block_tag.set_left_margin(16);
        code_block_tag.set_right_margin(16);
        tag_table.add(&code_block_tag);
        tags.insert("code_block".to_string(), code_block_tag);

        let link_tag = TextTag::new(Some("link"));
        link_tag.set_foreground(Some("#2563eb"));
        link_tag.set_underline(gtk::pango::Underline::Single);
        tag_table.add(&link_tag);
        tags.insert("link".to_string(), link_tag);

        // Create a runtime for async operations
        let runtime = tokio::runtime::Runtime::new().ok();

        Self {
            tag_table,
            tags,
            base_path: base_path.map(PathBuf::from),
            runtime,
        }
    }

    pub fn render_markdown(&self, buffer: &TextBuffer, markdown_text: &str, text_view: &gtk::TextView) {
        buffer.set_text("");
        let mut iter = buffer.start_iter();

        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        let processed_text = self.process_github_alerts(markdown_text);
        let parser = Parser::new_ext(&processed_text, options);
        let events: Vec<Event> = parser.collect();

        self.render_events(buffer, &mut iter, &events, text_view);
    }

    fn process_github_alerts(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("> [!") {
                if let Some(end_pos) = line.find(']') {
                    let alert_type = line[4..end_pos].to_lowercase();
                    let alert_content = if line.len() > end_pos + 1 {
                        line[end_pos + 1..].trim()
                    } else {
                        ""
                    };
                    result.push(format!("{{{{ALERT_START_{}}}}}", alert_type.to_uppercase()));
                    let title = match alert_type.as_str() {
                        "note" => "📝 Note",
                        "tip" => "💡 Tip",
                        "important" => "⚠️ Important",
                        "warning" => "⚠️ Warning",
                        "caution" => "🚨 Caution",
                        _ => "ℹ️ Info",
                    };
                    result.push(format!("{}: {}", title, alert_content));
                    i += 1;
                    while i < lines.len() && lines[i].trim_start().starts_with("> ") {
                        let content = lines[i].trim_start();
                        if content.len() > 2 {
                            result.push(content[2..].to_string());
                        } else {
                            result.push(String::new());
                        }
                        i += 1;
                    }
                    result.push(format!("{{{{ALERT_END_{}}}}}", alert_type.to_uppercase()));
                    continue;
                }
            }
            result.push(line.to_string());
            i += 1;
        }
        result.join("\n")
    }

    async fn load_image(&self, path_or_url: &str, max_width: i32, max_height: i32) -> Option<Pixbuf> {
        if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
            // Handle URL images
            match get(path_or_url).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let bytes = match response.bytes().await {
                            Ok(bytes) => bytes,
                            Err(e) => {
                                eprintln!("Failed to read bytes from URL {}: {}", path_or_url, e);
                                return None;
                            }
                        };
                        let stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(&bytes));
                        match Pixbuf::from_stream(&stream, None::<&gio::Cancellable>) {
                            Ok(pixbuf) => {
                                // Scale image if needed
                                let (width, height) = (pixbuf.width(), pixbuf.height());
                                if width > max_width || height > max_height {
                                    let scale_x = max_width as f64 / width as f64;
                                    let scale_y = max_height as f64 / height as f64;
                                    let scale = scale_x.min(scale_y);
                                    let new_width = (width as f64 * scale) as i32;
                                    let new_height = (height as f64 * scale) as i32;
                                    
                                    match pixbuf.scale_simple(new_width, new_height, gdk_pixbuf::InterpType::Bilinear) {
                                        Some(scaled_pixbuf) => Some(scaled_pixbuf),
                                        None => Some(pixbuf),
                                    }
                                } else {
                                    Some(pixbuf)
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to create pixbuf from URL {}: {}", path_or_url, e);
                                None
                            }
                        }
                    } else {
                        eprintln!("Failed to download image from URL {}: HTTP {}", path_or_url, response.status());
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Failed to download image from URL {}: {}", path_or_url, e);
                    None
                }
            }
        } else {
            // Handle local images
            let path = if let Some(base) = &self.base_path {
                base.join(path_or_url)
            } else {
                PathBuf::from(path_or_url)
            };

            if !path.exists() {
                eprintln!("Local image file does not exist: {}", path.display());
                return None;
            }

            match Pixbuf::from_file(&path) {
                Ok(pixbuf) => {
                    // Scale image if needed
                    let (width, height) = (pixbuf.width(), pixbuf.height());
                    if width > max_width || height > max_height {
                        let scale_x = max_width as f64 / width as f64;
                        let scale_y = max_height as f64 / height as f64;
                        let scale = scale_x.min(scale_y);
                        let new_width = (width as f64 * scale) as i32;
                        let new_height = (height as f64 * scale) as i32;
                        
                        match pixbuf.scale_simple(new_width, new_height, gdk_pixbuf::InterpType::Bilinear) {
                            Some(scaled_pixbuf) => Some(scaled_pixbuf),
                            None => Some(pixbuf),
                        }
                    } else {
                        Some(pixbuf)
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load local image {}: {}", path.display(), e);
                    None
                }
            }
        }
    }

    fn load_image_sync(&self, path_or_url: &str, max_width: i32, max_height: i32) -> Option<Pixbuf> {
        if let Some(ref runtime) = self.runtime {
            runtime.block_on(self.load_image(path_or_url, max_width, max_height))
        } else {
            eprintln!("No runtime available for async image loading");
            None
        }
    }

    fn render_events(&self, buffer: &TextBuffer, iter: &mut gtk::TextIter, events: &[Event], text_view: &gtk::TextView) {
            let mut tag_stack: Vec<String> = Vec::new();
            let mut list_level: usize = 0;
            let mut in_code_block = false;
            let mut table_in_header = false;

            for event in events {
                match event {
                    Event::Start(tag) => {
                        match tag {
                            Tag::Heading { level, .. } => {
                                let tag_name = format!("heading{}", level);
                                tag_stack.push(tag_name);
                            }
                            Tag::Strong => tag_stack.push("bold".to_string()),
                            Tag::Emphasis => tag_stack.push("italic".to_string()),
                            Tag::Strikethrough => tag_stack.push("strikethrough".to_string()),
                            Tag::CodeBlock(_) => {
                                in_code_block = true;
                                tag_stack.push("code_block".to_string());
                            }
                            Tag::BlockQuote(_) => tag_stack.push("quote".to_string()),
                            Tag::List(_) => {
                                list_level += 1;
                            }
                            Tag::Item => {
                                // Add proper indentation based on list level
                                let indent = "  ".repeat(list_level.saturating_sub(1));
                                buffer.insert(iter, &format!("{}• ", indent));
                            }
                            Tag::Link { .. } => {
                                tag_stack.push("link".to_string());
                            }
                            Tag::Image { dest_url, title, .. } => {
                                // Create a placeholder first
                                let _fallback_text: &str = if !title.is_empty() { 
                                    title.as_ref() 
                                } else { 
                                    "Loading image..." 
                                };

                                let anchor = buffer.create_child_anchor(iter);

                                // Create a placeholder widget
                                let image_widget = gtk::Image::new();
                                image_widget.set_icon_name(Some("image-loading"));
                                image_widget.set_icon_size(gtk::IconSize::Large);
                                text_view.add_child_at_anchor(&image_widget, &anchor);

                                // Load the actual image synchronously
                                let dest_url_clone = dest_url.to_string();
                                if let Some(pixbuf) = self.load_image_sync(&dest_url_clone, 800, 600) {
                                    image_widget.set_from_pixbuf(Some(&pixbuf));
                                }

                                buffer.insert(iter, "\n");
                            }
                            Tag::Table(_) => {
                                // Add table start formatting
                                buffer.insert(iter, "\n");
                            }
                            Tag::TableHead => {
                                table_in_header = true;
                            }
                            Tag::TableRow => {
                                buffer.insert(iter, "| ");
                            }
                            Tag::TableCell => {
                                if table_in_header {
                                    tag_stack.push("table_header".to_string());
                                } else {
                                    tag_stack.push("table_cell".to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                    Event::End(tag_end) => {
                        match tag_end {
                            pulldown_cmark::TagEnd::Heading(_) => {
                                tag_stack.pop();
                                buffer.insert(iter, "\n\n");
                            }
                            pulldown_cmark::TagEnd::Strong => {
                                tag_stack.pop();
                            }
                            pulldown_cmark::TagEnd::Emphasis => {
                                tag_stack.pop();
                            }
                            pulldown_cmark::TagEnd::Strikethrough => {
                                tag_stack.pop();
                            }
                            pulldown_cmark::TagEnd::CodeBlock => {
                                in_code_block = false;
                                tag_stack.pop();
                                buffer.insert(iter, "\n\n");
                            }
                            pulldown_cmark::TagEnd::BlockQuote(_) => {
                                tag_stack.pop();
                                buffer.insert(iter, "\n\n");
                            }
                            pulldown_cmark::TagEnd::List(_) => {
                                list_level = list_level.saturating_sub(1);
                                if list_level == 0 {
                                    buffer.insert(iter, "\n");
                                }
                            }
                            pulldown_cmark::TagEnd::Item => {
                                buffer.insert(iter, "\n");
                            }
                            pulldown_cmark::TagEnd::Link => {
                                tag_stack.pop();
                            }
                            pulldown_cmark::TagEnd::Image => {
                                // Image handling is done in Start event
                            }
                            pulldown_cmark::TagEnd::Table => {
                                buffer.insert(iter, "\n\n");
                            }
                            pulldown_cmark::TagEnd::TableHead => {
                                table_in_header = false;
                                buffer.insert(iter, " |\n");
                                // Add separator line for table headers
                                buffer.insert(iter, "|");
                                // This would ideally calculate proper column widths
                                buffer.insert(iter, "---|");
                                buffer.insert(iter, "\n");
                            }
                            pulldown_cmark::TagEnd::TableRow => {
                                buffer.insert(iter, " |\n");
                            }
                            pulldown_cmark::TagEnd::TableCell => {
                                if tag_stack.pop().is_some() {
                                    buffer.insert(iter, " | ");
                                }
                            }
                            pulldown_cmark::TagEnd::Paragraph => {
                                buffer.insert(iter, "\n\n");
                            }
                            _ => {}
                        }
                    }
                    Event::Text(text) => {
                        // Handle custom alert syntax
                        if text.starts_with("{{ALERT_START_") && text.ends_with("}}") {
                            let alert_type = text[14..text.len() - 2].to_lowercase();
                            tag_stack.push(format!("alert_{}", alert_type));
                            continue;
                        } else if text.starts_with("{{ALERT_END_") && text.ends_with("}}") {
                            if tag_stack.pop().is_some() {
                                buffer.insert(iter, "\n\n");
                            }
                            continue;
                        }

                        // Always insert text in code blocks, otherwise check if it's not just whitespace
                        if in_code_block || !text.trim().is_empty() {
                            self.insert_formatted_text(buffer, iter, text, &tag_stack);
                        }
                    }
                    Event::Code(text) => {
                        tag_stack.push("code".to_string());
                        self.insert_formatted_text(buffer, iter, text, &tag_stack);
                        tag_stack.pop();
                    }
                    Event::Html(html) => {
                        // Be cautious with raw HTML insertion
                        if html.trim().is_empty() {
                            return;
                        }
                        buffer.insert(iter, html);
                    }
                    Event::SoftBreak => {
                        if in_code_block {
                            buffer.insert(iter, "\n");
                        } else {
                            buffer.insert(iter, " ");
                        }
                    }
                    Event::HardBreak => {
                        buffer.insert(iter, "\n");
                    }
                    Event::Rule => {
                        buffer.insert(iter, "\n────────────────────────────────────────────────────\n\n");
                    }
                    Event::TaskListMarker(checked) => {
                        let checkbox = if *checked { "☑ " } else { "☐ " };
                        buffer.insert(iter, checkbox);
                    }
                    _ => {
                        // Handle any other events that might be added in the future
                    }
                }
            }
        }

    fn insert_formatted_text(&self, buffer: &TextBuffer, iter: &mut gtk::TextIter, text: &str, tag_stack: &[String]) {
        let start_mark = buffer.create_mark(None, iter, false);
        buffer.insert(iter, text);

        let start_iter = buffer.iter_at_mark(&start_mark);

        for tag_name in tag_stack {
            if let Some(tag) = self.tags.get(tag_name) {
                buffer.apply_tag(tag, &start_iter, iter);
            }
        }

        buffer.delete_mark(&start_mark);
    }

    // Helper method to create a minimal clone for async operations
    fn clone_for_async(&self) -> AsyncImageLoader {
        AsyncImageLoader {
            base_path: self.base_path.clone(),
        }
    }
}

// Simplified struct for async image loading
#[derive(Clone)]
struct AsyncImageLoader {
    base_path: Option<PathBuf>,
}

impl AsyncImageLoader {
    async fn load_image(&self, path_or_url: &str, max_width: i32, max_height: i32) -> Option<Pixbuf> {
        if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
            // Handle URL images
            match get(path_or_url).await {
                Ok(response) => {
                    if response.status().is_success() {
                        let bytes = match response.bytes().await {
                            Ok(bytes) => bytes,
                            Err(e) => {
                                eprintln!("Failed to read bytes from URL {}: {}", path_or_url, e);
                                return None;
                            }
                        };
                        let stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(&bytes));
                        match Pixbuf::from_stream(&stream, None::<&gio::Cancellable>) {
                            Ok(pixbuf) => {
                                // Scale image if needed
                                let (width, height) = (pixbuf.width(), pixbuf.height());
                                if width > max_width || height > max_height {
                                    let scale_x = max_width as f64 / width as f64;
                                    let scale_y = max_height as f64 / height as f64;
                                    let scale = scale_x.min(scale_y);
                                    let new_width = (width as f64 * scale) as i32;
                                    let new_height = (height as f64 * scale) as i32;
                                    
                                    match pixbuf.scale_simple(new_width, new_height, gdk_pixbuf::InterpType::Bilinear) {
                                        Some(scaled_pixbuf) => Some(scaled_pixbuf),
                                        None => Some(pixbuf),
                                    }
                                } else {
                                    Some(pixbuf)
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to create pixbuf from URL {}: {}", path_or_url, e);
                                None
                            }
                        }
                    } else {
                        eprintln!("Failed to download image from URL {}: HTTP {}", path_or_url, response.status());
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Failed to download image from URL {}: {}", path_or_url, e);
                    None
                }
            }
        } else {
            // Handle local images (similar to URL handling but with file operations)
            let path = if let Some(base) = &self.base_path {
                base.join(path_or_url)
            } else {
                PathBuf::from(path_or_url)
            };

            if !path.exists() {
                eprintln!("Local image file does not exist: {}", path.display());
                return None;
            }

            match Pixbuf::from_file(&path) {
                Ok(pixbuf) => {
                    let (width, height) = (pixbuf.width(), pixbuf.height());
                    if width > max_width || height > max_height {
                        let scale_x = max_width as f64 / width as f64;
                        let scale_y = max_height as f64 / height as f64;
                        let scale = scale_x.min(scale_y);
                        let new_width = (width as f64 * scale) as i32;
                        let new_height = (height as f64 * scale) as i32;
                        
                        match pixbuf.scale_simple(new_width, new_height, gdk_pixbuf::InterpType::Bilinear) {
                            Some(scaled_pixbuf) => Some(scaled_pixbuf),
                            None => Some(pixbuf),
                        }
                    } else {
                        Some(pixbuf)
                    }
                }
                Err(e) => {
                    eprintln!("Failed to load local image {}: {}", path.display(), e);
                    None
                }
            }
        }
    }
}