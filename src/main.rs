mod components;
use components::canvas_container::CanvasContainer;
use components::webgpu_container::WebGPUContainer;
// use components::ui;
use yew::prelude::*;
// mod ui;

#[function_component]
fn HelloWorld() -> Html {
    html! { <div id="wasm-example"><div class="wasm-example"/></div> }
}

fn main() {
    // yew::Renderer::<HelloWorld>::new().render();
    // yew::Renderer::<CanvasContainer>::new().render();
    yew::Renderer::<WebGPUContainer>::new().render();
    // ui::run();
    // pollster::block_on(ui::run());

}