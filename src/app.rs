use std::rc::Rc;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, AboutDialog, Button, Grid, Entry, HeaderBar, MenuButton, gio};
use crate::config::Config;

// Define CalculateApp struct
pub struct CalculateApp {
    application: Application,
    config: Config,
}

// Implement CalculateApp methods
impl CalculateApp {
    pub fn new(app_id: &str, config_file: &str) -> Self {
        let application = Application::builder()
            .application_id(app_id)
            .build();
        let config = Config::from_file(config_file);

        Self {
            application,
            config,
        }
    }

    pub fn run(self){
        let (width, height) = self.config.get_window_size();

        self.application.connect_activate( move |app| {
            let window = ApplicationWindow::builder()
                .application(app)
                .title("Rust GTK Calculator")
                .width_request(width)
                .height_request(height)
                .build();

            let header_bar = HeaderBar::builder()
                .title("Rust GTK Calculator")
                .show_close_button(true)
                .build();

            let menu_button = Self::create_menu_button(app, Rc::new(window.clone()));
            header_bar.pack_end(&menu_button);

            let grid = Grid::builder()
                .row_spacing(5)
                .column_spacing(5)
                .margin_top(10)
                .margin_start(10)
                .margin_end(10)
                .margin_bottom(10)
                .expand(true)
                .build();

            let entry = Entry::builder()
                .editable(false)
                .hexpand(true)
                .vexpand(true)
                .build();
            grid.attach(&entry, 0, 0, 5, 1);

            let buttons = [
                ("7", 1, 0, 1, 1), ("8", 1, 1, 1, 1), ("9", 1, 2, 1, 1), ("+", 1, 3, 1, 1), ("-", 1, 4, 1, 1),
                ("4", 2, 0, 1, 1), ("5", 2, 1, 1, 1), ("6", 2, 2, 1, 1), ("*", 2, 3, 1, 1), ("/", 2, 4, 1, 1),
                ("1", 3, 0, 1, 1), ("2", 3, 1, 1, 1), ("3", 3, 2, 1, 1), ("Undo", 3, 3, 1, 1), ("Clear", 3, 4, 1, 1),
                ("0", 4, 0, 1, 1), (".", 4, 1, 1, 1), ("=", 4, 3, 2, 1),
            ];

            for (label, row, col, width, height) in buttons {
                let button = Button::builder()
                    .label(label)
                    .hexpand(true)
                    .vexpand(true)
                    .build();
                let entry_clone = entry.clone();
                button.connect_clicked(move |_| {
                    let mut text = entry_clone.text().to_string();
                    match label {
                        "=" => {
                            if let Ok(result) = meval::eval_str(&text) {
                                text = result.to_string();
                            } else {
                                text = "Error".to_string();
                            }
                        }
                        "C" => text.clear(),
                        _ => text.push_str(label),
                    }
                    entry_clone.set_text(&text);
                });
                grid.attach(&button, col, row, width, height);
            }

            window.set_titlebar(Some(&header_bar));
            window.add(&grid);
            window.show_all();
        });

        self.application.run();
    }

    fn create_menu_button(application: &Application, parent_window: Rc<ApplicationWindow>) -> MenuButton {
        let menu_button = MenuButton::builder()
            .direction(gtk::ArrowType::Down)
            .build();

        let menu_model = gio::Menu::new();
        menu_model.append(Some("Preferences"), Some("app.preferences"));
        menu_model.append(Some("About"), Some("app.about"));
        menu_button.set_menu_model(Some(&menu_model));

        let preference_action = gio::SimpleAction::new("preferences", None);
        preference_action.connect_activate(|_, _| {
            eprintln!("Preferences clicked");
        });
        application.add_action(&preference_action);
        
        let about_action = gio::SimpleAction::new("about", None);
        about_action.connect_activate(move |_, _| {
            let weak_window = Rc::downgrade(&parent_window);
            if let Some(parent) = weak_window.upgrade() {
                let dialog = AboutDialog::builder()
                .transient_for(&*parent)
                .modal(true)
                .program_name("Rust GTK Calculator")
                .version("1.0")
                .copyright("© 2025 Haiphamcoder")
                .comments("A simple calculator application")
                .license_type(gtk::License::MitX11)
                .website("https://github.com/haiphamcoder/rust-gtk-calculator")
                .website_label("Visit our website")
                .build();

                dialog.show();
            }

            
        });
        application.add_action(&about_action);

        menu_button
    }
}