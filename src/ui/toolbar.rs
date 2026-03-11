use iced::widget::{button, container, image, row, Space};
use iced::{Background, Element};

use crate::app::Message;
use crate::theme::{EditorColors, ThemeMode};
use crate::tool::Tool;

const ICON_SIZE: f32 = 24.0;

pub fn view(active_tool: Tool, theme_mode: ThemeMode, colors: EditorColors) -> Element<'static, Message> {
    let mut items: Vec<Element<'static, Message>> = Vec::new();

    // Tool buttons
    for tool in [Tool::Select, Tool::Shape, Tool::Line, Tool::Spline] {
        items.push(tool_button(tool, tool == active_tool, colors));
    }

    items.push(Space::new().width(8).into());

    // Undo / Redo
    items.push(action_button("action_undo", Message::Undo, colors));
    items.push(action_button("action_redo", Message::Redo, colors));

    items.push(Space::new().width(8).into());

    // Save / Save As
    items.push(action_button("action_save", Message::SaveSvg, colors));
    items.push(action_button("action_save_as", Message::SaveSvgAs, colors));

    items.push(Space::new().width(8).into());

    // Theme toggle
    let theme_icon = match theme_mode {
        ThemeMode::Dark => "action_light_mode",
        ThemeMode::Light => "action_dark_mode",
    };
    items.push(action_button(theme_icon, Message::ToggleTheme, colors));

    let bg = colors.panel_bg;
    let border_color = colors.panel_border;

    container(
        row(items).spacing(2).align_y(iced::Alignment::Center),
    )
    .padding(4)
    .style(move |_theme| container::Style {
        background: Some(Background::Color(bg)),
        border: iced::Border {
            radius: 8.0.into(),
            width: 1.0,
            color: border_color,
        },
        ..Default::default()
    })
    .into()
}

fn icon(name: &str) -> image::Image<image::Handle> {
    image::Image::new(image::Handle::from_path(format!("assets/icons/{}.png", name)))
        .width(ICON_SIZE)
        .height(ICON_SIZE)
}

fn tool_button(tool: Tool, is_active: bool, colors: EditorColors) -> Element<'static, Message> {
    let icon_name = match tool {
        Tool::Select => "tool_select",
        Tool::Shape => "tool_shape",
        Tool::Line => "tool_line",
        Tool::Spline => "tool_spline",
    };

    let active_bg = colors.panel_button_active;
    let hover_bg = colors.panel_button_hover;

    button(icon(icon_name))
        .on_press(Message::ToolSelected(tool))
        .padding(4)
        .style(move |_theme, status| {
            let bg = if is_active {
                Some(Background::Color(active_bg))
            } else {
                match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        Some(Background::Color(hover_bg))
                    }
                    _ => None,
                }
            };
            button::Style {
                background: bg,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
}

fn action_button(icon_name: &str, message: Message, colors: EditorColors) -> Element<'static, Message> {
    let hover_bg = colors.panel_button_hover;

    button(icon(icon_name))
        .on_press(message)
        .padding(4)
        .style(move |_theme, status| {
            let bg = match status {
                button::Status::Hovered | button::Status::Pressed => {
                    Some(Background::Color(hover_bg))
                }
                _ => None,
            };
            button::Style {
                background: bg,
                border: iced::Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            }
        })
        .into()
}
