// Dependencies
use std::{path::{PathBuf, StripPrefixError}, fs::File};

use clap::Parser;
use docx_rs::{Docx, DocxError, Paragraph, Run, TableRow, TableCell, Table, RunFonts, LineSpacing, Style, StyleType, TableOfContents};

static DEFAULT_EXTENSIONS: [&str; 9] = ["rs", "py", "js", "ts", "html", "css", "scss", "md", "txt"];

/// Convert your code to word document.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The input directory to scan for.
    #[arg(short, long)]
    input: PathBuf,

    /// The output file to write to.
    #[arg(short, long, default_value = "output.docx")]
    output: PathBuf,

    /// Whether to override the output file if it already exists.
    #[arg(long, default_value = "false")]
    overwrite: bool,

    /// Which file extensions to search for and include.
    #[arg(short, long, default_values_t = DEFAULT_EXTENSIONS.iter().map(|s| s.to_string()))]
    extensions: Vec<String>,

    /// The font size of the code.
    #[arg(short, long, default_value = "8")]
    size_font: usize,

    /// The font size of the code headings.
    #[arg(short, long, default_value = "12")]
    heading_size: usize,

    /// The font family of the code headings.
    #[arg(short, long, default_value = "Calibri Light")]
    font_family_heading: String,
}

/// All of the errors.
#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("Input directory does not exist.")]
    InvalidInputDirectory,
    #[error("Input directory is not a directory.")]
    FileInputDirectory,
    #[error("Output file already exists.")]
    InvalidOutputDirectory,

    #[error(transparent)]
    StripPrefix(#[from] StripPrefixError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    #[error(transparent)]
    Docx(#[from] DocxError),
}

/// Processes a file by adding it to the output.
fn process_file(root: &PathBuf, path: &PathBuf, font_size: usize, output: Docx) -> Result<Docx, Error> {
    // Read the file
    let file_path_from_root = path.strip_prefix(root)?;
    let contents = std::fs::read_to_string(path)?;

    // Add the file to the output
    Ok(output
        .add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text(file_path_from_root.to_string_lossy())
                )
                .style("Heading2")
                .page_break_before(true), 
        )
        .add_table(Table::new(vec![
            TableRow::new(vec![
                contents.lines().fold(
                    TableCell::new(),
                    |c, line| {
                        c.add_paragraph(
                            Paragraph::new()
                                .add_run(
                                    Run::new()
                                        .add_text(line.to_string())
                                        .fonts(
                                            RunFonts::new()
                                                .ascii("Courier New")
                                        )
                                        .size(font_size * 2),
                                )
                                .line_spacing(
                                    LineSpacing::new()
                                        .after_lines(0)
                                        .after(0)
                                ),
                        )
                    }
                ),
            ])
        ])))
}

/// Scans the input directory for files.
fn scan_dir(root: &PathBuf, input: &PathBuf, file_exts: &Vec<String>, font_size: usize, mut output: Docx) -> Result<Docx, Error> {
    for entry in input.read_dir()? {
        if let Err(ref e) = entry {
            eprintln!("Error: {}", e);
            continue;
        }

        let entry = entry.unwrap();
        let path = entry.path();

        // Recursive scan
        if path.is_dir() {
            output = scan_dir(root, &path, file_exts, font_size, output.clone())?;
        } else {
            let Some(file_ext) = path.extension() else {
                continue;
            };

            if !file_exts.contains(&file_ext.to_string_lossy().to_lowercase()) {
                continue;
            }

            output = process_file(root, &path, font_size, output.clone())?;
        }
    }

    // Done
    Ok(output)
}


/// The main entry point.
fn main() -> Result<(), Error> {
    // Parse the command line arguments
    let cli = Cli::parse();

    // Check the i/o directories
    if !cli.input.try_exists()? {
        return Err(Error::InvalidInputDirectory);
    }
    if !cli.input.is_dir() {
        return Err(Error::FileInputDirectory);
    }
    if !cli.overwrite && cli.output.try_exists()? {
        return Err(Error::InvalidOutputDirectory);
    }

    // Scan the input directory
    let docx = scan_dir(&cli.input, &cli.input, &cli.extensions, cli.size_font, Docx::new()
        .add_style(
            Style::new("Heading1", StyleType::Paragraph)
                .name("Heading 1")
        )
        .add_style(
            Style::new("Heading2", StyleType::Paragraph)
                .name("Heading 2")
                .size(cli.heading_size * 2)
                .fonts(
                    RunFonts::new()
                        .ascii(&cli.font_family_heading)
                )
        )
        .add_paragraph(
            Paragraph::new()
                .add_run(
                    Run::new()
                        .add_text("Source Code")
                )
                .style("Heading1")
        )
        .add_table_of_contents(
            TableOfContents::new()
                .heading_styles_range(1, 3)
                .alias("Table of contents")
                .auto(),
        )
    )?;
    
    // Write the output file
    let file = File::create(&cli.output)?;
    docx.build().pack(file).map_err(|e| Error::Docx(e.into()))?;

    // Done
    Ok(())
}
