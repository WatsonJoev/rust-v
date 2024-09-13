use leptos::*;

fn main() {
    leptos::mount_to_body(Home);
}


#[component]
fn App() -> impl IntoView {
    view!{
        <p>{"Hello World!"}</p>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    let (text, set_text) = create_signal("".to_string());
    let (logo, set_logo) = create_signal("".to_string());
    let (color, set_color) = create_signal("".to_string());
    let (src, set_src) = create_signal("https://img.shields.io/badge/default-000".to_string());

    view! {
        <link rel="stylesheet" href="https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" />
        <div class="mx-auto p-4 bg-gray-100 h-screen flex justify-center">
            <div class="mx-auto flex flex-col">
                <label class="block mt-2 text-sm font-medium text-gray-900 dark:text-white">
                    "Text"
                </label>
                <input
                    name="Text"
                    class="mb-3 pl-3 mx-auto"
                    value=text.get()
                    on:input=move |e| set_text.set(event_target_value(&e))
                />
                <label class="block mt-2 text-sm font-medium text-gray-900 dark:text-white">
                    "Logo"
                </label>
                <input
                    name="Logo"
                    class="mb-3 pl-3 mx-auto"
                    value=logo.get()
                    on:input=move |e| set_logo.set(event_target_value(&e))
                />
                <label class="block mt-2 text-sm font-medium text-gray-900 dark:text-white">
                    "Color#"
                </label>
                <input
                    name="Color#"
                    class="mb-3 pl-3 mx-auto"
                    value=color.get()
                    on:input=move |e| set_color.set(event_target_value(&e))
                />
                <button
                    class="bg-blue-500 hover:bg-blue-700 text-white font-bold p-2 rounded"
                    on:click=move |_| {
                        let new_src = format!(
                            "https://img.shields.io/badge/{}-{}?logo={}",
                            text.get(), color.get(), logo.get()
                        );
                        set_src.set(new_src);
                    }
                >
                    "Update Src!"
                </button>
                <img class="mt-5" src=src alt=src />
            </div>
        </div>
    }
}