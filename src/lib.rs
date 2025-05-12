//! Custom dialog for `iced`
//!
//! # Example
//! ```no_run
//! use iced::{
//!     Element, Length, Task,
//!     widget::{button, center, column, text},
//! };
//! use iced_dialog::dialog;
//!
//! #[derive(Default)]
//! struct State {
//!     is_open: bool,
//!     action_text: String,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     OpenDialog,
//!     Saved,
//!     Cancelled,
//! }
//!
//! fn main() -> iced::Result {
//!     iced::run("Dialog Example", State::update, State::view)
//! }
//!
//! impl State {
//!     fn update(&mut self, message: Message) -> Task<Message> {
//!         match message {
//!             Message::OpenDialog => self.is_open = true,
//!             Message::Saved => {
//!                 self.action_text = "User saved their work".to_owned();
//!                 self.is_open = false;
//!             }
//!             Message::Cancelled => {
//!                 self.action_text = "User cancelled the dialog".to_owned();
//!                 self.is_open = false;
//!             }
//!         }
//!         Task::none()
//!     }
//!
//!     fn view(&self) -> Element<'_, Message> {
//!         let base = center(
//!             column![
//!                 text(&self.action_text),
//!                 button("Open Dialog").on_press(Message::OpenDialog)
//!             ]
//!             .spacing(14.0),
//!         )
//!         .width(Length::Fill)
//!         .height(Length::Fill);
//!
//!         let dialog_content = text("Do you want to save?");
//!
//!         dialog(self.is_open, base, dialog_content)
//!             .title("Save")
//!             .push_button(iced_dialog::button("Save", Message::Saved))
//!             .push_button(iced_dialog::button("Cancel", Message::Cancelled))
//!             .width(350)
//!             .height(234)
//!             .into()
//!     }
//! }
//! ```
pub mod dialog;
pub use dialog::Dialog;
use iced_core as core;
use iced_core::alignment::Horizontal;
use iced_widget::Button;
use iced_widget::{container, text};

/// Creates a new [`Dialog`] with the given base and dialog content.
pub fn dialog<'a, Message, Theme, Renderer>(
    is_open: bool,
    base: impl Into<core::Element<'a, Message, Theme, Renderer>>,
    content: impl Into<core::Element<'a, Message, Theme, Renderer>>,
) -> Dialog<'a, Message, Theme, Renderer>
where
    Renderer: 'a + core::Renderer + core::text::Renderer,
    Theme: 'a + dialog::Catalog,
    Message: 'a + Clone,
    <Theme as container::Catalog>::Class<'a>:
        From<container::StyleFn<'a, Theme>>,
{
    Dialog::new(is_open, base, content)
}

/// Pre-styled [`Button`] for [`Dialog`]s.
///
/// [`Button`]: https://docs.rs/iced/0.13.1/iced/widget/struct.Button.html
pub fn button<'a, Message, Theme, Renderer>(
    content: &'a str,
    message: Message,
) -> Button<'a, Message, Theme, Renderer>
where
    Theme: 'a + iced_widget::button::Catalog + text::Catalog,
    Renderer: 'a + core::Renderer + core::text::Renderer,
{
    iced_widget::button(
        text(content)
            .size(14)
            .line_height(text::LineHeight::Absolute(core::Pixels(20.0)))
            .align_x(Horizontal::Center),
    )
    .on_press(message)
    .height(32)
    .width(core::Length::Fill)
}
