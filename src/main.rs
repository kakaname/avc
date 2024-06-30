mod parse_args;
mod command;


fn main() {
    let output = parse_args::parse_args();
    output.execute();


}
