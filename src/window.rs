use crate::document::DocumentObserver;

pub struct Window {
    inner: web_sys::Window,
}

pub fn window() -> Window {
    Window {
        inner: web_sys::window().expect("no global `window` exists"),
    }
}

impl Window {
    pub fn document(&self) -> DocumentObserver {
        let web_sys_doc = self.inner.document().expect("should have a document on window");
        DocumentObserver::from_web(web_sys_doc)
    }
}
