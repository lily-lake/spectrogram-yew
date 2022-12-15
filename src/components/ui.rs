use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit::*;
use gloo::console::log;

#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

// #[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
pub fn run() {
    cfg_if::cfg_if! {
        if #[cfg(target_arch = "wasm32")] {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
        } else {
            env_logger::init();
        }
    }
    log!("passed cfg_if block");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;
        window.set_inner_size(PhysicalSize::new(450, 400));
        log!("set window inner size");
        
        use winit::platform::web::WindowExtWebSys;
        log!("using winit websys");
    
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.body()?;
                // let dst = doc.get_element_by_id("wasm-example")?;
                log!("got wasm-example id");
                let canvas = web_sys::Element::from(window.canvas());
                log!("created canvas");
                dst.append_child(&canvas).ok()?;
                log!("appended canvas");
                // Some(c) => log!("got some value: {:?}", c),
                // None => log!("No value found")
                Some(())
            })
            .expect("Couldn't append canvas to document body.");
    }

    log!("outside canvas append section");

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
    log!("after event_loop.run");
}


// // #[cfg_attr(target_arch="wasm32", wasm_bindgen(start))]
// pub fn run() {
//     cfg_if::cfg_if! {
//         if #[cfg(target_arch = "wasm32")] {
//             std::panic::set_hook(Box::new(console_error_panic_hook::hook));
//             console_log::init_with_level(log::Level::Warn).expect("Couldn't initialize logger");
//         } else {
//             env_logger::init();
//         }
//     }
//     log!("passed cfg_if block");

//     let event_loop = EventLoop::new();
//     let window = WindowBuilder::new()
//         .build(&event_loop)
//         .unwrap();
//     log!("built window");
//     // #[cfg(target_arch = "wasm32")]
//     {
//         let win = web_sys::window()
//         .expect("window found");
//         log!("win: {:#?}", &win);
//         // let doc = win.document();
//         let doc = win.document()
//         .expect("document not found");
//         let body = doc.body().expect("document expect to have have a body");
//         log!("body: {:?}", &body);
//         // let dst = body.first_child().expect("no first child");
//         // log!("dst: {:?}", &dst);
//         // let val = doc.get_element_by_id("wasm-example")
// 		// // .unwrap()
// 		// // .dyn_into::<web_sys::HtmlElement>()
// 		// .unwrap();
//         // log!("val: {:?}", &val);
//         // match doc {
//         //     Some(x) => log!("result: {:?}", x),
//         //     None => log!("problem getting document"),
//         // }
//         // let dst = doc.get_element_by_id("wasm-example");
//         // // .expect("got canvas elmeent");
//         // match &dst {
//         //     Some(x) => log!("result: {:?}", x),
//         //     None => log!("problem getting canvas element"),
//         // }
//         let dsts = doc.get_elements_by_class_name("wasm-example");
//         // .expect("error getting elements by class name");
//         log!("dsts: {:?}", &dsts);
//         let dst = dsts.get_with_index(0);
//         match &dst {
//             Some(x) => log!("result: {:?}", x),
//             None => log!("problem getting canvas element"),
//         }
//         // .expect();
//         // match &dsts {
//         //     Some(x) => log!("result: {:?}", x),
//         //     None => log!("problem getting elements by class name"),
//         // }
//         log!("canvas: {:?}", dst);
//     }
//     // {
//     //     // Winit prevents sizing with CSS, so we have to set
//     //     // the size manually when on web.
//     //     use winit::dpi::PhysicalSize;
//     //     window.set_inner_size(PhysicalSize::new(450, 400));
//     //     log!("set window inner size");
        
//     //     use winit::platform::web::WindowExtWebSys;
//     //     log!("using winit websys");
    
//     //     web_sys::window()
//     //         .and_then(|win| win.document())
//     //         .and_then(|doc| {
//     //             let dst = doc.get_element_by_id("wasm-example")?;
//     //             log!("got wasm-example id");
//     //             let canvas = web_sys::Element::from(window.canvas());
//     //             log!("created canvas");
//     //             dst.append_child(&canvas).ok()?;
//     //             log!("appended canvas");
//     //             // Some(c) => log!("got some value: {:?}", c),
//     //             // None => log!("No value found")
//     //             Some(())
//     //         })
//     //         .expect("Couldn't append canvas to document body.");
//     // }

//     log!("outside canvas append section");

//     event_loop.run(move |event, _, control_flow| match event {
//         Event::WindowEvent {
//             ref event,
//             window_id,
//         } if window_id == window.id() => match event {
//             WindowEvent::CloseRequested
//             | WindowEvent::KeyboardInput {
//                 input:
//                     KeyboardInput {
//                         state: ElementState::Pressed,
//                         virtual_keycode: Some(VirtualKeyCode::Escape),
//                         ..
//                     },
//                 ..
//             } => *control_flow = ControlFlow::Exit,
//             _ => {}
//         },
//         _ => {}
//     });
//     log!("after event_loop.run");
// }