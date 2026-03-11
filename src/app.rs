use iced::widget::{canvas, column, container, row, Canvas};
use iced::{Color, Element, Length, Point, Task, Theme};

use crate::canvas::EditorCanvas;
use crate::document::Document;
use crate::palette::{self, Palette};
use crate::tool::{self, Tool, ToolEvent, ToolResult, ToolState};
use crate::viewport::Viewport;

pub struct App {
    document: Document,
    tool: Tool,
    tool_state: ToolState,
    viewport: Viewport,
    palette: Palette,
    palette_slug: String,
    palette_status: String,
    canvas_cache: canvas::Cache,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToolSelected(Tool),
    CanvasPress(Point),
    CanvasDrag(Point),
    CanvasRelease(Point),
    CanvasMove(Point),
    CanvasKeyEnter,
    Pan(f32, f32),
    Zoom(Point, f32),
    DeleteSelected,
    Undo,
    Redo,
    SaveSvg,
    SetStrokeWidth(f32),
    ClearFillColor,
    SetPolygonSides(usize),
    PaletteColorClicked(Color),
    PaletteSlugChanged(String),
    ImportPalette,
    PaletteLoaded(Result<Palette, String>),
    KeyboardEvent,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                document: Document::new(),
                tool: Tool::Rectangle,
                tool_state: ToolState::default(),
                viewport: Viewport::default(),
                palette: Palette::default(),
                palette_slug: String::new(),
                palette_status: String::new(),
                canvas_cache: canvas::Cache::new(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToolSelected(tool) => {
                self.tool_state.reset_drag();
                if self.tool == Tool::Pen && tool != Tool::Pen {
                    self.tool_state.reset_pen();
                }
                self.tool = tool;
                self.canvas_cache.clear();
            }
            Message::CanvasPress(pos) => {
                let result = self.dispatch_tool_event(ToolEvent::Press(pos));
                self.handle_tool_result(result);
            }
            Message::CanvasDrag(pos) => {
                let result = self.dispatch_tool_event(ToolEvent::Drag(pos));
                self.handle_tool_result(result);
            }
            Message::CanvasRelease(pos) => {
                let result = self.dispatch_tool_event(ToolEvent::Release(pos));
                self.handle_tool_result(result);
            }
            Message::CanvasMove(pos) => {
                self.tool_state.drag_current = Some(pos);
                let result = self.dispatch_tool_event(ToolEvent::Move(pos));
                self.handle_tool_result(result);
            }
            Message::CanvasKeyEnter => {
                let result = self.dispatch_tool_event(ToolEvent::KeyEnter);
                self.handle_tool_result(result);
            }
            Message::Pan(dx, dy) => {
                self.viewport.pan(dx, dy);
                self.canvas_cache.clear();
            }
            Message::Zoom(cursor, factor) => {
                self.viewport.zoom_at(cursor, factor);
                self.canvas_cache.clear();
            }
            Message::DeleteSelected => {
                if let Some(idx) = self.tool_state.selected_index {
                    self.document.remove_shape(idx);
                    self.tool_state.selected_index = None;
                    self.canvas_cache.clear();
                }
            }
            Message::Undo => {
                self.document.undo();
                self.tool_state.selected_index = None;
                self.canvas_cache.clear();
            }
            Message::Redo => {
                self.document.redo();
                self.tool_state.selected_index = None;
                self.canvas_cache.clear();
            }
            Message::SaveSvg => {
                self.save_svg();
            }
            Message::SetStrokeWidth(w) => {
                self.tool_state.current_style.stroke_width = w;
                self.canvas_cache.clear();
            }
            Message::ClearFillColor => {
                self.tool_state.current_style.fill_color = None;
                self.canvas_cache.clear();
            }
            Message::SetPolygonSides(n) => {
                self.tool_state.polygon_sides = n;
                self.canvas_cache.clear();
            }
            Message::PaletteColorClicked(color) => {
                self.tool_state.current_style.fill_color = Some(color);
                self.canvas_cache.clear();
            }
            Message::PaletteSlugChanged(slug) => {
                self.palette_slug = slug;
            }
            Message::ImportPalette => {
                let slug = self.palette_slug.clone();
                return Task::perform(
                    async move { palette::fetch_lospec_palette(&slug) },
                    Message::PaletteLoaded,
                );
            }
            Message::PaletteLoaded(result) => match result {
                Ok(p) => {
                    self.palette_status = format!("Loaded: {}", p.name);
                    self.palette = p;
                }
                Err(e) => {
                    self.palette_status = e;
                }
            },
            Message::KeyboardEvent => {}
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar = crate::ui::toolbar::view(self.tool);

        let canvas_widget: Element<Message> = Canvas::new(EditorCanvas {
            document: &self.document,
            tool: self.tool,
            tool_state: &self.tool_state,
            viewport: &self.viewport,
            selected_index: self.tool_state.selected_index,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        let sidebar = crate::ui::sidebar::view(
            self.tool,
            &self.tool_state.current_style,
            self.tool_state.polygon_sides,
            &self.palette,
            &self.palette_slug,
        );

        let content = row![canvas_widget, sidebar];

        container(column![toolbar, content])
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::keyboard::listen().map(|event| {
            use iced::keyboard::{Event as KbEvent, Key};
            match &event {
                KbEvent::KeyPressed { key, modifiers, .. } => {
                    if modifiers.control() {
                        match key {
                            Key::Character(c) if c.as_str() == "z" => {
                                if modifiers.shift() {
                                    return Message::Redo;
                                } else {
                                    return Message::Undo;
                                }
                            }
                            Key::Character(c) if c.as_str() == "y" => {
                                return Message::Redo;
                            }
                            _ => {}
                        }
                    }
                    Message::KeyboardEvent
                }
                _ => Message::KeyboardEvent,
            }
        })
    }

    fn dispatch_tool_event(&mut self, event: ToolEvent) -> ToolResult {
        match self.tool {
            Tool::Select => {
                tool::select::handle(&mut self.tool_state, event, &self.document)
            }
            Tool::Rectangle => tool::rectangle::handle(&mut self.tool_state, event),
            Tool::Circle => tool::circle::handle(&mut self.tool_state, event),
            Tool::RegularPolygon => tool::polygon::handle(&mut self.tool_state, event),
            Tool::Line => tool::line::handle(&mut self.tool_state, event),
            Tool::Pen => tool::pen::handle(&mut self.tool_state, event),
        }
    }

    fn handle_tool_result(&mut self, result: ToolResult) {
        match result {
            ToolResult::None => {}
            ToolResult::ShapeCompleted(shape) => {
                self.document.add_shape(shape);
                self.canvas_cache.clear();
            }
            ToolResult::SelectShape(idx) => {
                self.tool_state.selected_index = idx;
                self.canvas_cache.clear();
            }
            ToolResult::MoveSelected(dx, dy) => {
                if let Some(idx) = self.tool_state.selected_index {
                    self.document.move_shape(idx, dx, dy);
                    self.canvas_cache.clear();
                }
            }
            ToolResult::RequestRedraw => {
                self.canvas_cache.clear();
            }
        }
    }

    fn save_svg(&self) {
        let path = rfd::FileDialog::new()
            .add_filter("SVG", &["svg"])
            .set_file_name("drawing.svg")
            .save_file();

        if let Some(path) = path {
            let svg_doc = crate::export::export_svg(&self.document, 800.0, 600.0);
            if let Err(e) = svg::save(&path, &svg_doc) {
                eprintln!("Failed to save SVG: {}", e);
            }
        }
    }
}
