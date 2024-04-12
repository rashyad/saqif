use std::{path::Path, time::Duration};
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page};
use std::fs;

fn main() -> Result<(),  Box<dyn std::error::Error>> {

    let opts = LaunchOptions {
        headless: false,
        sandbox: true,
        enable_gpu: false,
        enable_logging: false,
        idle_browser_timeout: Duration::from_secs(30),
        window_size: None,
        path: None,
        user_data_dir: None,
        port: None,
        ignore_certificate_errors: true,
        extensions: Vec::new(),
        process_envs: None,
        args: Vec::new(),
        disable_default_args: false,
        proxy_server: None,
    };

    let mut path_to_save = Path::new("./");

    print!("{:?}", path_to_save);

   ;
    print!("{:?}", Path::is_dir(path_to_save));

    // let url = "https://en.wikipedia.org/wiki/WebKit";

    // let browser = Browser::new(opts)
    // .expect("Failed to initialized headless browser");

    // let tab = browser.new_tab()
    // .expect("Failed to initialized headless browser");

    // let viewport = tab.navigate_to(url)?
    //     .wait_for_element("#mw-content-text > div > table.infobox.vevent")?
    //     .get_box_model()?
    //     .margin_viewport();
    // let _png_data = tab.capture_screenshot(
    //     Page::CaptureScreenshotFormatOption::Png,
    //     Some(75),
    //     None,
    //     true)?;

    // fs::write("./image.png", _png_data)
    // .unwrap();

    Ok(())
}


