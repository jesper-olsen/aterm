use chrono::Local;
use gloo::timers::callback::Interval;
use web_sys::HtmlInputElement;
use yew::prelude::*;

static PWD: &str = "/home/jesper";
static LS: &str = "haiku.txt";
static LL: &str = "-rw-rw-r--  1 root  staff  71 Nov 29  1996 haiku.txt";

static HAIKU: &str = "An old silent pond\n\n\
                      A frog jumps into the pond\n\n\
                      Splash! Silence again.";

static FINGER: &str =
    "Login            Name             TTY      Idle  Login  Time   Office  Phone\n\
                       jesper           Jesper Olsen    *console    6d  Nov 21 08:23";

static MAIL: &str = "\"/var/mail/user\": 1 message \n\
      N 1 jesper.olsen@gmail.com     Wed Nov 27  12/480  Subject: Hello \n\
      \n\
      Greetings valued netizen - not much to see here at the moment. \n\
      \n\
      -Jesper\n\
      https://github.com/jesper-olsen";

fn process(command: &str) -> String {
    let now = Local::now();
    // Format it to "Thu Nov 23 10:21"
    let date = now.format("%a %b %d %H:%M").to_string();

    let words: Vec<String> = command.split_whitespace().map(String::from).collect();
    if words.is_empty() {
        return String::new();
    }
    match words[0].as_str() {
        "ll" => String::from(LL),
        "ls" if words.len() > 1 && words[1] == "-l" => String::from(LL),
        "ls" | "dir" => String::from(LS),
        "pwd" => String::from(PWD),
        "date" => date,
        "mail" => format!("{}", MAIL),
        "finger" => format!("{}", FINGER),
        "more" | "cat" if words.len() > 1 => {
            if words[1] == "haiku.txt" {
                format!("{}", HAIKU)
            } else {
                format!("{}: No such file or directory", words[1])
            }
        }

        _ => format!("-bash: {command}: command not found"),
    }
}

#[function_component(TerminalApp)]
fn terminal_app() -> Html {
    let new_mail = use_state(|| true);
    let history = use_state(|| {
        vec!["Welcome to the terminal\nHostname: ygdrasil.jesperolsen.com".to_string()]
    });
    let input = use_state(|| String::new());
    let cursor_visible = use_state(|| true);

    //let input_ref = use_node_ref();

    // Handle user input
    let onkeydown = {
        let history = history.clone();
        let input = input.clone();
        Callback::from(move |event: KeyboardEvent| {
            if event.key() == "Enter" {
                event.prevent_default();
                let command = (*input).clone();
                input.set("".to_string());

                // Process the command and update history
                let mut new_history = (*history).clone();
                new_history.push(format!("> {}", command));
                let output = process(&command);
                new_history.push(output); //format!("Response to '{}'", command));
                if *new_mail {
                    new_history.push(String::from("You have new mail\n"));
                    new_mail.set(false);
                }
                history.set(new_history);
            }
        })
    };

    // Cursor blinking
    {
        let cursor_visible = cursor_visible.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(500, move || {
                cursor_visible.set(!*cursor_visible);
            });
            move || drop(interval)
        });
    }

    let cursor = if *cursor_visible { "|" } else { " " };

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
               //{ for (*history).iter().map(|line| html! { <div>{ line }</div> }) }
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
                    onkeydown={onkeydown} // Correctly bind the Callback here
                    oninput={Callback::from(move |e: InputEvent| {
                        let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                        input.set(value);
                    })}
                />
                <span>{ cursor }</span>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<TerminalApp>::new().render();
}
