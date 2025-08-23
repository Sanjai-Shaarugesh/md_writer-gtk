# 🚀 Advanced Markdown Editor

This is a **comprehensive** markdown editor with *full GitHub support* including ~~strikethrough~~ and more!

## ✨ Features

- **Real-time Preview**: See your markdown rendered as you type
- **GitHub Flavored Markdown**: Full support for tables, alerts, and more
- **Syntax Highlighting**: Enhanced formatting for all elements
- **Responsive Design**: Works beautifully on all screen sizes

### Text Formatting

You can make text **bold**, *italic*, ~~strikethrough~~, and even `inline code`.

### Code Blocks

```rust
fn main() {
    println!("Hello, Markdown!");
    
    // This editor supports syntax highlighting
    let mut editor = MarkdownEditor::new();
    editor.render_preview();
}
```

```javascript
// JavaScript example
function renderMarkdown(text) {
    return pulldownCmark.parse(text);
}
```

### Lists and Tasks

#### Unordered Lists
- First item
- Second item
  - Nested item
  - Another nested item
- Third item

#### Ordered Lists
1. First step
2. Second step
3. Third step

### Tables

| Feature | Status | Priority |
|---------|--------|----------|
| Live Preview | ✅ Done | High |
| GitHub Alerts | ✅ Done | High |
| Tables | ✅ Done | Medium |
| Syntax Highlighting | 🚧 In Progress | Medium |

### GitHub Alerts

> [!NOTE]
> This is a note alert. It provides helpful information to users.

> [!TIP]
> This is a tip alert. It gives users a helpful suggestion or best practice.

> [!IMPORTANT]
> This is an important alert. It highlights crucial information that users must know.

> [!WARNING]
> This is a warning alert. It indicates potential issues or important considerations.