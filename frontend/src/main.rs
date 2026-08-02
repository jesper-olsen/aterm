mod commands;

use chrono::Local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

fn process(command: &str) -> String {
    let words: Vec<String> = command.split_whitespace().map(String::from).collect();
    if words.is_empty() {
        return String::new();
    }
    match commands::run(&words[0], &words[1..]) {
        Some(output) => output,
        None => format!("-bash: {command}: command not found"),
    }
}

#[function_component(TerminalApp)]
fn terminal_app() -> Html {
    let new_mail = use_state(|| true);
    let history = use_state(|| {
        let now = Local::now().format("%a %b %d %H:%M:%S").to_string();
        let storage = web_sys::window().and_then(|w| w.local_storage().ok().flatten());
        let last_login = storage
            .as_ref()
            .and_then(|s| s.get_item("last_login").ok().flatten());
        if let Some(s) = &storage {
            let _ = s.set_item("last_login", &now);
        }

        let mut lines = vec!["Unix Version 7\nHostname: ygdrasil.jesperolsen.com".to_string()];
        if let Some(prev) = last_login {
            lines.push(format!("Last login: {}", prev));
        }
        lines
    });
    let input = use_state(String::new);

    // Command history for up/down arrow navigation.
    let cmd_history = use_state(Vec::<String>::new);
    // None = not currently navigating; Some(i) = showing the i-th most recent
    // command, where 0 is the most recent.
    let history_pos = use_state(|| None::<usize>);
    // Whatever was typed before the user started navigating history, restored
    // when they arrow back down past the most recent command.
    let draft = use_state(String::new);

    let onkeydown = {
        let history = history.clone();
        let input = input.clone();
        let new_mail = new_mail.clone();
        let cmd_history = cmd_history.clone();
        let history_pos = history_pos.clone();
        let draft = draft.clone();
        Callback::from(move |event: KeyboardEvent| match event.key().as_str() {
            "Enter" => {
                event.prevent_default();
                let command = (*input).clone();
                input.set(String::new());
                history_pos.set(None);
                draft.set(String::new());

                if !command.trim().is_empty() {
                    let mut ch = (*cmd_history).clone();
                    ch.push(command.clone());
                    cmd_history.set(ch);
                }

                if command.trim() == "clear" {
                    history.set(vec![]);
                    return;
                }

                let mut new_history = (*history).clone();
                new_history.push(format!("> {}", command));
                let output = process(&command);
                new_history.push(output);
                if *new_mail {
                    new_history.push(String::from("You have new mail\n"));
                    new_mail.set(false);
                }
                history.set(new_history);
            }
            "ArrowUp" => {
                event.prevent_default();
                let len = cmd_history.len();
                if len == 0 {
                    return;
                }
                let next_pos = match *history_pos {
                    None => {
                        draft.set((*input).clone());
                        0
                    }
                    Some(p) if p + 1 < len => p + 1,
                    Some(p) => p,
                };
                input.set(cmd_history[len - 1 - next_pos].clone());
                history_pos.set(Some(next_pos));
            }
            "ArrowDown" => {
                event.prevent_default();
                match *history_pos {
                    None => {}
                    Some(0) => {
                        input.set((*draft).clone());
                        history_pos.set(None);
                    }
                    Some(p) => {
                        let next_pos = p - 1;
                        let len = cmd_history.len();
                        input.set(cmd_history[len - 1 - next_pos].clone());
                        history_pos.set(Some(next_pos));
                    }
                }
            }
            _ => {}
        })
    };

    let input_ref = use_node_ref();
    let ir = input_ref.clone();
    use_effect_with((), move |_| {
        gloo::timers::callback::Timeout::new(0, move || {
            if let Some(input) = ir.cast::<HtmlInputElement>() {
                input.focus().expect("Failed to focus the input element");
            }
        })
        .forget();
        || () // Cleanup function
    });

    html! {
        <div style="background: black; color: green; font-family: monospace; padding: 10px;">
            <div>
               { for (*history).iter().map(|line| html! { <pre>{ line }</pre> }) }
            </div>
            <div>
                <span>{ "> " }</span>
                <input
                    ref={input_ref} // Attach the NodeRef
                    type="text"
                    placeholder=""
                    style="background: black; color: green; border: none; outline: none; font-family: monospace; width: 80%;"
                    value={(*input).clone()}
                    onkeydown={onkeydown} // bind the callback
                    oninput={Callback::from(move |e: InputEvent| {
                        let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                        input.set(value);
                    })}
                />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<TerminalApp>::new().render();
}
