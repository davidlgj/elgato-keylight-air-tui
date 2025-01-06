pub mod app;
pub mod keylight;

pub use app::App;

use clap::Parser;

use keylight::fetch_light_status;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// IP address of the Elgato key-light
    #[arg(short, long, value_name = "IP ADRESS")]
    ip: String,
}

fn main() -> color_eyre::Result<()> {
    let args = Args::parse();

    let light = fetch_light_status(&args.ip);

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(args.ip, light).run(terminal);
    ratatui::restore();
    result
}
