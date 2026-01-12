use clap::{Parser, Subcommand};
use lopdf::Document;
use lopdf::Object;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "Rusty PDF")]
#[command(version = "1.0")]
#[command(about = "A blazingly fast PDF tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Merge {
        #[arg(short, long)]
        output: PathBuf,

        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Merge { output, files } => {
            println!("Starting merge operation for {} files...", files.len());

            match merge_pdfs(files, output) {
                Ok(_) => println!("Success! Saved to {:?}", output),
                Err(e) => println!("Error during merge: {}", e),
            }
        }
    }
}

fn merge_pdfs(files: &[PathBuf], output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let mut master_doc = Document::load(&files[0])?;
    let mut max_id = master_doc.max_id;
    let catalog = master_doc.catalog()?;
    let pages_id = catalog.get(b"Pages")?.as_reference()?;

    for file_path in files.iter().skip(1) {
        let mut doc = Document::load(file_path)?;

        doc.renumber_objects_with(max_id + 1);
        max_id = doc.max_id;

        let page_ids: Vec<lopdf::ObjectId> = doc.get_pages().values().cloned().collect();

        for (id, object) in doc.objects {
            master_doc.objects.insert(id, object);
        }

        if let Ok(pages_object) = master_doc.get_object_mut(pages_id) {
            if let Object::Dictionary(dict) = pages_object {
                let kids = dict.get_mut(b"Kids")?.as_array_mut()?;
                for pid in &page_ids {
                    kids.push(Object::Reference(*pid));
                }

                let count = dict.get_mut(b"Count")?;
                if let Object::Integer(c) = count {
                    *c += page_ids.len() as i64;
                }
            }
        }
    }

    master_doc.compress();
    master_doc.save(output)?;
    Ok(())
}
