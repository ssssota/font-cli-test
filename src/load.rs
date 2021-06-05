use std::{
    fs::File,
    io::{Error, Read},
    path::Path,
};

use ttf_parser::{fonts_in_collection, name_id, Face};

pub fn load_font<P: AsRef<Path>>(path: P) {
    let data = load_file(path).unwrap();
    let fonts_count = fonts_in_collection(&data[..]).unwrap_or(1);
    for index in 0..fonts_count {
        let res = Face::from_slice(&data[..], index);
        if let Err(e) = res {
            println!("{}: Error {}", index, e);
            continue;
        }
        if let Ok(face) = res {
            println!("{}:", index);
            face.names().for_each(|name| {
                println!("\t{}", parse_name_id(name.name_id()));
                println!(
                    "\t\t{}",
                    name.to_string().unwrap_or("no unicode".to_string())
                );
            });
        }
    }
}

fn load_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    let f = File::open(path);
    let mut buf = Vec::new();
    let res = f.map(|mut file| file.read_to_end(&mut buf));
    res.map(|_| buf)
}

fn parse_name_id(id: u16) -> String {
    match id {
        name_id::COPYRIGHT_NOTICE => String::from("Copyright notice"),
        name_id::FAMILY => String::from("Family name"),
        name_id::SUBFAMILY => String::from("Sub family name"),
        name_id::UNIQUE_ID => String::from("Unique id"),
        name_id::FULL_NAME => String::from("Full name"),
        name_id::VERSION => String::from("Version"),
        name_id::POST_SCRIPT_NAME => String::from("PostScript name"),
        name_id::TRADEMARK => String::from("Trademark"),
        name_id::MANUFACTURER => String::from("Manufacturer"),
        name_id::DESIGNER => String::from("Designer"),
        name_id::DESCRIPTION => String::from("Description"),
        name_id::VENDOR_URL => String::from("Vendor url"),
        name_id::DESIGNER_URL => String::from("Designer url"),
        name_id::LICENSE => String::from("License"),
        name_id::LICENSE_URL => String::from("License url"),
        name_id::TYPOGRAPHIC_FAMILY => String::from("Typographic family name"),
        name_id::TYPOGRAPHIC_SUBFAMILY => String::from("Typographic subfamily name"),
        name_id::COMPATIBLE_FULL => String::from("Compatible fullname"),
        name_id::SAMPLE_TEXT => String::from("Sample text"),
        name_id::POST_SCRIPT_CID => String::from("Postscript CID"),
        name_id::WWS_FAMILY => String::from("WWS Family name"),
        name_id::WWS_SUBFAMILY => String::from("WWS Subfamily name"),
        name_id::LIGHT_BACKGROUND_PALETTE => String::from("Light background palette"),
        name_id::DARK_BACKGROUND_PALETTE => String::from("Dark background palette"),
        name_id::VARIATIONS_POST_SCRIPT_NAME_PREFIX => String::from("Variation PostScript name"),
        _ => String::from("Unexpected id"),
    }
}
