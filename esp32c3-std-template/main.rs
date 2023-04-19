use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    esp_idf_sys::link_patches();
    println!("Hello, world!");
}
