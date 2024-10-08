use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use react_reconciler::fiber::FiberRootNode;
use react_reconciler::Reconciler;

#[wasm_bindgen]
pub struct Renderer {
    root: Rc<RefCell<FiberRootNode>>,
    reconciler: Reconciler,
}

impl Renderer {
    pub fn new(root: Rc<RefCell<FiberRootNode>>, reconciler: Reconciler) -> Self {
        Self { root, reconciler }
    }
}

#[wasm_bindgen]
impl Renderer {
    pub fn render(&self, element: &JsValue) {
        self.reconciler
            .update_container(Rc::new(element.clone()), self.root.clone())
    }
}
