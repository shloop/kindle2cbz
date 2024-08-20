use clap::Parser;
use mobi::Mobi;
use mobi::headers::ExthRecord;
use std::io::{BufRead, Write};
use std::fs;
use std::process::ExitCode;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to mobi file to convert
    #[arg()]
    in_file: String,

    /// Path to save CBZ to. If omitted, original path and base filename will be used.
    #[arg()]
    out_file: Option<String>,

    /// If set, will overwrite any existing file with same path as output.
    #[arg(short, long, default_value_t = false)]
    force: bool,
}

fn main() -> ExitCode {

    // Parse command line args
    let args = Args::parse();

    // Try to parse input book
    let m = Mobi::from_path(&args.in_file).expect("Failed to parse input file.");

    // Infer output path if one isn't provided
    let out_file = match args.out_file {
        Some(x) => x,
        None => {
            let p = std::path::Path::new(&args.in_file);
            if let Some(ext) = p.extension() {
                // Replace extension with ".cbz"
                p.to_str().unwrap().to_string().replace(
                    ext.to_str().unwrap(), "cbz")
            } else {
                // No extension => append ".cbz" to original filename
                args.in_file.to_string() + ".cbz"
            }
        }
    };

    // If output file already exists, prompt user to overwrite unless force option is used
    if std::path::Path::new(&out_file).exists() && !args.force {
        loop {
            print!("{} already exists. Do you wish to overwrite? y/N:", &out_file);
            std::io::stdout().flush().unwrap();
            let mut line = String::new();
            std::io::stdin().lock().read_line(&mut line).unwrap();
            match line.trim().to_lowercase().as_str() {
                "y" => { break; }
                "n" => { return ExitCode::FAILURE; }
                _ => { continue; }
            }
        }
    }


    // Check if there are images marked as cover or thumbnail
    let mut cover_offset = -1;
    let mut thumb_offset = -1;
    for x in &m.metadata.exth.records {
        match x.0 {
            ExthRecord::CoverOffset => {
                cover_offset = i32::from_be_bytes(<[u8; 4]>::try_from(&x.1[0][0..4]).unwrap());
            }
            ExthRecord::ThumbOffset => {
                thumb_offset =
                    i32::from_be_bytes(<[u8; 4]>::try_from(&x.1[0][0..4]).unwrap()) - 1;
            }
            _ => {}
        }
    }

    // Begin image extraction

    println!("Extracting images from {}...", m.title());

    // Open file for writing
    let file = fs::File::create(out_file).expect("Failed to create output file.");
    let mut zip = ZipWriter::new(file);
    let options =
        SimpleFileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    // Page number, incremented for each image that is not the cover or thumbnail
    let mut i_page = 0;
    // Total offset, incremented with each image
    let mut i_offset = -1;
    for x in m.image_records() {
        i_offset += 1;

        let filename = if i_offset == cover_offset {
            // Cover => cover.ext
            "cover.jpg".to_string()
        } else if i_offset == thumb_offset {
            // Skip thumbnail
            continue
        } else {
            // Page => pXXXX.ext
            i_page += 1;
            std::format!("p{:0>4}.jpg", i_page)
        };

        zip.start_file(filename, options).expect("Failed to add file to CBZ.");
        zip.write_all(&x.content).expect("Failed to add file to CBZ.");
    }

    zip.finish().expect("Failed to finalize CBZ file.");
    println!("Done");
    ExitCode::SUCCESS
}