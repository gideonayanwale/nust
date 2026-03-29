//! Desktop bookmark tab renderer with movable position.

use crate::bookmark_manager::BookmarkManagerService;
use crate::settings_system::BookmarkTabPosition;

pub struct BookmarkTab;

impl BookmarkTab {
    pub fn render(position: &BookmarkTabPosition, bookmarks: &BookmarkManagerService) -> String {
        let position_class = match position {
            BookmarkTabPosition::Top => "top",
            BookmarkTabPosition::Left => "left",
            BookmarkTabPosition::Right => "right",
            BookmarkTabPosition::Bottom => "bottom",
        };

        let bookmark_items = bookmarks
            .all()
            .iter()
            .map(|bookmark| {
                format!(
                    "<a class=\"chip\" href=\"{}\" style=\"text-decoration:none;\">{}</a>",
                    bookmark.url, bookmark.title
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "<aside id=\"bookmark-tab\" data-position=\"{position_class}\" style=\"display:flex;gap:8px;flex-wrap:wrap;margin:12px 0;\">{bookmark_items}</aside>"
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::bookmark_manager::BookmarkManagerService;
    use crate::settings_system::BookmarkTabPosition;

    use super::BookmarkTab;

    #[test]
    fn renders_bookmark_tab_with_position() {
        let mut bookmarks = BookmarkManagerService::default();
        bookmarks.add("https://nust.dev", "NUST", None);
        let html = BookmarkTab::render(&BookmarkTabPosition::Left, &bookmarks);
        assert!(html.contains("bookmark-tab"));
        assert!(html.contains("data-position=\"left\""));
    }
}
