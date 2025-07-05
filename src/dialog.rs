//! Dialogs can be used to provide users with
//! important information and make them act on it.
use iced_core::{
    self as core, Color, Element, Length, Padding, Pixels, alignment, color,
};
use iced_widget::{
    Column, Container, Row, Theme, container, mouse_area, opaque, stack, text,
    text::{Fragment, IntoFragment},
    vertical_space,
};

/// A message dialog.
///
/// Only the content is required, [`buttons`] and the [`title`] are optional.
///
/// The sizing strategy used depends on whether you add any buttons: if you don't, the dialog will
/// be sized to fit the content similarly to a container. If you *do* add buttons, [`max_width`]
/// & [`max_height`] (or [`width`] & [`height`] when set to a fixed pixel value) are used. If
/// these aren't set, [`DEFAULT_MAX_WIDTH`] and/or [`DEFAULT_MAX_HEIGHT`] are used.
///
/// [`buttons`]: Dialog::with_buttons
/// [`title`]: Dialog::title
/// [`max_width`]: Dialog::max_width
/// [`max_height`]: Dialog::max_height
/// [`width`]: Dialog::width
/// [`height`]: Dialog::height
/// [`DEFAULT_MAX_WIDTH`]: Dialog::DEFAULT_MAX_WIDTH
/// [`DEFAULT_MAX_HEIGHT`]: Dialog::DEFAULT_MAX_HEIGHT
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
    on_press: Option<Box<dyn Fn() -> Message + 'a>>,
    font: Option<Renderer::Font>,
    width: Length,
    height: Length,
    max_width: Option<f32>,
    max_height: Option<f32>,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    spacing: f32,
    padding_inner: Padding,
    padding_outer: Padding,
    button_alignment: alignment::Vertical,
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
    /// The default maximum width of a [`Dialog`].
    ///
    /// Check the main documentation of [`Dialog`] to see when this is used.
    pub const DEFAULT_MAX_WIDTH: f32 = 400.0;

    /// The default maximum height of a [`Dialog`].
    ///
    /// Check the main documentation of [`Dialog`] to see when this is used.
    pub const DEFAULT_MAX_HEIGHT: f32 = 260.0;

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
        let content = content.into();
        let size = content.as_widget().size_hint();

        Self {
            is_open,
            base: base.into(),
            title: None,
            content,
            buttons,
            on_press: None,
            font: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            max_width: None,
            max_height: None,
            horizontal_alignment: alignment::Horizontal::Center,
            vertical_alignment: alignment::Vertical::Center,
            spacing: 8.0,
            padding_inner: 24.into(),
            padding_outer: Padding::ZERO,
            button_alignment: alignment::Vertical::Top,
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

    /// Sets the message that will be produced when the [`Dialog`]'s backdrop is pressed.
    pub fn on_press(mut self, on_press: Message) -> Self
    where
        Message: Clone,
    {
        self.on_press = Some(Box::new(move || on_press.clone()));
        self
    }

    /// Sets the message that will be produced when the [`Dialog`]'s backdrop is pressed.
    ///
    /// This is analogous to [`Dialog::on_press`], but using a closure to produce
    /// the message.
    pub fn on_press_with(
        mut self,
        on_press: impl Fn() -> Message + 'a,
    ) -> Self {
        self.on_press = Some(Box::new(on_press));
        self
    }

    /// Sets the message that will be produced when the [`Dialog`]'s backdrop is pressed, if `Some`.
    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self
    where
        Message: Clone,
    {
        self.on_press = if let Some(on_press) = on_press {
            Some(Box::new(move || on_press.clone()))
        } else {
            None
        };

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

    /// Sets the [`Dialog`]'s maximum width.
    pub fn max_width(mut self, max_width: impl Into<Pixels>) -> Self {
        self.max_width = Some(max_width.into().0);
        self
    }

    /// Sets the [`Dialog`]'s maximum height.
    pub fn max_height(mut self, max_height: impl Into<Pixels>) -> Self {
        self.max_height = Some(max_height.into().0);
        self
    }

    /// Aligns the [`Dialog`] to the left.
    pub fn align_left(self) -> Self {
        self.align_x(alignment::Horizontal::Left)
    }

    /// Aligns the [`Dialog`] to the right.
    pub fn align_right(self) -> Self {
        self.align_x(alignment::Horizontal::Right)
    }

    /// Aligns the [`Dialog`] to the top.
    pub fn align_top(self) -> Self {
        self.align_y(alignment::Vertical::Top)
    }

    /// Aligns the [`Dialog`] to the bottom.
    pub fn align_bottom(self) -> Self {
        self.align_y(alignment::Vertical::Bottom)
    }

    /// Sets the [`Dialog`]'s alignment for the horizontal axis.
    ///
    /// [`Dialog`]s are horizontally centered by default.
    pub fn align_x(
        mut self,
        alignment: impl Into<alignment::Horizontal>,
    ) -> Self {
        self.horizontal_alignment = alignment.into();
        self
    }

    /// Sets the [`Dialog`]'s alignment for the vertical axis.
    ///
    /// [`Dialog`]s are vertically centered by default.
    pub fn align_y(
        mut self,
        alignment: impl Into<alignment::Vertical>,
    ) -> Self {
        self.vertical_alignment = alignment.into();
        self
    }

    /// Sets the [`Dialog`]'s inner padding.
    pub fn padding_inner(mut self, padding: impl Into<Padding>) -> Self {
        self.padding_inner = padding.into();
        self
    }

    /// Sets the [`Dialog`]'s outer padding (sometimes called "margin").
    pub fn padding_outer(mut self, padding: impl Into<Padding>) -> Self {
        self.padding_outer = padding.into();
        self
    }

    /// Sets the [`Dialog`]'s spacing.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into().0;
        self
    }

    /// Sets the vertical alignment of the [`Dialog`]'s buttons.
    pub fn align_buttons(
        mut self,
        align: impl Into<alignment::Vertical>,
    ) -> Self {
        self.button_alignment = align.into();
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
            let has_title = self.title.is_some();
            let has_buttons = !self.buttons.is_empty();

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
                    .push_maybe(
                        has_title.then_some(vertical_space().height(12)),
                    )
                    .push(self.content),
            )
            .padding(self.padding_inner);

            let contents = if has_buttons {
                contents.width(Length::Fill)
            } else {
                contents
            };

            let buttons = has_buttons.then_some(
                Container::new(
                    Row::with_children(self.buttons)
                        .spacing(self.spacing)
                        .align_y(self.button_alignment),
                )
                .height(80)
                .padding(self.padding_inner),
            );

            let max_width = self.max_width.unwrap_or(
                if has_buttons && !matches!(self.width, Length::Fixed(_)) {
                    Self::DEFAULT_MAX_WIDTH
                } else {
                    f32::INFINITY
                },
            );

            let max_height = self.max_height.unwrap_or(
                if has_buttons && !matches!(self.height, Length::Fixed(_)) {
                    Self::DEFAULT_MAX_HEIGHT
                } else {
                    f32::INFINITY
                },
            );

            Container::new(
                Column::new()
                    .push(contents)
                    .push_maybe(has_buttons.then_some(vertical_space()))
                    .push_maybe(buttons),
            )
            .width(self.width)
            .height(self.height)
            .max_width(max_width)
            .max_height(max_height)
            .class(self.container_class)
            .clip(true)
        });

        modal(
            self.base,
            dialog,
            self.on_press,
            self.horizontal_alignment,
            self.vertical_alignment,
            self.padding_outer,
            self.class,
        )
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
    on_press: Option<impl Fn() -> Message + 'a>,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    padding: Padding,
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
        let backdrop = mouse_area(
            container(opaque(content))
                .style(move |theme| container::Style {
                    background: Some(
                        Catalog::style(theme, &class).backdrop_color.into(),
                    ),
                    ..Default::default()
                })
                .padding(padding)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(horizontal_alignment)
                .align_y(vertical_alignment),
        );

        if let Some(on_press) = on_press {
            opaque(backdrop.on_press(on_press()))
        } else {
            opaque(backdrop)
        }
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
