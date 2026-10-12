fn main() {
    println!("Starting {} v{}", wow_buddy::APP_NAME, wow_buddy::VERSION);
    dioxus::launch(wow_buddy::router::App);
}
