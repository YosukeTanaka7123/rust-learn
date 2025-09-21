enum OS {
    Windows(u32, String),
    MacOS(u32, String),
    Linux(u32, String),
}

pub fn run() {
    let windows = OS::Windows(1985, String::from("Microsoft"));
    let mac = OS::MacOS(2001, String::from("Apple"));
    let linux = OS::Linux(1991, String::from("Linus"));

    print_os_info(windows);
    print_os_info(mac);
    print_os_info(linux);
}

fn print_os_info(os: OS) {
    match os {
        OS::Windows(year, creator) => {
            println!("Windows was created in {} by {}.", year, creator);
        }
        OS::MacOS(year, creator) => {
            println!("MacOS was created in {} by {}.", year, creator);
        }
        OS::Linux(year, creator) => {
            println!("Linux was created in {} by {}.", year, creator);
        }
    }
}
