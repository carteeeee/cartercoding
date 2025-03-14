use js_sys::*;
use leptos::prelude::{Effect, *};
use web_sys::*;

use leptos::html::Canvas;
use leptos::task::spawn_local;
use std::future::Future;
use std::pin::pin;
use web_sys::wasm_bindgen::JsCast;

pub mod game;
pub mod material;
pub mod util;
use game::*;

use std::sync::Arc;
use std::task::Context;
use std::task::Wake;

// thank you thunder!
struct NoopWaker;
impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {}
}

#[component]
pub fn App() -> impl IntoView {
    let canvas_ref = NodeRef::<Canvas>::new();
    let (maybe_rom, set_rom) = signal(None);

    let handle_files_upload = move |ev: leptos::ev::Event| {
        let inp: HtmlInputElement = ev.target().unwrap().unchecked_into();

        if let Some(files) = inp.files() {
            let file = files.get(0).unwrap();
            let blob = file.slice().expect("File reading should not fail");

            spawn_local(async move {
                let file_raw_data = wasm_bindgen_futures::JsFuture::from(blob.array_buffer())
                    .await
                    .expect("this should not fail");

                let file_raw_data = file_raw_data
                    .dyn_into::<ArrayBuffer>()
                    .expect("Expected an ArrayBuffer");
                let file_raw_data = Uint8Array::new(&file_raw_data);

                let mut file_bytes = vec![0; file_raw_data.length() as usize];
                file_raw_data.copy_to(file_bytes.as_mut_slice());

                set_rom(Some(file_bytes));
            });
        };
    };

    Effect::new(move |_| {
        if let Some(rom) = maybe_rom() {
            if let Some(canvas) = canvas_ref.get_untracked() {
                let waker = Arc::new(NoopWaker).into();
                let mut cx = Context::from_waker(&waker);

                // this is done so the Future actually executes
                let _p = pin!(run(rom, canvas)).as_mut().poll(&mut cx);
            }
        }
    });

    view! {
        <p>"please input a US sm64 rom in the .z64 format!"</p>

        <input
            type="file"
            name="sm64"
            accept=".z64"
            on:input=handle_files_upload
        />

        <canvas style="position: absolute;top:0;bottom: 0;left: 0;right: 0;margin:auto;" node_ref=canvas_ref></canvas>
    }
}
