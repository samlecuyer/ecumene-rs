
extern crate cairo;
extern crate image;

extern crate getopts;
extern crate ecumene;

use std::path::Path;
use std::io::Result;
use getopts::Options;
use std::env;

use ecumene::map::Map;
use ecumene::rendering::CairoRenderer;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("o", "", "set output file name", "a.out.png");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output = matches.opt_str("o").unwrap_or("a.out.png".to_string());
    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    match Map::from_file(Path::new(&input)) {
        Ok(m) => {
            do_work(&m, &output);
        },
        Err(e) => panic!("Couldn't load the map: {:?}", e.to_string())
    };
}

fn do_work(map: &Map, out: &str) {
    let width = 1000.;
    let height = 1000.;

    let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width as i32, height as i32);
    let cr = cairo::Context::new(surface.as_ref());
    cr.set_line_width(0.2);

    let renderer = CairoRenderer::new(&cr, map);
    match renderer.process_layers() {
        Err(e) => {
            println!("{:?}", e);
        },
        Ok(_) => match save_image(&surface, &out) {
            Err(e) => panic!("Could not save image `{}`: {}", out, e),
            _ => {}
        }
    }
}

fn save_image(surface: &cairo::ImageSurface, file: &str) -> Result<()> {
	let len = surface.len();
    let mut buffer : Vec<u8> = Vec::with_capacity(len);
    unsafe {
    	buffer.set_len(len);
    }
    surface.get_data(&mut buffer[..]);
    image::save_buffer(&Path::new(file), 
    	&buffer[..], 
    	surface.get_width() as u32,
    	surface.get_height() as u32,
    	image::RGBA(8))
}