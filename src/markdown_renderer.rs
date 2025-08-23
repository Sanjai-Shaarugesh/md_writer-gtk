use gtk::prelude::*;
use gtk::{TextBuffer, TextTag, TextTagTable};
use std::collections::HashMap;
use pulldown_cmark::{Parser, Event, Tag, Options};

pub struct MarkdownRenderer {
    pub tag_table: TextTagTable,
    tags: HashMap<String, TextTag>,
}

impl MarkdownRenderer {
    pub fn new() -> Self {
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
        
        let heading4_tag = TextTag::new(Some("heading4"));
        heading4_tag.set_scale(1.3);
        heading4_tag.set_weight(600);
        heading4_tag.set_foreground(Some("#6b7280"));
        heading4_tag.set_pixels_below_lines(3);
        tag_table.add(&heading4_tag);
        tags.insert("heading4".to_string(), heading4_tag);
        
        let heading5_tag = TextTag::new(Some("heading5"));
        heading5_tag.set_scale(1.1);
        heading5_tag.set_weight(500);
        heading5_tag.set_foreground(Some("#6b7280"));
        tag_table.add(&heading5_tag);
        tags.insert("heading5".to_string(), heading5_tag);
        
        let heading6_tag = TextTag::new(Some("heading6"));
        heading6_tag.set_scale(1.0);
        heading6_tag.set_weight(500);
        heading6_tag.set_foreground(Some("#9ca3af"));
        tag_table.add(&heading6_tag);
        tags.insert("heading6".to_string(), heading6_tag);
        
        let bold_tag = TextTag::new(Some("bold"));
        bold_tag.set_weight(700);
        tag_table.add(&bold_tag);
        tags.insert("bold".to_string(), bold_tag);
        
        let italic_tag = TextTag::new(Some("italic"));
        italic_tag.set_style(gtk::pango::Style::Italic);
        tag_table.add(&italic_tag);
        tags.insert("italic".to_string(), italic_tag);
        
        let strikethrough_tag = TextTag::new(Some("strikethrough"));
        strikethrough_tag.set_strikethrough(true);
        tag_table.add(&strikethrough_tag);
        tags.insert("strikethrough".to_string(), strikethrough_tag);
        
        let code_tag = TextTag::new(Some("code"));
        code_tag.set_family(Some("monospace"));
        code_tag.set_background(Some("#f3f4f6"));
        code_tag.set_foreground(Some("#dc2626"));
        code_tag.set_size_points(13.0);
        tag_table.add(&code_tag);
        tags.insert("code".to_string(), code_tag);
        
        let code_block_tag = TextTag::new(Some("code_block"));
        code_block_tag.set_family(Some("monospace"));
        code_block_tag.set_background(Some("#f8fafc"));
        code_block_tag.set_foreground(Some("#1e293b"));
        code_block_tag.set_left_margin(20);
        code_block_tag.set_right_margin(20);
        code_block_tag.set_pixels_above_lines(8);
        code_block_tag.set_pixels_below_lines(8);
        code_block_tag.set_size_points(12.0);
        tag_table.add(&code_block_tag);
        tags.insert("code_block".to_string(), code_block_tag);
        
        let quote_tag = TextTag::new(Some("quote"));
        quote_tag.set_left_margin(20);
        quote_tag.set_style(gtk::pango::Style::Italic);
        quote_tag.set_foreground(Some("#6b7280"));
        quote_tag.set_background(Some("#f9fafb"));
        quote_tag.set_pixels_above_lines(4);
        quote_tag.set_pixels_below_lines(4);
        tag_table.add(&quote_tag);
        tags.insert("quote".to_string(), quote_tag);
        
        let list_tag = TextTag::new(Some("list"));
        list_tag.set_left_margin(20);
        tag_table.add(&list_tag);
        tags.insert("list".to_string(), list_tag);
        
        let link_tag = TextTag::new(Some("link"));
        link_tag.set_foreground(Some("#2563eb"));
        link_tag.set_underline(gtk::pango::Underline::Single);
        tag_table.add(&link_tag);
        tags.insert("link".to_string(), link_tag);
        
        // GitHub alert tags
        let alert_note_tag = TextTag::new(Some("alert_note"));
        alert_note_tag.set_background(Some("#dbeafe"));
        alert_note_tag.set_foreground(Some("#1e40af"));
        alert_note_tag.set_left_margin(20);
        alert_note_tag.set_pixels_above_lines(6);
        alert_note_tag.set_pixels_below_lines(6);
        tag_table.add(&alert_note_tag);
        tags.insert("alert_note".to_string(), alert_note_tag);
        
        let alert_tip_tag = TextTag::new(Some("alert_tip"));
        alert_tip_tag.set_background(Some("#dcfce7"));
        alert_tip_tag.set_foreground(Some("#166534"));
        alert_tip_tag.set_left_margin(20);
        alert_tip_tag.set_pixels_above_lines(6);
        alert_tip_tag.set_pixels_below_lines(6);
        tag_table.add(&alert_tip_tag);
        tags.insert("alert_tip".to_string(), alert_tip_tag);
        
        let alert_important_tag = TextTag::new(Some("alert_important"));
        alert_important_tag.set_background(Some("#fef3c7"));
        alert_important_tag.set_foreground(Some("#92400e"));
        alert_important_tag.set_left_margin(20);
        alert_important_tag.set_pixels_above_lines(6);
        alert_important_tag.set_pixels_below_lines(6);
        tag_table.add(&alert_important_tag);
        tags.insert("alert_important".to_string(), alert_important_tag);
        
        let alert_warning_tag = TextTag::new(Some("alert_warning"));
        alert_warning_tag.set_background(Some("#fed7aa"));
        alert_warning_tag.set_foreground(Some("#c2410c"));
        alert_warning_tag.set_left_margin(20);
        alert_warning_tag.set_pixels_above_lines(6);
        alert_warning_tag.set_pixels_below_lines(6);
        tag_table.add(&alert_warning_tag);
        tags.insert("alert_warning".to_string(), alert_warning_tag);
        
        let alert_caution_tag = TextTag::new(Some("alert_caution"));
        alert_caution_tag.set_background(Some("#fecaca"));
        alert_caution_tag.set_foreground(Some("#dc2626"));
        alert_caution_tag.set_left_margin(20);
        alert_caution_tag.set_pixels_above_lines(6);
        alert_caution_tag.set_pixels_below_lines(6);
        tag_table.add(&alert_caution_tag);
        tags.insert("alert_caution".to_string(), alert_caution_tag);
        
        let table_header_tag = TextTag::new(Some("table_header"));
        table_header_tag.set_weight(700);
        table_header_tag.set_background(Some("#f3f4f6"));
        tag_table.add(&table_header_tag);
        tags.insert("table_header".to_string(), table_header_tag);
        
        let table_cell_tag = TextTag::new(Some("table_cell"));
        table_cell_tag.set_left_margin(10);
        table_cell_tag.set_right_margin(10);
        tag_table.add(&table_cell_tag);
        tags.insert("table_cell".to_string(), table_cell_tag);
        
        Self {
            tag_table,
            tags,
        }
    }
    
