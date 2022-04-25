use std::ops::{Deref, DerefMut};
use crate::log;
use crate::console::console_log;

#[derive(Clone)]
pub struct Document {
    pub title: String,
}

impl Document {
    pub fn from_web(doc: web_sys::Document) -> Self {
        Document {
            title: doc.title(),
        }
    }
}

pub struct DocumentObserver {
    original: Document,
    modifiable: Document,
}

impl DocumentObserver {
    pub fn from_web(doc: web_sys::Document) -> Self {
        let doc = Document::from_web(doc);
        Self {
            original: doc.clone(),
            modifiable: doc,
        }
    }
}

impl Deref for DocumentObserver {
    type Target = Document;

    fn deref(&self) -> &Self::Target {
        &self.modifiable
    }
}

impl DerefMut for DocumentObserver {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.modifiable
    }
}

impl Drop for DocumentObserver {
    fn drop(&mut self) {
        if self.modifiable.title != self.original.title {
            console_log!("document.title was updated from {} to {}", self.original.title, self.modifiable.title);
            web_sys::window().unwrap().document().unwrap().set_title(&self.modifiable.title.clone())
        }
    }
}