# ParseNmap
Parsenmap
This is a tool for parsing nmap xml file to csv or json.

USAGE:
      $ parsenmap <from> <to> -f <filetype>

FLAGS:
      -h, --help - Prints help information
      -V, --version - Prints version information

OPTIONS:
      -f {filetype} - Output file format csv or json

ARGS:
      {from} - Nmap xml file path
      {to} - Output file path

EXAMPLE:
      $ parsenmap nmap.xml nmap.csv -f csv

IN-CODE USAGE EXAMPLE:

use parsenmap::models::scan::FileType;

fn main() {
    let scan:Scan = parsenmap::parse("nmap.xml").unwrap();
    scan.write_to_file(FileType::CSV, "nmap.csv".unwrap());
    let json:String =  scan.to_json();
    let csv:String =  scan.to_csv();
}
