use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::fs;
use std::io::{self, Write};
use std::sync::mpsc;
use std::time::Duration;

// Update index.html file with the new images
fn update_html() -> io::Result<()> {
    let folder_path = "./images"; // Change this to the path of your image folder

    // Open the output file for writing
    let mut output_file = std::fs::File::create("index.html")?;

    // Write the HTML header
    writeln!(
        output_file,
        "<html><head><title>Image Gallery</title></head><body><div style=\"text-align:center;\">"
    )
    .expect("Failed to write HTML header");

    // Iterate over all files in the image directory
    if let Ok(files) = fs::read_dir(folder_path) {
        for file in files {
            if let Ok(file) = file {
                let path = file.path();
                if let Some(extension) = path.extension() {
                    // Check if the file has an extension
                    if extension.to_string_lossy().to_lowercase() == "jpg"
                        || extension.to_string_lossy().to_lowercase() == "jpeg"
                        || extension.to_string_lossy().to_lowercase() == "png"
                        || extension.to_string_lossy().to_lowercase() == "gif"
                        || extension.to_string_lossy().to_lowercase() == "bmp"
                        || extension.to_string_lossy().to_lowercase() == "webp"
                        || extension.to_string_lossy().to_lowercase() == "svg"
                        || extension.to_string_lossy().to_lowercase() == "tiff"
                        || extension.to_string_lossy().to_lowercase() == "ico"
                        || extension.to_string_lossy().to_lowercase() == "favicon"
                    {
                        // Write the image tag to the HTML file
                        writeln!(
                            output_file,
                            "<img src=\"{}\" alt=\"{}\" height=\"250\">",
                            path.display(),
                            path.file_name().unwrap().to_string_lossy()
                        )?;
                    }
                }
            }
        }
    } else {
        // If the image directory doesn't exist
        println!("Failed to read image directory")
    }

    // Write the HTML footer
    writeln!(output_file, "</div></body></html>")?;

    println!("HTML file generated successfully: index.html");

    Ok(())
}

// Main function to start the watcher and update the HTML file
fn main() {
    // Specify the path to the folder you want to monitor
    let folder_path = "./images";

    // Create a channel for communication between the watcher thread and the main thread
    let (tx, rx) = mpsc::channel();

    // Create a watcher for the folder and set the polling interval to 10 seconds
    let mut watcher = RecommendedWatcher::new(
        tx,
        Config::default().with_poll_interval(Duration::from_secs(10)),
    )
    .expect("Failed to create watcher");

    // Start watching the folder
    watcher
        .watch(folder_path.as_ref(), RecursiveMode::Recursive)
        .expect("Failed to watch folder");

    println!("Watching for changes in: {}", folder_path);

    // Main loop to handle events
    loop {
        match rx.recv() {
            // Wait for an event to be received
            Ok(events) => {
                // If an event is received
                match events {
                    // Match the event type received from the watcher
                    Ok(notify::Event {
                        // If a create event is received, update the HTML file
                        kind: EventKind::Create(notify::event::CreateKind::File),
                        ..
                    }) => {
                        update_html().expect("Error writing to index.html");
                    }
                    Ok(notify::Event {
                        // If a remove event is received, update the HTML file
                        kind: EventKind::Remove(notify::event::RemoveKind::File),
                        ..
                    }) => {
                        update_html().expect("Error writing to index.html");
                    }
                    Ok(notify::Event {
                        // If a modify event is received, update the HTML file
                        kind: EventKind::Modify(notify::event::ModifyKind::Any),
                        ..
                    }) => {
                        update_html().expect("Error writing to index.html");
                    }
                    Err(e) => {
                        // If an error occurs, print the error
                        println!("Error: {:?}", e);
                    }
                    _ => {} // Ignore other events
                }
            }
            Err(e) => {
                // If an error occurs, print the error
                println!("Error: {:?}", e);
            }
        }
    }
}
