use config::TerminalAction;
use dioxus::prelude::*;
use log::log;
use crate::{handle_action, COMMAND_PALETTE};

#[component]
pub fn CommandPalette() -> Element {
    let commands = use_signal(|| TerminalAction::palette_usable());
    let mut search = use_signal(|| String::new());
    let commands_str = use_signal(|| {
        TerminalAction::palette_usable()
            .into_iter()
            .map(|a| a.readable())
    });
    let matches = use_memo(move || {
        commands()
            .into_iter()
            .filter(|c| c.readable().to_lowercase().starts_with(&search().to_lowercase()))
            .collect::<Vec<TerminalAction>>()
    });
    let mut raw_selected = use_signal(|| 0.0);
    let mut selected = use_memo(move || raw_selected() as usize);

    use_future(move || async move {
        wait_for_next_render().await;

        let mut clickoff = eval(
            r#"
            document.addEventListener('click', function(event) {
                const divElement = document.getElementById('commandpalette');
                if (divElement && !divElement.contains(event.target)) {
                    dioxus.send({});
                }
            });
        "#,
        );

        //*raw_selected.write() = (matches().len()) as f64;

        loop {
            clickoff.recv().await;
            handle_action(TerminalAction::ToggleCommandPalette);
        }
    });

    rsx! {
        div {
            class: "commandpalette",
            id: "commandpalette",
            input {
                class: "commandsearch",
                oninput: move |event| search.set(event.value()),
                onkeydown: move |e| match e.key() {
                    Key::ArrowUp if selected() != 0 => raw_selected -= 1.0,
                    Key::ArrowUp => *raw_selected.write() = matches.read().len() as f64 - 1.0,
                    Key::ArrowDown if selected() != matches.read().len() - 1 => raw_selected += 1.0,
                    Key::ArrowDown => *raw_selected.write() = 0.0,
                    Key::Enter => {
                        handle_action(matches.read()[selected()].clone());
                        *COMMAND_PALETTE.write() = false;
                    }
                    _ => {}
                }
            }
            select {
                class: "searchresults",
                id: "searchresults",
                size: 999,
                value: raw_selected,
                for (i, result) in matches().into_iter().enumerate() {
                    option {
                        class: "searchresult",
                        value: i as f64,
                        onclick: move |_| {
                            handle_action(matches.read()[i].clone());
                            *COMMAND_PALETTE.write() = false;
                        },
                        "{result}"
                    }
                }
            }
        }
    }
}
