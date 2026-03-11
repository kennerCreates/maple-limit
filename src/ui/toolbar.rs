use iced::widget::{button, row, text};
use iced::Element;

use crate::app::Message;
use crate::tool::Tool;

const TOOLS: &[Tool] = &[
    Tool::Select,
    Tool::Rectangle,
    Tool::Circle,
    Tool::RegularPolygon,
    Tool::Line,
    Tool::Pen,
];

pub fn view(active_tool: Tool) -> Element<'static, Message> {
    let tool_buttons = TOOLS.iter().map(|tool| {
        let label = tool.label();
        let btn = button(text(label).size(13));
        let btn = if *tool == active_tool {
            btn.style(button::primary)
        } else {
            btn.style(button::secondary)
        };
        btn.on_press(Message::ToolSelected(*tool)).into()
    });

    let save_btn: Element<'static, Message> = button(text("Save SVG").size(13))
        .on_press(Message::SaveSvg)
        .style(button::success)
        .into();

    let mut items: Vec<Element<'static, Message>> = tool_buttons.collect();
    items.push(save_btn);

    row(items).spacing(4).padding(6).into()
}
