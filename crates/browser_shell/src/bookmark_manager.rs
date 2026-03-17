//! Bookmark manager with optional folder support.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub url: String,
    pub title: String,
    pub folder: Option<String>,
}

#[derive(Debug, Default)]
pub struct BookmarkManagerService {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkManagerService {
    pub fn add(
        &mut self,
        url: impl Into<String>,
        title: impl Into<String>,
        folder: Option<String>,
    ) {
        self.bookmarks.push(Bookmark {
            url: url.into(),
            title: title.into(),
            folder,
        });
    }

    pub fn all(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    pub fn by_folder(&self, folder: &str) -> Vec<Bookmark> {
        self.bookmarks
            .iter()
            .filter(|bookmark| bookmark.folder.as_deref() == Some(folder))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::BookmarkManagerService;

    #[test]
    fn supports_bookmark_folders() {
        let mut manager = BookmarkManagerService::default();
        manager.add("https://example.com", "Example", Some("Work".to_string()));
        manager.add("https://nust.dev", "NUST", Some("Learning".to_string()));

        assert_eq!(manager.all().len(), 2);
        assert_eq!(manager.by_folder("Work").len(), 1);
    }
}
