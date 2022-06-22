use std::borrow::Borrow;
use std::ffi::{OsStr, OsString};
use std::path::PathBuf;
use clap::{App, arg, Parser};
use dirs;
use url::{Url};
use reqwest;
use std::fs::File;
use std::io::Write;


#[derive(Debug, Parser)]
#[clap(author = "Int Algambra", about = "Simple cli programm to download and unpack stuff from thingiverse")]
struct CliArguments {
    thing_id: String,
    #[clap(parse(from_os_str))]
    path_to_save: Option<PathBuf>,
}


#[derive(Debug)]
struct AppData {
    url: Url,
    path: PathBuf,
}

impl AppData {
    fn new(args: CliArguments) -> Self {
        let url = Url::parse(
            &format!("https://www.thingiverse.com/thing:{}/zip", args.thing_id)
        ).unwrap();
        let mut path = match args.path_to_save {
            Some(p) => p,
            None => dirs::download_dir().unwrap()
        };
        if !(path.exists() & path.is_dir()) {
            panic!("NO SUCH FOLDER")
        }
        path = path.join(PathBuf::from(format!("{}.zip", args.thing_id)));
        AppData {
            url,
            path,
        }
    }
}

fn download(data: AppData) {
    let mut res =reqwest::blocking::get(data.url).unwrap();
    println!("{}", res.status());
    let mut file = File::create(data.path).unwrap();
    file.write_all(res.bytes().unwrap().borrow());
}

fn main() {
    let cli_args = CliArguments::parse();
    let data = AppData::new(cli_args);
    println!("{:?}", data);
    download(data);
}
