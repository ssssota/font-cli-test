use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "font", about = "font searcher")]
pub enum CliOptions {
    List,
    Search { search_text: String },
}
