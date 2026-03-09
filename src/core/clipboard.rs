use crate::AnyResult;
// #[cfg(not(target_arch = "wasm32"))]
// pub struct Clipboard {
//     context: clipboard_rs::ClipboardContext,
//     cached: Option<String>,
//     // join_handle: Option<JoinHandle<()>>,
//     // watcher_shutdown: Option<WatcherShutdown>,
//     count: usize,
// }

// #[cfg(target_arch = "wasm32")]
pub struct Clipboard {
    cached: Option<String>,
    count: usize,
}
/*
#[cfg(not(target_arch = "wasm32"))]
impl Clipboard {
    pub fn new() -> AnyResult<Self> {
        let clipboard = Self {
            context: anyhow(clipboard_rs::ClipboardContext::new_with_options(
                clipboard_rs::ClipboardContextX11Options { read_timeout: None },
            ))?,
            cached: None,
            // join_handle: None,
            // watcher_shutdown: None,
            count: 0,
        };
        // let mut watcher = ClipboardWatcherContext::new().unwrap();
        // let watcher_shutdown = watcher.add_handler(clipboard).get_shutdown_channel();
        // let handle = thread::spawn(move || {
        //     watcher.start_watch();
        // });
        // clipboard.join_handle = Some(handle);
        // clipboard.watcher_shutdown = Some(watcher_shutdown);
        Ok(clipboard)
    }

    pub fn copy(&mut self, text: String) -> AnyResult<()> {
        use clipboard_rs::Clipboard;
        self.cached = Some(text.clone());
        anyhow(self.context.set_text(text))
    }

    pub fn paste(&self) -> Option<&String> {
        self.cached.as_ref()
    }
    pub fn maybe_refresh(&mut self) -> AnyResult<()> {
        self.count += 1;
        if self.count > 20 {
            self.count = 0;
            // let start = Instant::now();
            let result = self.refresh();
            // println!(
            //     "refresh clipboard: elapsed: {}ms",
            //     (Instant::now() - start).as_millis()
            // );
            result
        } else {
            Ok(())
        }
    }
    pub fn refresh(&mut self) -> AnyResult<()> {
        use clipboard_rs::Clipboard;
        let text = anyhow(self.context.get_text())?;
        if text.len() == 0 {
            self.cached = None;
        } else {
            self.cached = Some(text)
        };
        Ok(())
    }
}*/

// #[cfg(target_arch = "wasm32")]
impl Clipboard {
    pub fn new() -> AnyResult<Self> {
        let clipboard = Self {
            cached: None,
            count: 0,
        };
        // let mut watcher = ClipboardWatcherContext::new().unwrap();
        // let watcher_shutdown = watcher.add_handler(clipboard).get_shutdown_channel();
        // let handle = thread::spawn(move || {
        //     watcher.start_watch();
        // });
        // clipboard.join_handle = Some(handle);
        // clipboard.watcher_shutdown = Some(watcher_shutdown);
        Ok(clipboard)
    }

    pub fn copy(&mut self, text: String) -> AnyResult<()> {
        self.cached = Some(text.clone());
        // miniquad::window::clipboard_set(&text); // doesn't work in linux nor wasm
        Ok(())
    }

    pub fn paste(&self) -> Option<&String> {
        self.cached.as_ref()
    }
    pub fn maybe_refresh(&mut self) -> AnyResult<()> {
        self.count += 1;
        if self.count > 20 {
            self.count = 0;
            // let start = Instant::now();
            let result = self.refresh();
            // println!(
            //     "refresh clipboard: elapsed: {}ms",
            //     (Instant::now() - start).as_millis()
            // );
            result
        } else {
            Ok(())
        }
    }
    pub fn refresh(&mut self) -> AnyResult<()> {
        // self.cached = miniquad::window::clipboard_get(); // doesn't work in wasm
        Ok(())
    }
}

// #[cfg(not(target_arch = "wasm32"))]
// impl clipboard_rs::ClipboardHandler for Clipboard {
//     fn on_clipboard_change(&mut self) {}
// }
// fn anyhow<T, E: Display>(result: Result<T, E>) -> AnyResult<T> {
//     result.map_err(|e| e.to_string().into())
// }
