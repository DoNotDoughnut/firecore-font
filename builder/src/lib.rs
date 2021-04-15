use std::io::Write;
use std::path::Path;

use firecore_font_lib::FontSheet;
use firecore_font_lib::FontSheetFile;

// pub mod error;


pub fn compile<P: AsRef<Path>>(font_folder: P, output_file: P) {
    let font_folder = font_folder.as_ref();
    let output_file = output_file.as_ref();

    let mut fonts = Vec::new();

    for entry in std::fs::read_dir(font_folder).unwrap() {
        let file = entry.unwrap().path();
        if file.is_file() {
            let content = std::fs::read_to_string(&file).unwrap();
            let font_sheet_file: FontSheetFile = ron::from_str(&content).unwrap();//.map_err(|err| FontError::ParseError(file.to_string_lossy().to_string(), err))?;
            let image = std::fs::read(font_folder.join(&font_sheet_file.file)).unwrap();
            fonts.push(FontSheet {
                image,
                data: font_sheet_file.data,
            });
        }
    }    

    println!("Creating file...");
    let mut file = std::fs::File::create(output_file).unwrap();

    println!("Serializing fonts...");
    let bytes = postcard::to_allocvec(&firecore_font_lib::SerializedFonts {
        fonts
    }).unwrap();

    println!("Writing fonts to file...");
    let bytes = file.write(&bytes).unwrap();
    println!("Wrote {} bytes to font file!", bytes);
}