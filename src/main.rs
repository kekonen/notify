// #[cfg(target_os = "macos")] 
use notify_rust::Notification;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Binary for desktop notifications")
        .version("0.1")
        .author("Daniil Naumetc <daniil.naumetc@gmail.com>")
        .about("Shows desktop notifications")
        .arg(Arg::with_name("TITLE")
            .short("t")
            .long("title")
            .value_name("TITLE")
            .help("Sets title")
            .takes_value(true))
        .arg(Arg::with_name("BODY")
            .short("b")
            .long("BODY")
            .help("Sets body")
            .takes_value(true)
            // .index(1)
        )
        .arg(Arg::with_name("ICON")
            .short("i")
            .long("ICON")
            .help("Sets icon (can be a path)")
            .takes_value(true))
        .arg(Arg::with_name("JUSTBODY").multiple(true))
        .get_matches();

    let title = matches.value_of("TITLE").unwrap_or("");
    let flag_body = matches.value_of("BODY").unwrap_or("");
    let icon = matches.value_of("ICON").unwrap_or("");
    let positional_body = if matches.is_present("JUSTBODY") {
        let positional_body_multiple: Vec<_> = matches.values_of("JUSTBODY").unwrap().collect();
        if positional_body_multiple.len() > 1 {
            String::from(positional_body_multiple.join(" "))
        } else {
            String::from(*positional_body_multiple.get(0).unwrap_or(&""))
        }
    } else {
        String::from("")
    };
    
    // let positional_body = matches.value_of("JUSTBODY").unwrap_or("");

    let body = if flag_body == "" {
        &positional_body
    } else {
        flag_body
    };

    // println!("title:{}\nicon:{}\nflag_body:{}\npositional_body:{}\nbody:{}", title, icon, flag_body, positional_body, body);

    Notification::new()
    .summary(title)
    .body(body)
    .icon(icon)
    .show()
    .expect("Couln't show the notigfication");
}
