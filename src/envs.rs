use std::env;

pub use env::args;

macro_rules! declare_env {
    ($env:ident: $env_var:ident, $def:expr, $type:ty) => {
        pub fn $env() -> $type {
            match env::var("TDT_$env_var") {
                Ok(res) => res.into(),
                _ => $def.into()
            }
        }
    };
    ($env:ident: $env_var:ident, $def:expr) => {
        pub fn $env() -> String {
            match env::var("TDT_$env_var") {
                Ok(res) => res,
                _ => $def.to_string()
            }
        }
    }
}

declare_env!(server_host: SERVER_HOST, "127.0.0.1:3000");
declare_env!(html_folder: SERVER_HTML_FOLDER, "html");
declare_env!(thread_pool: THREAD_POOL, "4");
