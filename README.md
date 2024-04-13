**Saqif is a simple application to quickly take screenshots of any given URL that is publicly accessible**

## Saqif in action

```rust
use saqif::screenshot;


fn main() {
    let urls = vec![
        String::from("https://doc.rust-lang.org/reference/visibility-and-privacy.html"),
        String::from("https://doc.rust-lang.org/reference/names/namespaces.html"),
        String::from("https://doc.rust-lang.org/reference/paths.html")
    ];

    let file_path = String::from("./examples");

    let r = screenshot::Screenshot::new(urls, file_path)
    .expect("Failed to init");

    r.get_screenshots();
}
```