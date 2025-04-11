use clap::Parser;

/// Simple dns resolver
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Domain name to resolve
    #[arg(short, long)]
    pub domain: String,

    /// Type of ip address expected. Either v4 or v6
    #[arg(short, long)]
    pub ip_type: String,
}

// Possible variations of ip type user input.
pub enum IpType {
    V4,
    V6,
}

// Basically a wrapper over Args.
pub struct Flags {
    pub domain: String,
    pub ip_type: IpType,
}

impl Flags {
    // We create a new Flags struct and hide
    // the process of parsing args.
    pub fn new() -> Flags {
        let args = Args::parse();

        // For convenience in main.rs
        match args.ip_type.as_str() {
            "v4" => Flags {
                domain: args.domain,
                ip_type: IpType::V4,
            },
            "v6" => Flags {
                domain: args.domain,
                ip_type: IpType::V6,
            },
            // We assume that default is v4.
            _ => Flags {
                domain: args.domain,
                ip_type: IpType::V4,
            },
        }
    }
}
