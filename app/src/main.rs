use std::path::PathBuf;

use app::config::Config;
use app::App;
use clap::Parser;

#[derive(Debug, clap::Parser)]
#[command(version, about = "Main app backend which runs the core logic.", long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "Path of dir where the config files are, default path is ."
    )]
    config_dir: PathBuf,
    #[arg(short, long)]
    hpe: Option<String>,
    #[arg(short, long)]
    head_detection: Option<String>,
    #[arg(short, long)]
    gesture_detection: Option<String>,
    #[arg(short, long)]
    picam: Option<String>,
    #[arg(short, long)]
    pool_size: Option<usize>,
}

fn main() {
    let args = Args::parse();
    let config = {
        let mut c = Config::open(args.config_dir).unwrap();
        if let Some(v) = args.hpe {
            c.hpe_addr = v;
        }
        if let Some(v) = args.head_detection {
            c.head_detection_addr = v;
        }
        if let Some(v) = args.gesture_detection {
            c.gesture_detection_addr = v;
        }
        if let Some(v) = args.picam {
            c.picam_addr = v;
        }
        if let Some(v) = args.pool_size {
            c.pool_size = v;
        }

        c
    };

    let app = App::new(config).unwrap();

    app.run().unwrap();
}
