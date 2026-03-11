use iced::widget::{button, container, row, svg, Space};
use iced::{Background, Color, Element, Length};

use crate::app::{Message, SidebarMode};
use crate::theme::EditorColors;
use crate::tool::Tool;

const ICON_SIZE: f32 = 28.0;

fn separator<'a>(colors: EditorColors) -> Element<'a, Message> {
    let border_color = colors.panel_border;
    row![
        Space::new().width(4),
        container(Space::new().width(1).height(Length::Fill))
            .style(move |_theme| container::Style {
                background: Some(Background::Color(border_color)),
                ..Default::default()
            }),
        Space::new().width(4),
    ]
    .height(ICON_SIZE)
    .into()
}

pub fn view(active_tool: Tool, sidebar_mode: SidebarMode, colors: EditorColors) -> Element<'static, Message> {
    let mut items: Vec<Element<'static, Message>> = Vec::new();

    // Drawing tool buttons + Palette + Settings (all in one group)
    for tool in [Tool::Select, Tool::Shape, Tool::Line, Tool::Spline] {
        let is_active = tool == active_tool && sidebar_mode == SidebarMode::ToolConfig;
        items.push(tool_button_icon(tool_icon_name(tool), Message::ToolSelected(tool), is_active, colors));
    }
    items.push(tool_button_icon(
        "tool_palette",
        Message::SetSidebarMode(SidebarMode::Palette),
        sidebar_mode == SidebarMode::Palette,
        colors,
    ));
    items.push(tool_button_icon(
        "tool_settings",
        Message::SetSidebarMode(SidebarMode::Settings),
        sidebar_mode == SidebarMode::Settings,
        colors,
    ));

    items.push(separator(colors));

    // Undo / Redo
    items.push(action_button("action_undo", Message::Undo, colors));
    items.push(action_button("action_redo", Message::Redo, colors));

    items.push(separator(colors));

    // Save / Save As
    items.push(action_button("action_save", Message::SaveSvg, colors));
    items.push(action_button("action_save_as", Message::SaveSvgAs, colors));

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

fn tool_icon_name(tool: Tool) -> &'static str {
    match tool {
        Tool::Select => "tool_select",
        Tool::Shape => "tool_shape",
        Tool::Line => "tool_line",
        Tool::Spline => "tool_spline",
    }
}

fn icon(name: &str, color: Color) -> svg::Svg<'static> {
    svg::Svg::new(svg::Handle::from_path(format!("assets/icons/{}.svg", name)))
        .width(ICON_SIZE)
        .height(ICON_SIZE)
        .style(move |_theme, _status| svg::Style {
            color: Some(color),
        })
}

fn tool_button_icon(icon_name: &str, message: Message, is_active: bool, colors: EditorColors) -> Element<'static, Message> {
    let active_bg = colors.panel_button_active;
    let hover_bg = colors.panel_button_hover;
    let icon_color = colors.icon_color;

    button(icon(icon_name, icon_color))
        .on_press(message)
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
    let icon_color = colors.icon_color;

    button(icon(icon_name, icon_color))
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
