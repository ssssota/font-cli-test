use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "font", about = "font searcher")]
pub enum CliOptions {
    List,
    Search {
        search_text: String,
    },
    Details {
        #[structopt(parse(from_os_str))]
        input: PathBuf,
    },
}
