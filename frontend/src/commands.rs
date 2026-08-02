use shunting_yard::eval;

static PWD: &str = "/home/jesper";
static HOME_LS: &str = "bin\nhaiku.txt";
static HOME_LL: &str = "drwxr-xr-x  2 jesper  staff    64 Nov 29  1996 bin\n\
                         -rw-rw-r--  1 root    staff    71 Nov 29  1996 haiku.txt";
static BIN_LS: &str = "calc\ncat\ndate\nfinger\nls\nmail\npwd";
static BIN_LL: &str = "-rwxr-xr-x  1 root  staff  137312 Nov 29  1996 calc\n\
                        -rwxr-xr-x  1 root  staff  118992 Nov 29  1996 cat\n\
                        -rwxr-xr-x  1 root  staff  135392 Nov 29  1996 date\n\
                        -rwxr-xr-x  1 root  staff  137312 Nov 29  1996 finger\n\
                        -rwxr-xr-x  1 root  staff  154624 Nov 29  1996 ls\n\
                        -rwxr-xr-x  1 root  staff  270976 Nov 29  1996 mail\n\
                        -rwxr-xr-x  1 root  staff  108492 Nov 29  1996 pwd";
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

fn ls_listing(target: Option<&str>, long: bool) -> String {
    match (target, long) {
        (None, false) => String::from(HOME_LS),
        (None, true) => String::from(HOME_LL),
        (Some("bin"), false) => String::from(BIN_LS),
        (Some("bin"), true) => String::from(BIN_LL),
        (Some(other), _) => format!("{other}: No such file or directory"),
    }
}

pub fn run(cmd: &str, args: &[String]) -> Option<String> {
    let target = || args.iter().find(|a| a.as_str() != "-l").map(String::as_str);
    match cmd {
        "ll" => Some(ls_listing(target(), true)),
        "ls" => Some(ls_listing(target(), args.iter().any(|a| a == "-l"))),
        "dir" => Some(ls_listing(target(), false)),
        "pwd" => Some(String::from(PWD)),
        "date" => Some(chrono::Local::now().format("%a %b %d %H:%M").to_string()),
        "mail" => Some(String::from(MAIL)),
        "finger" => Some(String::from(FINGER)),
        "more" | "cat" if !args.is_empty() => Some(match args[0].as_str() {
            "haiku.txt" => String::from(HAIKU),
            name => format!("{name}: No such file or directory"),
        }),
        "calc" | "bc" if !args.is_empty() => Some(match eval(&args.join(" ")) {
            Ok(v) => v.to_string(),
            Err(e) => format!("calc: {e}"),
        }),
        "calc" | "bc" => Some(String::from("usage: calc <expression>")),
        _ => None,
    }
}
