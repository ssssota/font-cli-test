mod input;
mod list;
mod search;

use input::CliOptions;
use list::list_fonts;
use search::search_font;
use structopt::StructOpt;

fn main() {
    let opt = CliOptions::from_args();
    match opt {
        CliOptions::List => list_fonts(),
        CliOptions::Search { search_text } => search_font(search_text),
    };
}
