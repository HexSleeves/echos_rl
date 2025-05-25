pub struct AppConstants;

impl AppConstants {
    /// Setup Folders to point to assets folder instead of the program root directory
    pub const BASE: &str = "./assets";

    /// Name of the company
    pub const COMPANY: &str = "HexSleeves";

    /// Name of the domain
    pub const DOMAIN: &str = "echosinthedark";

    /// Name of the app
    pub const APP_NAME: &str = "Echos in the Dark";

    /// Width of the window
    pub const WINDOW_WIDTH: f32 = 800.0;

    /// Height of the window
    pub const WINDOW_HEIGHT: f32 = 600.0;
}
