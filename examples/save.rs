#![allow(missing_docs)]
use iced::{
    Element, Task,
    widget::{button, center, column, text},
};
use iced_dialog::dialog;

#[derive(Default)]
struct State {
    is_open: bool,
    action_text: &'static str,
}

#[derive(Debug, Clone)]
enum Message {
    OpenDialog,
    Saved,
    Cancelled,
}

fn main() -> iced::Result {
    iced::run(State::update, State::view)
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenDialog => self.is_open = true,
            Message::Saved => {
                self.action_text = "User saved their work";
                self.is_open = false;
            }
            Message::Cancelled => {
                self.action_text = "User cancelled the dialog";
                self.is_open = false;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let base = center(
            column![
                text(self.action_text),
                button("Open Dialog").on_press(Message::OpenDialog)
            ]
            .spacing(14.0),
        );

        let dialog_content = text("Do you want to save?");

        dialog(self.is_open, base, dialog_content)
            .title("Save")
            .push_button(iced_dialog::button("Save work", Message::Saved))
            .push_button(iced_dialog::button("Cancel", Message::Cancelled))
            .width(350)
            .height(234)
            .on_press(Message::Cancelled)
            .into()
    }
}

#[cfg(test)]
mod tests {
    use iced_test::{Error, simulator};

    use super::*;

    #[test]
    fn dialog_opens() -> Result<(), Error> {
        let mut save = State {
            action_text: "",
            is_open: false,
        };
        let mut ui = simulator(save.view());

        let _ = ui.click("Open Dialog")?;
        for message in ui.into_messages() {
            let _ = save.update(message);
        }

        let mut ui = simulator(save.view());
        assert!(save.is_open);
        assert!(
            ui.find("Do you want to save?").is_ok(),
            "Dialog should be open"
        );

        Ok(())
    }

    #[test]
    fn dialog_closes() -> Result<(), Error> {
        let mut save = State {
            action_text: "",
            is_open: true,
        };
        let mut ui = simulator(save.view());

        let _ = ui.click("Save work")?;
        for message in ui.into_messages() {
            let _ = save.update(message);
        }

        let mut ui = simulator(save.view());
        assert_eq!(save.action_text, "User saved their work");
        assert!(
            ui.find("User saved their work").is_ok(),
            "Dialog should be closed"
        );

        Ok(())
    }
}
