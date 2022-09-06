use comrak::markdown_to_html;
use std::fs;

#[derive(Debug)]
struct File {
    name: String,
    content: String,
}

fn main() {
    // Constants
    let src_dir = String::from("./content/src");
    let out_dir = String::from("./content/out");

    // The vector of files to be processed
    let mut files: Vec<File> = vec![];
    let mut files_compiled: Vec<File> = vec![];
    let mut index = String::from("<ul>");

    // Delete the output directory
    println!("Deleting old files...");
    fs::remove_dir_all(&out_dir).unwrap_or_else(|_| {
        println!("No old files to delete.");
    });

    // Create the output directory
    fs::create_dir(&out_dir).unwrap();

    // Read the files in the source directory
    println!("Reading files...");
    add_files(&mut files, src_dir);

    println!("{:#?}", files);

    // Compile the files
    println!("Compiling files...");

    for file in files {
        let content = markdown_to_html(&file.content, &comrak::ComrakOptions::default());

        files_compiled.push(File {
            name: file.name,
            content,
        });
    }

    // Write the compiled files to the output directory
    println!("Writing files...");
    files_compiled.iter().for_each(|file| {
        fs::write(
            format!("{}/{}", out_dir, file.name.replace(".md", ".html")),
            &file.content,
        )
        .unwrap();

        println!("Wrote file: {}", file.name);
    });

    // Write the index file
    println!("Writing index...");
    for file in &files_compiled {
        index.push_str(&format!(
            "<li><a href=\"{}\">{}</a></li>",
            file.name.replace(".md", ".html"),
            file.name.replace(".md", "")
        ));
    }
    index.push_str("</ul>");
    fs::write(format!("{}/{}", out_dir, "index.html"), &index).unwrap();

    // Done
    println!("Done!");
}

fn add_files(files: &mut Vec<File>, path: String) {
    // Read the directory
    fs::read_dir(path).unwrap().for_each(|entry| {
        let entry = entry.unwrap();
        let path = entry.path();
        let name = path.file_name().unwrap().to_str().unwrap().to_string();

        // If the entry is a file, add it to the vector
        if path.is_dir() {
            // .. Ignore directories
        } else {
            let content = fs::read_to_string(path).unwrap();
            files.push(File { name, content });
        }
    });
}
