use std::process::Command;
use systray;

pub fn run_in_tray() -> Result<(), systray::Error> {
    let mut app;
    match systray::Application::new() {
        Ok(w) => app = w,
        Err(_) => panic!("Can't create window!"),
    }
    app.set_icon_from_file("./build/icon.png")?;

    app.add_menu_item("😉", |_| {
        let window_list = Command::new("wmctrl")
            .arg("-l")
            .output()
            .expect("could not find windows")
            .stdout;

        let window_list_string = String::from_utf8(window_list).unwrap();
        println!("{}", window_list_string);

        match window_list_string.find("Emoji Picker") {
            Some(m) => {
                Command::new("wmctrl")
                    .args(vec!["-a", "Emoji Picker"])
                    .output()
                    .expect("Could not focus emoji picker");
            }
            None => {
                Command::new("emoji-picker")
                    .spawn()
                    .expect("Failed to launch emoji-picker");
            }
        }

        Ok::<_, systray::Error>(())
    })?;

    app.add_menu_item("Quit", |window| {
        window.quit();
        Ok::<_, systray::Error>(())
    })?;

    app.wait_for_message()?;
    Ok(())
}
