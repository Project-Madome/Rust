use std::convert::From;
use std::sync::Arc;

use serde::{Deserialize, Serialize};

use super::{ContentType, Language, Metadata};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub groups: Vec<String>,
    pub artists: Vec<String>,
    pub series: Vec<String>,
    pub tags: Vec<String>,
    pub characters: Vec<String>,
    pub language: String,
    #[serde(rename(serialize = "type"))]
    pub content_type: String,
    pub created_at: String,
    pub page_count: usize,
}

#[derive(Debug, PartialEq)]
pub struct MetadataBook {
    pub id: Metadata,
    pub title: Metadata,
    pub groups: Metadata,
    pub artists: Metadata,
    pub series: Metadata,
    pub tags: Metadata,
    pub characters: Metadata,
    pub language: Metadata,
    pub content_type: Metadata,
    pub created_at: Metadata,
    pub thumbnail_url: Metadata,
    pub page_count: Metadata,
}

impl From<MetadataBook> for Book {
    fn from(metadata_book: MetadataBook) -> Book {
        let content_type: ContentType = metadata_book.content_type.into();
        let language: Language = metadata_book.language.into();

        Book {
            id: metadata_book.id.into(),
            title: metadata_book.title.into(),
            groups: metadata_book.groups.into(),
            artists: metadata_book.artists.into(),
            series: metadata_book.series.into(),
            tags: metadata_book.tags.into(),
            characters: metadata_book.characters.into(),
            language: language.into(),
            content_type: content_type.into(),
            created_at: metadata_book.created_at.into(),
            page_count: metadata_book.page_count.into(),
        }
    }
}

impl From<Arc<Book>> for Book {
    fn from(arc_book: Arc<Book>) -> Self {
        let arc_book = Arc::try_unwrap(arc_book).unwrap();

        Self {
            id: arc_book.id,
            title: arc_book.title,
            groups: arc_book.groups,
            artists: arc_book.artists,
            series: arc_book.series,
            tags: arc_book.tags,
            characters: arc_book.characters,
            language: arc_book.language,
            content_type: arc_book.content_type,
            created_at: arc_book.created_at,
            page_count: arc_book.page_count,
        }
    }
}
