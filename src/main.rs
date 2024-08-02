use gtk::gio::ApplicationFlags;
use gtk::{glib, Application, ApplicationWindow};
use gtk::{prelude::*, Image, Button, Box as GtkBox, Orientation, Label};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        std::process::exit(1);
    }

    let file = &args[1];
    let content = read_file(file);
    main1(content);
}

fn read_file(file_path: &str) -> Vec<String> {
    let file = File::open(file_path).expect("file not found");
    let reader = BufReader::new(file);
    reader.lines().filter_map(Result::ok).collect()
}

fn main1(content: Vec<String>) -> glib::ExitCode {
    let app = Application::builder()
        .application_id("com.widget")
        .flags(ApplicationFlags::HANDLES_OPEN)
        .build();

    let content_clone = content.clone();
    app.connect_activate(move |app| build_ui(app, &content_clone));

    app.connect_open(move |app, files, _hint| {
        if let Some(file) = files.first() {
            if let Some(path) = file.path() {
                let content = read_file(path.to_str().unwrap());
                build_ui(app, &content);
            }
        }
    });

    app.run()
}

fn build_ui(app: &Application, content: &Vec<String>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_height(500)
        .default_width(500)
        .build();

    let box_container = GtkBox::new(Orientation::Vertical, 6);

    for line in content {
        if line.starts_with("button") {
            let parts: Vec<&str> = line.split(';').collect();
            let label_text = if parts.len() > 1 { parts[1].trim() } else { "Default" };

            let button = if parts.len() > 2 && parts[2].trim().starts_with("image:") {
                let image_path = parts[2].trim()[6..].trim();
                let image = Image::from_file(image_path);
                image.set_pixel_size(64); // 이미지 크기를 조절합니다.

                let label = Label::new(Some(label_text));
                let button_box = GtkBox::new(Orientation::Horizontal, 6);
                button_box.append(&image);
                button_box.append(&label);

                let button = Button::new();
                button.set_child(Some(&button_box));
                button
            } else {
                Button::with_label(label_text)
            };

            button.set_margin_top(12);
            button.set_margin_bottom(12);
            button.set_margin_start(12);
            button.set_margin_end(12);

            if parts.len() > 4 && parts[3].trim() == "onclick" {
                if parts[4].trim().starts_with("setlabel:") {
                    let new_label = parts[4].trim()[9..].trim().to_string();
                    button.connect_clicked(move |button| {
                        if let Some(box_child) = button.child() {
                            if let Ok(button_box) = box_child.downcast::<GtkBox>() {
                                if let Some(label) = button_box.last_child() {
                                    if let Ok(label) = label.downcast::<Label>() {
                                        label.set_text(&new_label);
                                    }
                                }
                            } else {
                                button.set_label(&new_label);
                            }
                        }
                    });
                } else if parts[4].trim().starts_with("runcommand:") {
                    let command = parts[4].trim()[11..].trim().to_string();
                    let cmd: Vec<String> = command.split_whitespace().map(String::from).collect();
                    button.connect_clicked(move |button| {
                        let output = Command::new(&cmd[0])
                            .args(&cmd[1..])
                            .output()
                            .expect("failed to execute command");

                        let output_str = String::from_utf8_lossy(&output.stdout);
                        button.set_label(&output_str);
                    });
                }
            }

            box_container.append(&button);
        } else if line.starts_with("label") {
            let parts: Vec<&str> = line.split(';').collect();
            let label_text = if parts.len() > 1 { parts[1].trim() } else { "Default" };
            
            let label = if parts.len() > 2 && parts[2].trim().starts_with("image:") {
                let image_path = parts[2].trim()[6..].trim();
                let image = Image::from_file(image_path);
                image.set_pixel_size(64); // 이미지 크기를 조절합니다.
                
                let label = Label::new(Some(label_text));
                let label_box = GtkBox::new(Orientation::Horizontal, 6);
                label_box.append(&image);
                label_box.append(&label);
                
                label_box
            }else {
                let label = Label::new(Some(label_text));
                let label_box = GtkBox::new(Orientation::Horizontal, 6);
                label_box.append(&label);
                label_box
            };

            box_container.append(&label);
        }
    }

    window.set_child(Some(&box_container));
    window.present();
}
