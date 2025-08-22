use iced::Event;
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::widget::{self, Tree, Widget, tree};
use iced::gradient::{ColorStop, Linear};
use iced::{Background, Padding, border, event, mouse, touch};
use iced::{Color, Element, Length, Rectangle, Size};
use iced_widget::button::{Catalog, Status};
use iced_widget::core::window;
use std::vec;

pub struct AeroButton<'a, Message, Theme, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<OnPress<'a, Message>>,
    width: Length,
    height: Length,
    padding: Padding,
    clip: bool,
    status: Option<Status>,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

// get message from OnPress enum
impl<'a, Message: Clone> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message, Theme, Renderer> AeroButton<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Theme: Catalog,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        Self {
            content,
            on_press: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            padding: DEFAULT_PADDING,
            clip: false,
            status: None,
        }
    }
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }

    pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
        self.on_press = on_press.map(OnPress::Direct);
        self
    }

    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }
}

pub fn aero_button<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> AeroButton<'a, Message, Theme, Renderer>
where
    Theme: Catalog + 'a,
    Renderer: iced::advanced::Renderer,
{
    AeroButton::new(content)
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
struct State {
    is_pressed: bool,
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for AeroButton<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Theme: 'a + Catalog,
    Renderer: renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
{
    fn state(&self) -> widget::tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Shrink,
            height: Length::Shrink,
        }
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn operate(
        &self,
        tree: &mut tree::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn layout(
        &self,
        tree: &mut widget::Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::padded(limits, self.width, self.height, self.padding, |limits| {
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits)
        })
    }

    fn draw(
        &self,
        tree: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let status = self.status.unwrap();

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: border::Border::default()
                    .rounded(4)
                    .color(match status {
                        Status::Hovered => {
                            Color::from_rgb(72.0 / 255.0, 135.0 / 255.0, 182.0 / 255.0)
                        }
                        Status::Pressed => {
                            Color::from_rgb(109.0 / 255.0, 145.0 / 255.0, 171.0 / 255.0)
                        }
                        Status::Disabled => {
                            Color::from_rgb(173.0 / 255.0, 178.0 / 255.0, 181.0 / 255.0)
                        }
                        _ => Color::from_rgb(107.0 / 255.0, 107.0 / 255.0, 107.0 / 255.0),
                    })
                    .width(1),
                ..renderer::Quad::default()
            },
            if status == Status::Pressed {
                Background::Color(Color::from_rgb(109.0 / 255.0, 145.0 / 255.0, 171.0 / 255.0))
            } else {
                Background::Color(Color::from_rgb(1.0, 1.0, 1.0))
            },
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: if status == Status::Pressed {
                        bounds.x + 2.5
                    } else {
                        bounds.x + 2.0
                    },
                    y: if status == Status::Pressed {
                        bounds.y + 2.3
                    } else {
                        bounds.y + 2.0
                    },
                    width: bounds.width - 4.0,
                    height: bounds.height - 4.0,
                },
                border: border::Border::default().rounded(3),
                ..renderer::Quad::default()
            },
            match status {
                Status::Hovered => {
                    Background::Gradient(iced::Gradient::Linear(Linear::new(0).add_stops([
                        ColorStop {
                            offset: 0.0,
                            color: Color::from_rgb(169.0 / 255.0, 219.0 / 255.0, 246.0 / 255.0),
                        },
                        ColorStop {
                            offset: 0.49,
                            color: Color::from_rgb(190.0 / 255.0, 230.0 / 255.0, 253.0 / 255.0),
                        },
                        ColorStop {
                            offset: 0.50,
                            color: Color::from_rgb(220.0 / 255.0, 241.0 / 255.0, 252.0 / 255.0),
                        },
                        ColorStop {
                            offset: 1.0,
                            color: Color::from_rgb(240.0 / 255.0, 249.0 / 255.0, 253.0 / 255.0),
                        },
                    ])))
                }
                Status::Pressed => {
                    Background::Gradient(iced::Gradient::Linear(Linear::new(0).add_stops([
                        ColorStop {
                            offset: 0.0,
                            color: Color::from_rgb(108.0 / 255.0, 182.0 / 255.0, 221.0 / 255.0),
                        },
                        ColorStop {
                            offset: 0.49,
                            color: Color::from_rgb(146.0 / 255.0, 205.0 / 255.0, 237.0 / 255.0),
                        },
                        ColorStop {
                            offset: 0.50,
                            color: Color::from_rgb(196.0 / 255.0, 229.0 / 255.0, 246.0 / 255.0),
                        },
                        ColorStop {
                            offset: 1.0,
                            color: Color::from_rgb(215.0 / 255.0, 232.0 / 255.0, 242.0 / 255.0),
                        },
                    ])))
                }
                Status::Disabled => {
                    Background::Color(Color::from_rgb(244.0 / 255.0, 244.0 / 255.0, 244.0 / 255.0))
                }
                _ => Background::Gradient(iced::Gradient::Linear(Linear::new(0).add_stops([
                    ColorStop {
                        offset: 0.0,
                        color: Color::from_rgb(208.0 / 255.0, 208.0 / 255.0, 208.0 / 255.0),
                    },
                    ColorStop {
                        offset: 0.49,
                        color: Color::from_rgb(220.0 / 255.0, 220.0 / 255.0, 220.0 / 255.0),
                    },
                    ColorStop {
                        offset: 0.50,
                        color: Color::from_rgb(234.0 / 255.0, 234.0 / 255.0, 234.0 / 255.0),
                    },
                    ColorStop {
                        offset: 1.0,
                        color: Color::from_rgb(242.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0),
                    },
                ]))),
            },
        );

        let viewport = if self.clip {
            bounds.intersection(viewport).unwrap_or(*viewport)
        } else {
            *viewport
        };

        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            &viewport,
        );
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn iced::advanced::Clipboard,
        shell: &mut iced::advanced::Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> iced_widget::core::event::Status {
        if let event::Status::Captured = self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        ) {
            return event::Status::Captured;
        }

        match event {
            // Pressed
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if self.on_press.is_some() {
                    let bounds = layout.bounds();
                    if cursor.is_over(bounds) {
                        let state = tree.state.downcast_mut::<State>();
                        state.is_pressed = true;

                        return event::Status::Captured;
                    }
                }
            }

            // Released
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(on_press) = self.on_press.as_ref().map(OnPress::get) {
                    let state = tree.state.downcast_mut::<State>();

                    if state.is_pressed {
                        state.is_pressed = false;

                        let bounds = layout.bounds();

                        if cursor.is_over(bounds) {
                            shell.publish(on_press);
                        }
                    }
                }
                return event::Status::Captured;
            }

            Event::Touch(touch::Event::FingerLost { .. }) => {
                let state = tree.state.downcast_mut::<State>();
                state.is_pressed = false;

                return event::Status::Captured;
            }

            _ => {}
        }

        let current_status = if self.on_press.is_none() {
            Status::Disabled
        } else if cursor.is_over(layout.bounds()) {
            let state = tree.state.downcast_ref::<State>();
            if state.is_pressed {
                Status::Pressed
            } else {
                Status::Hovered
            }
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw(window::RedrawRequest::NextFrame);
        };

        event::Status::Ignored
    }
}

pub(crate) const DEFAULT_PADDING: Padding = Padding {
    top: 6.0,
    bottom: 6.0,
    right: 20.0,
    left: 20.0,
};

impl<'a, Message, Theme, Renderer> From<AeroButton<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer<Font = iced::Font>,
    Theme: Catalog + 'a,
    Message: Clone + 'a,
{
    fn from(value: AeroButton<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
