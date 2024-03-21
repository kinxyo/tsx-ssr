pub mod handlers;
pub mod file;
pub mod template;
pub mod macros;
/* 
enum PackageManager {
    YARN,
    NPM,
    BUN,
    PNPM,
}

struct App {
    entry: String,
    package_manager: PackageManager, // for when building the files
    client_path: String,
    port: u32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            entry: String::from("index."),
            package_manager: PackageManager::NPM,
            client_path: String::from("./react-src"),
            port: 3000
        }
    }
}

impl App {

    fn default() -> Self {
        
    }

    fn config() -> Self {

    }

} */