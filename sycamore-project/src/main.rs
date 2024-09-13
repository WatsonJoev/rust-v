use sycamore::prelude::*;

fn main() {
    sycamore::render(|cx| view! { cx,
        Home()
    });
}

#[component]
fn Home<'a, G: Html>(cx: Scope) -> View<G> {
    let text = create_signal(cx, "".to_string());
    let logo = create_signal(cx, "".to_string());
    let color = create_signal(cx, "".to_string());
    let src = create_signal(cx, "https://img.shields.io/badge/default-000".to_string());

    view! { cx,
        div(class="mx-auto p-4 bg-gray-100 h-screen flex justify-center") {
            div(class="mx-auto flex flex-col") {
                label(class="block mt-2 text-sm font-medium text-gray-900 dark:text-white") {
                    "Text"
                }
                input(
                    name="Text",
                    class="mb-3 pl-3 mx-auto",
                    bind:value=text
                )
                label(class="block mt-2 text-sm font-medium text-gray-900 dark:text-white") {
                    "Logo"
                }
                input(
                    name="Logo",
                    class="mb-3 pl-3 mx-auto",
                    bind:value=logo
                ) {}
                label(class="block mt-2 text-sm font-medium text-gray-900 dark:text-white") {
                    "Color#"
                }
                input(
                    name="Color#",
                    class="mb-3 pl-3 mx-auto",
                    bind:value=color
                ) {}
                button(
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 rounded",
                    on:click=move |_| {
                        let new_src = format!(
                            "https://img.shields.io/badge/{}-{}?logo={}",
                            text, color, logo
                        );
                        src.set(new_src);
                    }
                ) {
                    "Update Src!"
                }
                img(class="mt-5", src=src.get(), alt=src.get()) {}
            }
        }
    }
}