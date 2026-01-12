use lopdf::{Document, Object};
use std::path::PathBuf;

pub fn run(files: &[PathBuf], output: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
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
