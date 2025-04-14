//! Custom dialog for `iced`
//!
//! # Example
//! See [here](https://github.com/pml68/iced_dialog/tree/0.13/example)
pub mod dialog;
use dialog::Dialog;
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
