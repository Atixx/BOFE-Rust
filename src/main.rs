fn main() {
    use clap::{load_yaml, App};

    let yaml = load_yaml!("cli.yml");
    let m = App::from(yaml).get_matches();

    if m.is_present("email") {
        println!("Email was present")
    }


    // if let Some(mode) = m.value_of("mode") {

    // match mode {
    //         "vi" => println!("You are using vi"),
    //         "emacs" => println!("You are using emacs..."),
    //         _ => unreachable!(),
    //     }
    // } else {
    //     println!("--mode <MODE> wasn't used...");
    // }
}

// Use https://github.com/clap-rs/clap/blob/master/examples/17_yaml.yaml
// for additional args handling
