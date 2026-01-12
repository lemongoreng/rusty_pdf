use lopdf::{Document, Object};
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

pub fn run(input: &PathBuf, dir: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let doc = Document::load(input)?;

    if !dir.exists() {
        fs::create_dir_all(dir)?;
        println!("Created directory: {:?}", dir);
    }

    println!("Scanning {:?} for images...", input);
    let mut image_count = 0;

    for (page_num, page_id) in doc.get_pages() {
        let resources = match doc.get_page_resources(page_id) {
            Ok((Some(res), _)) => res,
            _ => continue,
        };

        if let Ok(xobjects) = resources.get(b"XObject").and_then(|o| o.as_dict()) {
            for (name, object) in xobjects {
                let obj_id = match object.as_reference() {
                    Ok(id) => id,
                    Err(_) => continue,
                };

                if let Ok(Object::Stream(stream)) = doc.get_object(obj_id) {
                    let is_image = stream
                        .dict
                        .get(b"Subtype")
                        .and_then(|o| o.as_name())
                        .map(|bytes| bytes == b"Image")
                        .unwrap_or(false);

                    if !is_image {
                        continue;
                    }

                    let filter = stream
                        .dict
                        .get(b"Filter")
                        .and_then(|o| o.as_name())
                        .unwrap_or(b"");

                    let extension = match filter {
                        b"DCTDecode" => "jpg",
                        b"JPXDecode" => "jp2",
                        b"FlateDecode" => {
                            println!(
                                "Page {}: Found a FlateDecode image (PNG-like). skipping for now...",
                                page_num
                            );
                            continue;
                        }
                        other => {
                            println!(
                                "Page {}: Found unsupported format: {:?}",
                                page_num,
                                String::from_utf8_lossy(other)
                            );
                            continue;
                        }
                    };

                    let safe_name = String::from_utf8_lossy(name);
                    let file_name = format!("p{}_{}.{}", page_num, safe_name, extension);
                    let file_path = dir.join(file_name);

                    let mut file = File::create(&file_path)?;
                    file.write_all(&stream.content)?;

                    println!("Extracted: {:?}", file_path);
                    image_count += 1;
                }
            }
        }
    }

    println!("Done! Extracted {} images.", image_count);
    Ok(())
}
