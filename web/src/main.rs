use dioxus::prelude::*;
use ui::App;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    // Initialize database before launching the app
    #[cfg(not(target_arch = "wasm32"))]
    {
        use api::db::{init_database, default_db_path};
        
        // Use tokio runtime to initialize async database
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        rt.block_on(async {
            let db_path = default_db_path();
            if let Err(e) = init_database(&db_path).await {
                eprintln!("Failed to initialize database: {}", e);
                std::process::exit(1);
            }
            println!("Database initialized at: {}", db_path);
        });
    }
    
    dioxus::launch(|| rsx! {
        App {
            favicon: FAVICON,
            include_bootstrap: true,
            include_fontawesome: true,
            include_theme: true,
        }
    });
}
