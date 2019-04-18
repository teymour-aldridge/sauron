//#![deny(warnings)]
use app::App;
use sauron::html::attributes::*;
use sauron::html::events::*;
use sauron::html::*;
use sauron::DomUpdater;
use wasm_bindgen::prelude::*;

use crate::app::Msg;
use sauron::dom::Renderer;
use sauron::Component;
use std::cell::RefCell;
use std::rc::Rc;

mod app;

#[wasm_bindgen]
pub struct Client {
    renderer: Rc<Renderer<App, Msg>>,
}

/// Build using
/// ```sh
/// $ wasm-pack build --target no-modules
/// ```
///
#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Client {
        sauron::log("I see you!");
        let mut app = App::new();
        let body = sauron::body();
        let view = app.view();
        let rc_app = Rc::new(RefCell::new(app));
        let mut dom_updater = DomUpdater::new(view, body.as_ref());

        let renderer = Renderer {
            app: rc_app,
            dom_updater: Rc::new(RefCell::new(dom_updater)),
        };

        let rc_renderer = Rc::new(renderer);
        rc_renderer.dom_updater.borrow_mut().mount(&rc_renderer);

        Client {
            renderer: rc_renderer,
        }
    }

    /*
    #[wasm_bindgen]
    pub fn render(&mut self) {
        self.app.update();
        self.dom_updater.update(self.app.view());
    }
    */
}

#[wasm_bindgen]
extern "C" {
    pub type GlobalJS;
    pub static global_js: GlobalJS;
    #[wasm_bindgen(method)]
    pub fn update(this: &GlobalJS);
}

#[wasm_bindgen]
pub fn initialize() -> Client {
    let client = Client::new();
    client
}
