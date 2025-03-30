use iced::{
    Element, Length, Task,
    widget::{center, column, text},
};
use iced_dialog::{button, dialog};

#[derive(Default)]
struct State {
    is_open: bool,
    action_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    OpenDialog,
    Saved,
    Cancelled,
}

fn main() -> iced::Result {
    iced::run("Dialog Example", State::update, State::view)
}

impl State {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenDialog => self.is_open = true,
            Message::Saved => {
                self.action_text = "User saved their work".to_owned();
                self.is_open = false;
            }
            Message::Cancelled => {
                self.action_text = "User cancelled the dialog".to_owned();
                self.is_open = false;
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let base = center(
            column![
                text(&self.action_text),
                iced::widget::button("Open Dialog")
                    .on_press(Message::OpenDialog)
            ]
            .spacing(14.0),
        )
        .width(Length::Fill)
        .height(Length::Fill);

        let dialog_content = text("Do you want to save?");

        dialog(self.is_open, base, dialog_content)
            .title("Save")
            .push_button(button("Save").on_press(Message::Saved))
            .push_button(button("Cancel").on_press(Message::Cancelled))
            .width(350)
            .height(234)
            .into()
    }
}
