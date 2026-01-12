use lopdf::{Document, Object};
use std::path::PathBuf;

pub fn run(
    input: &PathBuf,
    output: &PathBuf,
    angle: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut doc = Document::load(input)?;
    let rotation_add = angle % 360;

    println!("Rotating pages by {} degrees...", rotation_add);

    for (_page_num, page_id) in doc.get_pages() {
        if let Ok(page_dict) = doc
            .get_object_mut(page_id)
            .and_then(|obj| obj.as_dict_mut())
        {
            let current_rotation = page_dict
                .get(b"Rotate")
                .and_then(|obj| obj.as_i64())
                .unwrap_or(0);

            let new_rotation = (current_rotation + rotation_add) % 360;
            page_dict.set(b"Rotate", Object::Integer(new_rotation));
        }
    }

    doc.save(output)?;
    println!("Saved rotated PDF to {:?}", output);
    Ok(())
}
