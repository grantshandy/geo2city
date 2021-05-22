use reverse_geocoder::{Locations, ReverseGeocoder};

fn main() {
    let args = &std::env::args().collect::<Vec<_>>();

    if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
        println!("geo2city {}", env!("CARGO_PKG_VERSION"));
        println!("By Grant Handy <grantshandy@gmail.com>");
        println!("Command Line Reverse Geocoder");
        println!("\nUSAGE:\n    geo2city [FLAGS] [LATITUDE] [LONGITUDE]");
        println!("\nFLAGS:");
        println!("    -v, --version       Prints version information");
        println!("    -h, --help          Prints help information");
        println!("\nARGS:\n    <LATITUDE>\n    <LONGITUDE>");
        std::process::exit(0);
    };

    if args.contains(&"--version".to_string()) || args.contains(&"-V".to_string()) {
        println!("{}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    };

    if args.len() > 3 || args.len() != 3 {
        eprintln!("error:\n    Incorrect number of arguments.\n\nUSAGE:\n    geo2city <LATITUDE> <LONGITUDE>");
        std::process::exit(1);
    };

    let latitude = &args[1];
    let longitude = &args[2];

    let latitude = match latitude.parse::<f64>() {
        Ok(data) => data,
        Err(_) => {
            eprintln!("error:\n    <LATITUDE> must be a number.\n\nUSAGE:\n    geo2city <LATITUDE> <LONGITUDE>");
            std::process::exit(1);
        },
    };

    let longitude = match longitude.parse::<f64>() {
        Ok(data) => data,
        Err(_) => {
            eprintln!("error:\n    <LONGITUDE> must be a number");
            std::process::exit(1);
        },
    };

    let loc = Locations::from_memory();
    let geocoder = ReverseGeocoder::new(&loc);
    let search_result = match geocoder.search((latitude, longitude)) {
        Some(data) => data,
        None => {
            println!("No data found for this location? This is a bug tbh");
            std::process::exit(1);
        },
    };

    println!("({}, {}) - {}, {}", search_result.record.lat, search_result.record.lon, search_result.record.name, search_result.record.admin3);
}