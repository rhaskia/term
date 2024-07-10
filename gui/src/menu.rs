mod settings;
mod keybinding;
use keybinding::Keybinds;
use dioxus::prelude::*;
use crate::{KEYBINDS, CONFIG};
use serde::{Serialize, Deserialize};
use dioxus_form::Form;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
struct Example {
    pub amount: i64,
    pub name: String,
    pub keybinds: Vec<i64>,
    pub nested: Example2,
    pub b: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
struct Example2 {
    pub amount: bool,
    pub name: String,
}

#[component]
pub fn Menu(active: bool) -> Element {
    // Temporary config
    let mut config = use_signal(|| CONFIG.cloned());
    let keybinds = use_signal(|| KEYBINDS().clone());
    let value = use_signal(|| Example {amount:56,name:"hello".to_string(),keybinds:vec![2,3,4,56],nested:Example2{amount:true,name:"john".to_string()}, b: false });

    rsx! {
        div {
            class: "menu",
            display: if active { "flex" } else { "none" },
            width: "100%",
            display: "flex",
            flex_direction: "column",
            id: "menu",
            div { 
              id: "menuheader", 
              class: "menuheader",
              h2 { "Settings" }, 
            }
            div {
                class: "menucontent",
                Form { value: config  }
                Keybinds { keybinds }
            }
            div {
                height: "20px",
                width: "100%",
                class: "savebar",
                button {
                    onclick: move |_| {
                        *CONFIG.write() = config();
                        *KEYBINDS.write() = keybinds();
                        config::save_keybinds(keybinds().clone());
                        // Save to file
                    },
                    "Save Config"
                }
                button {
                    "Open Config Folder"
                }
                button {
                    onclick: move |_| config.set(CONFIG()),
                    "Discard All"
                }
            }
        }
    }
}
