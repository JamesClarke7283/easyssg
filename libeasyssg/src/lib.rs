use pulldown_cmark::{html, Parser};
use std::collections::HashMap;

/// Represents the content of a page.
pub struct Page {
    markdown: String,
    html: String,
}

impl Page {
    #[must_use]
    pub fn new(markdown: &str) -> Self {
        let mut page = Self {
            markdown: markdown.to_string(),
            html: String::new(),
        };
        page.build();
        page
    }

    fn build(&mut self) {
        let parser = Parser::new(&self.markdown);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        self.html = html_output;
    }

    #[must_use]
    pub fn get_html(&self) -> &str {
        &self.html
    }
}

/// Represents a node in the site structure, which can be either a Page or a Folder.
pub enum SiteNode {
    Page(Page),
    Folder(HashMap<String, SiteNode>),
}

/// Represents the entire site, containing a root folder which in turn can contain pages and subfolders.
pub struct Site {
    root: HashMap<String, SiteNode>,
}

impl Default for Site {
    fn default() -> Self {
        Self::new()
    }
}

impl Site {
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: HashMap::new(),
        }
    }

    /// Adds a page to the site at the specified path.
    /// Path is a vector of string slices, representing the hierarchical path to the page.
    /// # Panics
    /// The `add_page` function panics if a page already exists with the same path and name.
    pub fn add_page(&mut self, path: &[&str], page: Page) {
        let mut current = &mut self.root;

        // Iterate through the path to find or create the correct location for the page.
        for &segment in path.iter().take(path.len() - 1) {
            current = match current
                .entry(segment.to_string())
                .or_insert_with(|| SiteNode::Folder(HashMap::new()))
            {
                SiteNode::Folder(folder) => folder,
                SiteNode::Page(_) => panic!("A page already exists at this path segment"),
            };
        }

        let page_name = (*path.last().unwrap()).to_string();
        current.insert(page_name, SiteNode::Page(page));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_page_to_root() {
        // Lower Bound Case: Adding directly to root
        let mut site = Site::new();
        let page = Page::new("Markdown content for root page");
        site.add_page(&["root_page"], page);

        assert!(matches!(
            site.root.get("root_page"),
            Some(SiteNode::Page(_))
        ));
    }

    #[test]
    fn test_add_page_to_mid_level_folder() {
        // Normal Case: Adding to a mid-level folder
        let mut site = Site::new();
        let page = Page::new("Markdown content for mid-level page");
        site.add_page(&["folder1", "folder2", "mid_page"], page);

        if let Some(SiteNode::Folder(folder)) = site.root.get("folder1") {
            if let Some(SiteNode::Folder(subfolder)) = folder.get("folder2") {
                assert!(matches!(subfolder.get("mid_page"), Some(SiteNode::Page(_))));
            } else {
                panic!("Subfolder folder2 not found");
            }
        } else {
            panic!("Folder folder1 not found");
        }
    }

    #[test]
    fn test_add_page_to_deeply_nested_folder() {
        // Upper Bound Case: Adding to a deeply nested folder
        let mut site = Site::new();
        let page = Page::new("Markdown content for deeply nested page");
        site.add_page(&["lvl1", "lvl2", "lvl3", "lvl4", "deep_page"], page);

        let mut current = &site.root;
        for &level in ["lvl1", "lvl2", "lvl3", "lvl4"].iter() {
            match current.get(level) {
                Some(SiteNode::Folder(folder)) => current = folder,
                _ => panic!("Expected folder not found at level {}", level),
            }
        }
        assert!(matches!(current.get("deep_page"), Some(SiteNode::Page(_))));
    }
}
