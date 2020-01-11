use headless_chrome::Browser;

const USAGE_ERROR: &str = "Need arguments for what to process.

Usage:

    get-icon [term] [path]

Example:

    get-icon madoka ./dist/icons/madoka";

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 3 {
        println!("{}", USAGE_ERROR);
        std::process::exit(1);
    }

    let term = &args[1];
    let path = &args[2];

    let browser = Browser::default().expect("Headless chrome failed to initialize.");

    let tab = browser
        .wait_for_initial_tab()
        .expect("Could not get initial tab.");

    let url = format!(
        "https://duckduckgo.com/?iax=images&ia=images&q={}+anime+wiki",
        term
    );

    tab.navigate_to(&url)
        .expect(&format!("Could not navigate to url: {}", url));

    let element = tab
        .wait_for_element(".tile--img__img")
        .expect("Could not get tile element");

    let attrs = element
        .get_attributes()
        .expect("Could not get attributes of element")
        .expect("Attributes of element was empty");

    let src = attrs
        .get("src")
        .expect("Element did not have a src attribute");

    println!("Src was {}", src);

    let download = std::process::Command::new("curl")
        .arg("-L")
        .arg(format!("https:{}", src))
        .arg("-o")
        .arg(path)
        .output()
        .expect("Failed to run wget to download icon");

    if download.status.success() {
        println!("Downloaded icon from {}", src);
        std::process::exit(0);
    } else {
        let stdout = String::from_utf8_lossy(&download.stdout);
        let stderr = String::from_utf8_lossy(&download.stderr);
        println!("Failed to run icon download: {}, {}", stdout, stderr);
        std::process::exit(1);
    }
}
