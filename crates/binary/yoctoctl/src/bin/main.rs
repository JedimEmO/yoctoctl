use clap::Clap;

use yoctoctl::layers::application::cmdline_params::YoctoctlCmdline;

fn main() {
    let cmdline_params = YoctoctlCmdline::parse();

    println!("params: {:?}", cmdline_params);
}
