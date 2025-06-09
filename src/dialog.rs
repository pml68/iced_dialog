//! Dialogs can be used to provide users with
//! important information and make them act on it.
use iced_core::{
    self as core, Alignment, Color, Element, Length, Padding, Pixels,
    alignment::Vertical, color,
};
use iced_widget::{
    Column, Container, Row, Theme, center, container, mouse_area, opaque,
    stack, text,
    text::{Fragment, IntoFragment},
    vertical_space,
};

/// A message dialog.
///
/// Only the content is required, [`buttons`] and the [`title`] are optional.
///
/// [`buttons`]: Dialog::with_buttons
/// [`title`]: Dialog::title
pub struct Dialog<
    'a,
    Message,
    Theme = iced_widget::Theme,
    Renderer = iced_widget::Renderer,
> where
    Renderer: 'a + core::text::Renderer,
    Theme: 'a + Catalog,
{
    is_open: bool,
    base: Element<'a, Message, Theme, Renderer>,
    title: Option<Fragment<'a>>,
    content: Element<'a, Message, Theme, Renderer>,
    buttons: Vec<Element<'a, Message, Theme, Renderer>>,
    font: Option<Renderer::Font>,
    width: Length,
    height: Length,
    spacing: f32,
    padding: Padding,
    button_alignment: Alignment,
    class: <Theme as Catalog>::Class<'a>,
    title_class: <Theme as text::Catalog>::Class<'a>,
    container_class: <Theme as container::Catalog>::Class<'a>,
}

