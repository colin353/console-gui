pub fn run<I, S>(args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let args_owned: Vec<_> = args.into_iter().collect();
    let mut args = vec!["-u", "colinwm"];
    args.extend(args_owned.iter().map(|a| a.as_ref()));
    if let Err(e) = std::process::Command::new("sudo").args(&args).spawn() {
        eprintln!("couldn't launch command! {:?}", e);
    }
}

pub fn open_url(url: &str) {
    if let Err(e) = std::process::Command::new("i3-msg")
        .args(&["exec", "/home/colinwm/bin/open-url.sh", url])
        .spawn()
    {
        eprintln!("couldn't open url! {:?}", e);
    }
}
