use std::path::Path;
use std::string::String;
use std::time::Duration;
use headless_chrome::{Browser, LaunchOptions, protocol::cdp::Page};
use std::fs;
use chrono;

pub struct Screenshot {
   urls: Vec<String>,
   folder_path: String
}

impl Screenshot {
    pub fn new(urls: Vec<String>, folder_path: String) -> Result<Self, String> {

        if !Path::is_dir(Path::new(folder_path.as_str())) {
            return Err("Failed to initialize screenshot".to_string());
        }

        if urls.len() == 0 {
            return Err("Urls cannot be an empty vector".to_string());
        }

        // do folder_path and urls check here
        Ok(Screenshot {
            urls,
            folder_path
        })
    }


    pub fn get_screenshots(&self) {
        let cloned = self.urls.clone();
        let folder_path = self.folder_path.clone();
        let browser = Self::init_headless_chrome()
        .expect("Failed to initialize chrome");

        for url in cloned {
            let _ = Self::take_screenshot(&browser, url.as_str(), folder_path.as_str());
        }
    }


    fn init_headless_chrome() -> Result<Browser, String> {
        let opts = LaunchOptions {
            headless: true,
            sandbox: true,
            enable_gpu: false,
            enable_logging: false,
            idle_browser_timeout: Duration::from_secs(30),
            window_size: Some((1000, 8500)),
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

        let browser = Browser::new(opts)
        .expect("Failed to initialized headless browser");

        Ok(browser)
    }

    fn take_screenshot(browser: &Browser, url: &str, folder_path: &str) -> Result<(), Box<dyn std::error::Error>>{

        let tab = browser.new_tab()
        .expect("Failed to open chrome tab");

        tab.set_default_timeout(std::time::Duration::from_secs(30));

        tab.navigate_to(url)
        .expect("Failed to open navigate to URL");

        tab.wait_until_navigated()
        .expect("Failed to open navigate to URL");

        let _png_data = tab.capture_screenshot(
            Page::CaptureScreenshotFormatOption::Jpeg,
            None,
            None,
            true,
            )
        .expect("Failed to capture screenshot");;

        let image_path = format!("{}/{}.jpeg", folder_path, chrono::Utc::now().timestamp());

        let _ = fs::write(image_path, _png_data);

        Ok(())

    }

}