impl<'a, Message, Theme, Renderer> Dialog<'a, Message, Theme, Renderer>
where
    Renderer: 'a + core::Renderer + core::text::Renderer,
    Theme: 'a + Catalog,
    Message: 'a + Clone,
{
    /// Creates a new [`Dialog`] with the given base and dialog content.
    pub fn new(
        is_open: bool,
        base: impl Into<Element<'a, Message, Theme, Renderer>>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self::with_buttons(is_open, base, content, Vec::new())
    }

    /// Creates a new [`Dialog`] with the given base, dialog content and buttons.
    pub fn with_buttons(
        is_open: bool,
        base: impl Into<Element<'a, Message, Theme, Renderer>>,
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
        buttons: Vec<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            is_open,
            base: base.into(),
            title: None,
            content: content.into(),
            buttons,
            font: None,
            width: 400.into(),
            height: 260.into(),
            spacing: 8.0,
            padding: 24.into(),
            button_alignment: Alignment::Start,
            class: <Theme as Catalog>::default(),
            title_class: <Theme as Catalog>::default_title(),
            container_class: <Theme as Catalog>::default_container(),
        }
    }

    /// Sets the [`Dialog`]'s title.
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(title.into_fragment());
        self
    }

    /// Sets the [`Dialog`]'s width.
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the [`Dialog`]'s height.
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the [`Dialog`]'s padding.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the [`Dialog`]'s spacing.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the vertical alignment of the [`Dialog`]'s buttons.
    pub fn align_buttons(mut self, align: impl Into<Vertical>) -> Self {
        self.button_alignment = Alignment::from(align.into());
        self
    }

    /// Sets the [`Font`] of the [`Dialog`]'s title.
    ///
    /// [`Font`]: https://docs.iced.rs/iced_core/text/trait.Renderer.html#associatedtype.Font
    pub fn font(mut self, font: impl Into<Renderer::Font>) -> Self {
        self.font = Some(font.into());
        self
    }

    /// Adds a button to the [`Dialog`].
    pub fn push_button(
        mut self,
        button: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        self.buttons.push(button.into());
        self
    }

    /// Adds a button to the [`Dialog`], if `Some`.
    pub fn push_button_maybe(
        self,
        button: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
    ) -> Self {
        if let Some(button) = button {
            self.push_button(button)
        } else {
            self
        }
    }

    /// Extends the [`Dialog`] with the given buttons.
    pub fn extend_buttons(
        self,
        buttons: impl IntoIterator<Item = Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        buttons.into_iter().fold(self, Self::push_button)
    }

    /// Sets the backdrop color of the [`Dialog`].
    pub fn backdrop(self, color: impl Into<Color>) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        let backdrop_color = color.into();

        self.style(move |_theme| Style { backdrop_color })
    }

    /// Sets the style of the [`Dialog`].
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self
    where
        <Theme as Catalog>::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style of the [`Dialog`]'s title.
    #[must_use]
    pub fn title_style(
        mut self,
        style: impl Fn(&Theme) -> text::Style + 'a,
    ) -> Self
    where
        <Theme as text::Catalog>::Class<'a>: From<text::StyleFn<'a, Theme>>,
    {
        self.title_class = (Box::new(style) as text::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style of the [`Dialog`]'s container.
    #[must_use]
    pub fn container_style(
        mut self,
        style: impl Fn(&Theme) -> container::Style + 'a,
    ) -> Self
    where
        <Theme as container::Catalog>::Class<'a>:
            From<container::StyleFn<'a, Theme>>,
    {
        self.container_class =
            (Box::new(style) as container::StyleFn<'a, Theme>).into();
        self
    }

    /// Sets the style class of the [`Dialog`].
    #[must_use]
    pub fn class(
        mut self,
        class: impl Into<<Theme as Catalog>::Class<'a>>,
    ) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the style class of the [`Dialog`]'s title.
    #[must_use]
    pub fn title_class(
        mut self,
        class: impl Into<<Theme as text::Catalog>::Class<'a>>,
    ) -> Self {
        self.title_class = class.into();
        self
    }

    /// Sets the style class of the [`Dialog`]'s container.
    #[must_use]
    pub fn container_class(
        mut self,
        class: impl Into<<Theme as container::Catalog>::Class<'a>>,
    ) -> Self {
        self.container_class = class.into();
        self
    }

    fn view(self) -> Element<'a, Message, Theme, Renderer>
    where
        <Theme as container::Catalog>::Class<'a>:
            From<container::StyleFn<'a, Theme>>,
    {
        let dialog = self.is_open.then(|| {
            let contents = Container::new(
                Column::new()
                    .push_maybe(self.title.map(|title| {
                        let text = text(title)
                            .size(20)
                            .line_height(text::LineHeight::Absolute(Pixels(
                                26.0,
                            )))
                            .class(self.title_class);

                        if let Some(font) = self.font {
                            text.font(font)
                        } else {
                            text
                        }
                    }))
                    .push(vertical_space().height(12))
                    .push(self.content),
            )
            .width(Length::Fill)
            .padding(self.padding);

            let buttons = Container::new(
                Row::with_children(self.buttons).spacing(self.spacing),
            )
            .height(80)
            .padding(self.padding);

            Container::new(
                Column::new()
                    .push(contents)
                    .push(vertical_space())
                    .push(buttons),
            )
            .width(self.width)
            .height(self.height)
            .class(self.container_class)
            .clip(true)
        });

        modal(self.base, dialog, self.class)
    }
}

impl<'a, Message, Theme, Renderer> From<Dialog<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + core::Renderer + core::text::Renderer,
    Theme: 'a + Catalog,
    Message: 'a + Clone,
    <Theme as container::Catalog>::Class<'a>:
        From<container::StyleFn<'a, Theme>>,
{
    fn from(dialog: Dialog<'a, Message, Theme, Renderer>) -> Self {
        dialog.view()
    }
}

fn modal<'a, Message, Theme, Renderer>(
    base: impl Into<Element<'a, Message, Theme, Renderer>>,
    content: Option<impl Into<Element<'a, Message, Theme, Renderer>>>,
    class: <Theme as Catalog>::Class<'a>,
) -> Element<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + core::Renderer,
    Theme: 'a + container::Catalog + Catalog,
    <Theme as container::Catalog>::Class<'a>:
        From<container::StyleFn<'a, Theme>>,
{
    let area = content.map(|content| {
        opaque(mouse_area(center(opaque(content)).style(move |theme| {
            container::Style {
                background: Some(
                    Catalog::style(theme, &class).backdrop_color.into(),
                ),
                ..Default::default()
            }
        })))
    });

    stack![base.into()].push_maybe(area).into()
}

/// The style of a [`Dialog`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// The [`Dialog`]'s backdrop.
    pub backdrop_color: Color,
}

/// The theme catalog of a [`Dialog`].
pub trait Catalog: text::Catalog + container::Catalog {
    /// The item class of the [`Catalog`].
    type Class<'a>;

    /// The default class produced by the [`Catalog`].
    fn default<'a>() -> <Self as Catalog>::Class<'a>;

    /// The default class for the [`Dialog`]'s title.
    fn default_title<'a>() -> <Self as text::Catalog>::Class<'a> {
        <Self as text::Catalog>::default()
    }

    /// The default class for the [`Dialog`]'s container.
    fn default_container<'a>() -> <Self as container::Catalog>::Class<'a> {
        <Self as container::Catalog>::default()
    }

    /// The [`Style`] of a class.
    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style;
}

/// A styling function for a [`Dialog`].
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Box::new(default)
    }

    fn default_container<'a>() -> <Self as container::Catalog>::Class<'a> {
        Box::new(|theme| {
            container::background(
                theme.extended_palette().background.base.color,
            )
        })
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        class(self)
    }
}

/// The default style of a [`Dialog`].
pub fn default(_theme: &Theme) -> Style {
    Style {
        backdrop_color: color!(0x000000, 0.3),
    }
}
