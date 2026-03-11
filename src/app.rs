use std::path::PathBuf;

use iced::widget::{canvas, container, row, stack, Canvas, Space};
use iced::{Element, Length, Padding, Point, Task, Theme};

use crate::canvas::EditorCanvas;
use crate::document::Document;
use crate::grid::{GridConfig, GridStyle};
use crate::palette::{self, Palette};
use crate::shape::{LineCap, LineJoin};
use crate::theme::{EditorColors, ThemeMode};
use crate::tool::{self, ShapeType, Tool, ToolEvent, ToolResult, ToolState};
use crate::viewport::Viewport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaletteTarget {
    Fill,
    Stroke,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarMode {
    ToolConfig,
    Palette,
    Settings,
}

pub struct App {
    document: Document,
    tool: Tool,
    tool_state: ToolState,
    viewport: Viewport,
    palette: Palette,
    palette_slug: String,
    palette_status: String,
    canvas_cache: canvas::Cache,
    grid: GridConfig,
    palette_target: Option<PaletteTarget>,
    stroke_color_index: Option<usize>,
    fill_color_index: Option<usize>,
    palette_reorder: Option<usize>, // index being moved (1-based, 0=None is immovable)
    palette_reorder_mode: bool,
    theme_mode: ThemeMode,
    editor_colors: EditorColors,
    save_path: Option<PathBuf>,
    polygon_submenu_open: bool,
    sidebar_mode: SidebarMode,
    color_picker_target: Option<usize>,
    color_picker_r: f32,
    color_picker_g: f32,
    color_picker_b: f32,
    default_palette: Palette,
    base_text_size: f32,
    settings_color_field: Option<String>,
    settings_picker_r: f32,
    settings_picker_g: f32,
    settings_picker_b: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    ToolSelected(Tool),
    CanvasPress(Point),
    CanvasDrag(Point),
    CanvasRelease(Point),
    CanvasMove(Point),
    CanvasKeyEnter,
    CanvasRightClick(Point),
    Pan(f32, f32),
    Zoom(Point, f32),
    DeleteSelected,
    Undo,
    Redo,
    SaveSvg,
    SaveSvgAs,
    SetStrokeWidth(f32),
    SetShapeType(ShapeType),
    SetSkewAngle(f32),
    SetPaletteTarget(PaletteTarget),
    PaletteColorClicked(usize),
    PaletteReorderToggle,
    PaletteReorderPickUp(usize),
    PaletteReorderDrop(usize),
    PaletteSlugChanged(String),
    ImportPalette,
    PaletteLoaded(Result<Palette, String>),
    KeyboardEvent,
    // Grid
    SetGridStyle(GridStyle),
    ToggleGridVisible(bool),
    ToggleGridSnap(bool),
    // Shape editing
    SetSelectedStrokeWidth(f32),
    SetSelectedCornerRadius(f32),
    SetSelectedLineCap(LineCap),
    SetSelectedLineJoin(LineJoin),
    // Sidebar mode
    SetSidebarMode(SidebarMode),
    // Theme
    SetThemeMode(ThemeMode),
    // Grid size
    SetGridSize(f32),
    // Palette management
    AddPaletteColor,
    DeletePaletteColor(usize),
    EditPaletteColor(usize),
    ColorPickerR(f32),
    ColorPickerG(f32),
    ColorPickerB(f32),
    ColorPickerApply,
    ColorPickerCancel,
    ResetPalette,
    SetAsDefaultPalette,
    // Settings
    SetBaseTextSize(f32),
    EditThemeColor(String),
    SettingsPickerR(f32),
    SettingsPickerG(f32),
    SettingsPickerB(f32),
    SettingsPickerApply,
    SettingsPickerCancel,
    ResetThemeColors,
    // Polygon submenu
    TogglePolygonSubmenu,
    // Stroke width text input
    StrokeWidthInput(String),
    SelectedStrokeWidthInput(String),
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                document: Document::new(),
                tool: Tool::Shape,
                tool_state: ToolState::default(),
                viewport: Viewport::default(),
                palette: Palette::default(),
                palette_slug: String::new(),
                palette_status: String::new(),
                canvas_cache: canvas::Cache::new(),
                grid: GridConfig::default(),
                palette_target: None,
                stroke_color_index: Some(1), // black
                fill_color_index: None,      // none
                palette_reorder: None,
                palette_reorder_mode: false,
                theme_mode: ThemeMode::Dark,
                editor_colors: EditorColors::dark(),
                save_path: None,
                polygon_submenu_open: false,
                sidebar_mode: SidebarMode::ToolConfig,
                color_picker_target: None,
                color_picker_r: 1.0,
                color_picker_g: 1.0,
                color_picker_b: 1.0,
                default_palette: Palette::default(),
                base_text_size: 11.0,
                settings_color_field: None,
                settings_picker_r: 0.0,
                settings_picker_g: 0.0,
                settings_picker_b: 0.0,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ToolSelected(tool) => {
                self.tool_state.reset_drag();
                if self.tool == Tool::Spline && tool != Tool::Spline {
                    self.tool_state.reset_pen();
                }
                if self.tool == Tool::Line && tool != Tool::Line {
                    self.tool_state.reset_line();
                }
                self.tool = tool;
                self.sidebar_mode = SidebarMode::ToolConfig;
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
            Message::CanvasRightClick(pos) => {
                let result = self.dispatch_tool_event(ToolEvent::RightClick(pos));
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
                if let Some(path) = &self.save_path {
                    let svg_doc = crate::export::export_svg(&self.document, 800.0, 600.0);
                    if let Err(e) = svg::save(path, &svg_doc) {
                        eprintln!("Failed to save SVG: {}", e);
                    }
                } else {
                    self.save_svg_as();
                }
            }
            Message::SaveSvgAs => {
                self.save_svg_as();
            }
            Message::SetStrokeWidth(w) => {
                self.tool_state.current_style.stroke_width = w;
                self.canvas_cache.clear();
            }
            Message::StrokeWidthInput(s) => {
                if let Ok(w) = s.parse::<f32>() {
                    let w = w.clamp(0.0, 20.0);
                    self.tool_state.current_style.stroke_width = w;
                    self.canvas_cache.clear();
                }
            }
            Message::SetShapeType(t) => {
                self.tool_state.shape_type = t;
                self.polygon_submenu_open = false;
                self.canvas_cache.clear();
            }
            Message::TogglePolygonSubmenu => {
                self.polygon_submenu_open = !self.polygon_submenu_open;
            }
            Message::SetSkewAngle(a) => {
                self.tool_state.skew_angle = a;
                self.canvas_cache.clear();
            }
            Message::SetPaletteTarget(target) => {
                if self.palette_target == Some(target) {
                    self.palette_target = None; // collapse
                } else {
                    self.palette_target = Some(target); // expand
                }
            }
            Message::PaletteReorderToggle => {
                self.palette_reorder_mode = !self.palette_reorder_mode;
                self.palette_reorder = None;
            }
            Message::PaletteReorderPickUp(idx) => {
                if idx == 0 { return Task::none(); } // can't move None
                if self.palette_reorder == Some(idx) {
                    self.palette_reorder = None; // deselect
                } else {
                    self.palette_reorder = Some(idx);
                }
            }
            Message::PaletteReorderDrop(target_idx) => {
                if let Some(src_idx) = self.palette_reorder {
                    if src_idx > 0 && target_idx > 0 && src_idx != target_idx {
                        // Convert from 1-based UI indices to 0-based vec indices
                        let src_vec = src_idx - 1;
                        let target_vec = target_idx - 1;

                        // Remove from old position
                        let color = self.palette.colors.remove(src_vec);

                        // Adjust target if it was after the source
                        let insert_at = if target_vec > src_vec {
                            target_vec - 1
                        } else {
                            target_vec
                        };

                        // Clamp to valid range
                        let insert_at = insert_at.min(self.palette.colors.len());
                        self.palette.colors.insert(insert_at, color);

                        // Update tracked indices to follow their colors
                        let new_ui_idx = insert_at + 1; // back to 1-based
                        // Helper: remap an index after a move from src to insert_at (0-based vec)
                        let remap = |idx: usize| -> usize {
                            let vec_idx = idx - 1; // to 0-based
                            if idx == src_idx {
                                new_ui_idx
                            } else {
                                let adjusted = if vec_idx >= src_vec && vec_idx > 0 {
                                    vec_idx - 1
                                } else {
                                    vec_idx
                                };
                                let final_idx = if adjusted >= insert_at {
                                    adjusted + 1
                                } else {
                                    adjusted
                                };
                                final_idx + 1 // back to 1-based
                            }
                        };

                        if let Some(si) = self.stroke_color_index {
                            self.stroke_color_index = Some(remap(si));
                        }
                        if let Some(fi) = self.fill_color_index {
                            self.fill_color_index = Some(remap(fi));
                        }
                    }
                    self.palette_reorder = None;
                }
            }
            Message::PaletteColorClicked(color_index) => {
                let target = match self.palette_target {
                    Some(t) => t,
                    None => return Task::none(),
                };
                // Index 0 = None, index 1+ = palette.colors[i-1]
                let color = if color_index == 0 {
                    None
                } else {
                    self.palette.colors.get(color_index - 1).copied()
                };

                // Update tracked indices
                match target {
                    PaletteTarget::Fill => self.fill_color_index = if color_index == 0 { None } else { Some(color_index) },
                    PaletteTarget::Stroke => {
                        self.stroke_color_index = if color_index == 0 { None } else { Some(color_index) };
                    }
                }

                if let Some(idx) = self.tool_state.selected_index {
                    if self.tool == Tool::Select {
                        let mut shape = self.document.shapes[idx].clone();
                        match target {
                            PaletteTarget::Fill => shape.style_mut().fill_color = color,
                            PaletteTarget::Stroke => shape.style_mut().stroke_color = color,
                        }
                        self.document.update_shape(idx, shape);
                        self.canvas_cache.clear();
                        return Task::none();
                    }
                }
                match target {
                    PaletteTarget::Fill => self.tool_state.current_style.fill_color = color,
                    PaletteTarget::Stroke => self.tool_state.current_style.stroke_color = color,
                }
                self.palette_target = None; // collapse after selection
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
                    self.palette_reorder = None;
                    self.palette_reorder_mode = false;
                    self.stroke_color_index = Some(1);
                    self.fill_color_index = None;
                }
                Err(e) => {
                    self.palette_status = e;
                }
            },
            Message::KeyboardEvent => {}
            // Grid
            Message::SetGridStyle(style) => {
                self.grid.style = style;
                self.canvas_cache.clear();
            }
            Message::ToggleGridVisible(visible) => {
                self.grid.visible = visible;
                self.canvas_cache.clear();
            }
            Message::ToggleGridSnap(snap) => {
                self.grid.snap = snap;
                self.canvas_cache.clear();
            }
            // Shape editing
            Message::SetSelectedStrokeWidth(w) => {
                if let Some(idx) = self.tool_state.selected_index {
                    let mut shape = self.document.shapes[idx].clone();
                    shape.style_mut().stroke_width = w;
                    self.document.update_shape(idx, shape);
                    self.canvas_cache.clear();
                }
            }
            Message::SelectedStrokeWidthInput(s) => {
                if let Ok(w) = s.parse::<f32>() {
                    let w = w.clamp(0.0, 20.0);
                    if let Some(idx) = self.tool_state.selected_index {
                        let mut shape = self.document.shapes[idx].clone();
                        shape.style_mut().stroke_width = w;
                        self.document.update_shape(idx, shape);
                        self.canvas_cache.clear();
                    }
                }
            }
            Message::SetSelectedCornerRadius(r) => {
                if let Some(idx) = self.tool_state.selected_index {
                    let mut shape = self.document.shapes[idx].clone();
                    shape.set_corner_radius(r);
                    self.document.update_shape(idx, shape);
                    self.canvas_cache.clear();
                }
            }
            Message::SetSelectedLineCap(cap) => {
                if let Some(idx) = self.tool_state.selected_index {
                    let mut shape = self.document.shapes[idx].clone();
                    shape.style_mut().line_cap = cap;
                    self.document.update_shape(idx, shape);
                    self.canvas_cache.clear();
                }
            }
            Message::SetSelectedLineJoin(join) => {
                if let Some(idx) = self.tool_state.selected_index {
                    let mut shape = self.document.shapes[idx].clone();
                    shape.style_mut().line_join = join;
                    self.document.update_shape(idx, shape);
                    self.canvas_cache.clear();
                }
            }
            Message::SetSidebarMode(mode) => {
                if self.sidebar_mode == mode {
                    self.sidebar_mode = SidebarMode::ToolConfig;
                } else {
                    self.sidebar_mode = mode;
                }
            }
            Message::SetThemeMode(mode) => {
                self.theme_mode = mode;
                self.editor_colors = EditorColors::from_mode(self.theme_mode);
                self.canvas_cache.clear();
            }
            Message::SetGridSize(size) => {
                self.grid.size = size;
                self.canvas_cache.clear();
            }
            Message::AddPaletteColor => {
                self.palette.colors.push(iced::Color::WHITE);
            }
            Message::DeletePaletteColor(index) => {
                if index > 0 && index <= self.palette.colors.len() {
                    self.palette.colors.remove(index - 1);
                    // Adjust stroke_color_index
                    if let Some(si) = self.stroke_color_index {
                        if si == index { self.stroke_color_index = None; }
                        else if si > index { self.stroke_color_index = Some(si - 1); }
                    }
                    // Adjust fill_color_index
                    if let Some(fi) = self.fill_color_index {
                        if fi == index { self.fill_color_index = None; }
                        else if fi > index { self.fill_color_index = Some(fi - 1); }
                    }
                    // Close color picker if editing deleted color
                    if self.color_picker_target == Some(index) {
                        self.color_picker_target = None;
                    } else if let Some(t) = self.color_picker_target {
                        if t > index { self.color_picker_target = Some(t - 1); }
                    }
                }
            }
            Message::EditPaletteColor(index) => {
                if index > 0 && index <= self.palette.colors.len() {
                    let c = self.palette.colors[index - 1];
                    self.color_picker_target = Some(index);
                    self.color_picker_r = c.r;
                    self.color_picker_g = c.g;
                    self.color_picker_b = c.b;
                } else {
                    self.color_picker_target = None;
                }
            }
            Message::ColorPickerR(v) => { self.color_picker_r = v; }
            Message::ColorPickerG(v) => { self.color_picker_g = v; }
            Message::ColorPickerB(v) => { self.color_picker_b = v; }
            Message::ColorPickerApply => {
                if let Some(idx) = self.color_picker_target {
                    if idx > 0 && idx <= self.palette.colors.len() {
                        self.palette.colors[idx - 1] = iced::Color::from_rgb(
                            self.color_picker_r,
                            self.color_picker_g,
                            self.color_picker_b,
                        );
                    }
                }
                self.color_picker_target = None;
                self.canvas_cache.clear();
            }
            Message::ColorPickerCancel => {
                self.color_picker_target = None;
            }
            Message::ResetPalette => {
                self.palette = self.default_palette.clone();
                self.stroke_color_index = Some(1);
                self.fill_color_index = None;
                self.color_picker_target = None;
                self.canvas_cache.clear();
            }
            Message::SetAsDefaultPalette => {
                self.default_palette = self.palette.clone();
            }
            Message::SetBaseTextSize(s) => {
                self.base_text_size = s;
            }
            Message::EditThemeColor(field) => {
                let c = self.editor_colors.get_field(&field);
                self.settings_color_field = Some(field);
                self.settings_picker_r = c.r;
                self.settings_picker_g = c.g;
                self.settings_picker_b = c.b;
            }
            Message::SettingsPickerR(v) => { self.settings_picker_r = v; }
            Message::SettingsPickerG(v) => { self.settings_picker_g = v; }
            Message::SettingsPickerB(v) => { self.settings_picker_b = v; }
            Message::SettingsPickerApply => {
                if let Some(ref field) = self.settings_color_field {
                    let color = iced::Color::from_rgb(
                        self.settings_picker_r,
                        self.settings_picker_g,
                        self.settings_picker_b,
                    );
                    self.editor_colors.set_field(field, color);
                }
                self.settings_color_field = None;
                self.canvas_cache.clear();
            }
            Message::SettingsPickerCancel => {
                self.settings_color_field = None;
            }
            Message::ResetThemeColors => {
                self.editor_colors = EditorColors::from_mode(self.theme_mode);
                self.settings_color_field = None;
                self.canvas_cache.clear();
            }
        }
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        let toolbar = crate::ui::toolbar::view(self.tool, self.sidebar_mode, self.editor_colors);

        let canvas_widget: Element<Message> = Canvas::new(EditorCanvas {
            document: &self.document,
            tool: self.tool,
            tool_state: &self.tool_state,
            viewport: &self.viewport,
            selected_index: self.tool_state.selected_index,
            grid: &self.grid,
            colors: &self.editor_colors,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into();

        let selected_shape = self.tool_state.selected_index
            .and_then(|i| self.document.shapes.get(i));

        let sidebar = crate::ui::sidebar::view(
            self.sidebar_mode,
            self.tool,
            &self.tool_state.current_style,
            self.tool_state.shape_type,
            self.tool_state.skew_angle,
            &self.palette,
            &self.palette_slug,
            &self.palette_status,
            &self.grid,
            selected_shape,
            self.palette_target,
            self.stroke_color_index,
            self.fill_color_index,
            self.palette_reorder_mode,
            self.palette_reorder,
            self.editor_colors,
            self.polygon_submenu_open,
            self.color_picker_target,
            self.color_picker_r,
            self.color_picker_g,
            self.color_picker_b,
            self.theme_mode,
            self.base_text_size,
            self.settings_color_field.as_deref(),
            self.settings_picker_r,
            self.settings_picker_g,
            self.settings_picker_b,
        );

        // Full-page canvas with floating panels on top
        stack![
            canvas_widget,
            // Toolbar centered at top
            container(toolbar)
                .center_x(Length::Fill)
                .padding(Padding { top: 8.0, right: 0.0, bottom: 0.0, left: 0.0 }),
            // Sidebar at top right
            container(
                row![
                    Space::new().width(Length::Fill),
                    sidebar,
                ]
            )
            .width(Length::Fill)
            .padding(Padding { top: 56.0, right: 8.0, bottom: 8.0, left: 0.0 }),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn theme(&self) -> Theme {
        crate::theme::iced_theme(self.theme_mode)
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
                            Key::Character(c) if c.as_str() == "s" => {
                                if modifiers.shift() {
                                    return Message::SaveSvgAs;
                                } else {
                                    return Message::SaveSvg;
                                }
                            }
                            Key::Character(c) if c.as_str() == "t" => {
                                return Message::SetSidebarMode(SidebarMode::Settings);
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
            Tool::Shape => tool::shape::handle(&mut self.tool_state, event),
            Tool::Line => tool::line::handle(&mut self.tool_state, event),
            Tool::Spline => tool::spline::handle(&mut self.tool_state, event),
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

    fn save_svg_as(&mut self) {
        let path = rfd::FileDialog::new()
            .add_filter("SVG", &["svg"])
            .set_file_name("drawing.svg")
            .save_file();

        if let Some(path) = path {
            let svg_doc = crate::export::export_svg(&self.document, 800.0, 600.0);
            if let Err(e) = svg::save(&path, &svg_doc) {
                eprintln!("Failed to save SVG: {}", e);
            }
            self.save_path = Some(path);
        }
    }
}
