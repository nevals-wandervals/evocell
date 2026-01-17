#[cfg(feature = "sdl3")]
use evocell::client::sdl::AppSdl;

use evocell::client::traits::App;

pub fn main() {
    #[cfg(feature = "sdl3")]
    {
        let app = AppSdl::new();
        app.init().run();
    }
}