    pub fn render_markdown(&self, buffer: &TextBuffer, markdown_text: &str) {
        buffer.set_text("");
        let mut iter = buffer.start_iter();
        
        // Set up pulldown-cmark options with GitHub features
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);
        
        // Parse GitHub alerts manually first
        let processed_text = self.process_github_alerts(markdown_text);
        
        let parser = Parser::new_ext(&processed_text, options);
        let events: Vec<Event> = parser.collect();
        
        self.render_events(buffer, &mut iter, &events);
    }
    
    fn process_github_alerts(&self, text: &str) -> String {
        let lines: Vec<&str> = text.lines().collect();
        let mut result = Vec::new();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();

            // Check for GitHub alert syntax
            if line.starts_with("> [!") {
                if let Some(end_pos) = line.find(']') {
                    let alert_type = line[4..end_pos].to_lowercase();
                    let alert_content = if line.len() > end_pos + 1 {
                        line[end_pos + 1..].trim()
                    } else {
                        ""
                    };

                    // Mark the beginning of alert
                    result.push(format!("{{{{ALERT_START_{}}}}}", alert_type.to_uppercase()));

                    // Add the alert title
                    let title = match alert_type.as_str() {
                        "note" => "📝 Note",
                        "tip" => "💡 Tip",
                        "important" => "⚠️ Important",
                        "warning" => "⚠️ Warning",
                        "caution" => "🚨 Caution",
                        _ => "ℹ️ Info",
                    };

                    result.push(format!("{}: {}", title, alert_content));

                    // Process continuation lines
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

                    // Mark the end of alert
                    result.push(format!("{{{{ALERT_END_{}}}}}", alert_type.to_uppercase()));
                    continue;
                }
            }

            result.push(line.to_string());
            i += 1;
        }

        result.join("\n")
    }
    
    fn render_events(&self, buffer: &TextBuffer, iter: &mut gtk::TextIter, events: &[Event]) {
            let mut tag_stack: Vec<String> = Vec::new();
            let mut list_level = 0;
            let mut in_code_block = false;
            let mut table_in_header = false;

            for event in events {
                match event {
                    Event::Start(tag) => {
                        match tag {
                            Tag::Heading { level, .. } => {
                                let tag_name = format!("heading{}", level);
                                tag_stack.push(tag_name);
                            },
                            Tag::Strong => tag_stack.push("bold".to_string()),
                            Tag::Emphasis => tag_stack.push("italic".to_string()),
                            Tag::Strikethrough => tag_stack.push("strikethrough".to_string()),
                            Tag::CodeBlock(_) => {
                                in_code_block = true;
                                tag_stack.push("code_block".to_string());
                            },
                            Tag::BlockQuote(_) => tag_stack.push("quote".to_string()),
                            Tag::List(_) => {
                                list_level += 1;
                            },
                            Tag::Item => {
                                buffer.insert(iter, &format!("{} ", "•".repeat(list_level)));
                            },
                            Tag::Link { .. } => {
                                tag_stack.push("link".to_string());
                            },
                            Tag::Table(_) => {},
                            Tag::TableHead => {
                                table_in_header = true;
                            },
                            Tag::TableRow => {},
                            Tag::TableCell => {
                                if table_in_header {
                                    tag_stack.push("table_header".to_string());
                                } else {
                                    tag_stack.push("table_cell".to_string());
                                }
                            },
                            _ => {}
                        }
                    },
                    Event::End(tag_end) => {
                        match tag_end {
                            pulldown_cmark::TagEnd::Heading(_) => {
                                tag_stack.pop();
                                buffer.insert(iter, "\n\n");
                            },
                            pulldown_cmark::TagEnd::Strong => {
                                tag_stack.pop();
                            },
                            pulldown_cmark::TagEnd::Emphasis => {
                                tag_stack.pop();
                            },
                            pulldown_cmark::TagEnd::Strikethrough => {
                                tag_stack.pop();
                            },
                            pulldown_cmark::TagEnd::CodeBlock => {
                                in_code_block = false;
                                tag_stack.pop();
                                buffer.insert(iter, "\n\n");
                            },
                            pulldown_cmark::TagEnd::BlockQuote(_) => {
                                tag_stack.pop();
                                buffer.insert(iter, "\n\n");
                            },
                            pulldown_cmark::TagEnd::List(_) => {
                                list_level = list_level.saturating_sub(1);
                            },
                            pulldown_cmark::TagEnd::Item => {
                                buffer.insert(iter, "\n");
                            },
                            pulldown_cmark::TagEnd::Link => {
                                tag_stack.pop();
                            },
                            pulldown_cmark::TagEnd::TableHead => {
                                table_in_header = false;
                                buffer.insert(iter, "\n");
                            },
                            pulldown_cmark::TagEnd::TableRow => {
                                buffer.insert(iter, "\n");
                            },
                            pulldown_cmark::TagEnd::TableCell => {
                                tag_stack.pop();
                                buffer.insert(iter, " | ");
                            },
                            pulldown_cmark::TagEnd::Paragraph => {
                                buffer.insert(iter, "\n\n");
                            },
                            _ => {}
                        }
                    },
                    Event::Text(text) => {
                        // Check for GitHub alert markers
                        if text.starts_with("{{ALERT_START_") {
                            let alert_type = text[14..text.len()-2].to_lowercase();
                            tag_stack.push(format!("alert_{}", alert_type));
                            continue;
                        } else if text.starts_with("{{ALERT_END_") {
                            tag_stack.pop();
                            buffer.insert(iter, "\n\n");
                            continue;
                        }

                        if !text.trim().is_empty() || in_code_block {
                            self.insert_formatted_text(buffer, iter, text, &tag_stack);
                        }
                    },
                    Event::Code(text) => {
                        tag_stack.push("code".to_string());
                        self.insert_formatted_text(buffer, iter, text, &tag_stack);
                        tag_stack.pop();
                    },
                    Event::Html(html) => {
                        // Handle HTML content if needed
                        buffer.insert(iter, html);
                    },
                    Event::SoftBreak => {
                        if in_code_block {
                            buffer.insert(iter, "\n");
                        } else {
                            buffer.insert(iter, " ");
                        }
                    },
                    Event::HardBreak => {
                        buffer.insert(iter, "\n");
                    },
                    Event::Rule => {
                        buffer.insert(iter, "\n────────────────────────────────────────────────────\n\n");
                    },
                    _ => {}
                }
            }
        }
    
    fn insert_formatted_text(&self, buffer: &TextBuffer, iter: &mut gtk::TextIter, text: &str, tag_stack: &[String]) {
        let start_mark = buffer.create_mark(None, iter, false);
        buffer.insert(iter, text);
        
        let start_iter = buffer.iter_at_mark(&start_mark);
        
        // Apply all active tags
        for tag_name in tag_stack {
            if let Some(tag) = self.tags.get(tag_name) {
                buffer.apply_tag(tag, &start_iter, iter);
            }
        }
        
        buffer.delete_mark(&start_mark);
    }
}