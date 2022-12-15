use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGlRenderingContext as GL, WebGlRenderingContext};
use yew::{html, Component, Context, Html, NodeRef};
use crate::components::ui;

// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct WebGPUContainer {
    node_ref: NodeRef,
}

impl Component for WebGPUContainer {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} />
        }
    }
}
