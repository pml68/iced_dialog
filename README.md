# iced_dialog

Custom dialog for [`iced`](https://iced.rs)

It's mostly the dialog from @frgp42's [Fluent Iced Gallery](https://github.com/frgp42/fluent_iced_gallery), but made into a "widget"

## Example

```rust,no_run
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
```

You can also run the above example:
```bash
cargo run -p example
```
