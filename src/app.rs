use std::path::PathBuf;

use iced::widget::{canvas, container, row, stack, Canvas, Space};
use iced::{Element, Length, Padding, Point, Task, Theme};

use crate::canvas::EditorCanvas;
use crate::document::Document;
use crate::grid::{GridConfig, GridStyle};
use crate::palette::{self, Palette};
use crate::shape::{LineCap, LineJoin};
use crate::theme::{EditorColors, ThemeMapping, ThemeMode, ThemePalette};
use crate::tool::{self, ShapeType, Tool, ToolEvent, ToolResult, ToolState};
use crate::settings::Settings;
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
    theme_palette: ThemePalette,
    theme_mapping: ThemeMapping,
    other_palette: ThemePalette,  // stored palette for the inactive mode
    other_mapping: ThemeMapping,  // stored mapping for the inactive mode
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
    settings_editing_palette_idx: Option<usize>,
    settings_picker_r: f32,
    settings_picker_g: f32,
    settings_picker_b: f32,
    theme_palette_slug: String,
    theme_palette_status: String,
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
    // Settings - theme palette
    SetBaseTextSize(f32),
    BaseTextSizeInput(String),
    EditThemePaletteColor(usize),
    SettingsPickerR(f32),
    SettingsPickerG(f32),
    SettingsPickerB(f32),
    SettingsPickerApply,
    SettingsPickerCancel,
    SetElementPaletteIndex(usize, usize),
    ResetThemePalette,
    ResetThemeMapping,
    ImportThemePalette,
    ThemePaletteSlugChanged(String),
    ThemePaletteLoaded(Result<crate::palette::Palette, String>),
    // Polygon submenu
    TogglePolygonSubmenu,
    // Stroke width text input
    StrokeWidthInput(String),
    SelectedStrokeWidthInput(String),
    SelectedCornerRadiusInput(String),
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        let saved = Settings::load();

        let theme_mode = saved.as_ref().map(|s| s.theme_mode).unwrap_or(ThemeMode::Dark);
        let dark_palette = saved.as_ref().map(|s| s.dark_palette.clone()).unwrap_or_else(ThemePalette::default_dark);
        let dark_mapping = saved.as_ref().map(|s| s.dark_mapping).unwrap_or_else(ThemeMapping::default_dark);
        let light_palette = saved.as_ref().map(|s| s.light_palette.clone()).unwrap_or_else(ThemePalette::default_light);
        let light_mapping = saved.as_ref().map(|s| s.light_mapping).unwrap_or_else(ThemeMapping::default_light);
        let base_text_size = saved.as_ref().map(|s| s.base_text_size).unwrap_or(11.0);
        let grid = saved.map(|s| GridConfig {
            style: s.grid_style,
            size: s.grid_size,
            visible: s.grid_visible,
            snap: s.grid_snap,
        }).unwrap_or_default();

        let (theme_palette, theme_mapping, other_palette, other_mapping) = match theme_mode {
            ThemeMode::Dark => (dark_palette, dark_mapping, light_palette, light_mapping),
            ThemeMode::Light => (light_palette, light_mapping, dark_palette, dark_mapping),
        };

        let editor_colors = EditorColors::from_palette(&theme_palette, theme_mode, &theme_mapping);

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
                grid,
                palette_target: None,
                stroke_color_index: Some(1),
                fill_color_index: None,
                palette_reorder: None,
                palette_reorder_mode: false,
                theme_mode,
                theme_palette,
                theme_mapping,
                other_palette,
                other_mapping,
                editor_colors,
                save_path: None,
                polygon_submenu_open: false,
                sidebar_mode: SidebarMode::ToolConfig,
                color_picker_target: None,
                color_picker_r: 1.0,
                color_picker_g: 1.0,
                color_picker_b: 1.0,
                default_palette: Palette::default(),
                base_text_size,
                settings_editing_palette_idx: None,
                settings_picker_r: 0.0,
                settings_picker_g: 0.0,
                settings_picker_b: 0.0,
                theme_palette_slug: String::new(),
                theme_palette_status: String::new(),
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
                self.save_settings();
            }
            Message::ToggleGridVisible(visible) => {
                self.grid.visible = visible;
                self.canvas_cache.clear();
                self.save_settings();
            }
            Message::ToggleGridSnap(snap) => {
                self.grid.snap = snap;
                self.canvas_cache.clear();
                self.save_settings();
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
            Message::SelectedCornerRadiusInput(s) => {
                if let Ok(r) = s.parse::<f32>() {
                    let r = r.clamp(0.0, 100.0);
                    if let Some(idx) = self.tool_state.selected_index {
                        let mut shape = self.document.shapes[idx].clone();
                        shape.set_corner_radius(r);
                        self.document.update_shape(idx, shape);
                        self.canvas_cache.clear();
                    }
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
                if mode != self.theme_mode {
                    // Swap current into other, other into current
                    std::mem::swap(&mut self.theme_palette, &mut self.other_palette);
                    std::mem::swap(&mut self.theme_mapping, &mut self.other_mapping);
                    self.theme_mode = mode;
                    self.settings_editing_palette_idx = None;
                    self.rebuild_editor_colors();
                    self.canvas_cache.clear();
                    self.save_settings();
                }
            }
            Message::SetGridSize(size) => {
                self.grid.size = size;
                self.canvas_cache.clear();
                self.save_settings();
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
            Message::SetBaseTextSize(v) => {
                self.base_text_size = v.clamp(9.0, 18.0);
                self.save_settings();
            }
            Message::BaseTextSizeInput(s) => {
                if let Ok(v) = s.parse::<f32>() {
                    self.base_text_size = v.clamp(9.0, 18.0);
                    self.save_settings();
                }
            }
            Message::EditThemePaletteColor(idx) => {
                if idx < 5 {
                    let c = self.theme_palette.colors[idx];
                    self.settings_editing_palette_idx = Some(idx);
                    self.settings_picker_r = c.r;
                    self.settings_picker_g = c.g;
                    self.settings_picker_b = c.b;
                }
            }
            Message::SettingsPickerR(v) => { self.settings_picker_r = v; }
            Message::SettingsPickerG(v) => { self.settings_picker_g = v; }
            Message::SettingsPickerB(v) => { self.settings_picker_b = v; }
            Message::SettingsPickerApply => {
                if let Some(idx) = self.settings_editing_palette_idx {
                    if idx < 5 {
                        self.theme_palette.colors[idx] = iced::Color::from_rgb(
                            self.settings_picker_r,
                            self.settings_picker_g,
                            self.settings_picker_b,
                        );
                        self.rebuild_editor_colors();
                        self.save_settings();
                    }
                }
                self.settings_editing_palette_idx = None;
                self.canvas_cache.clear();
            }
            Message::SettingsPickerCancel => {
                self.settings_editing_palette_idx = None;
            }
            Message::SetElementPaletteIndex(element_idx, palette_idx) => {
                if element_idx < 7 && palette_idx < 5 {
                    self.theme_mapping.indices[element_idx] = palette_idx;
                    self.rebuild_editor_colors();
                    self.canvas_cache.clear();
                    self.save_settings();
                }
            }
            Message::ResetThemePalette => {
                self.theme_palette = ThemePalette::from_mode(self.theme_mode);
                self.settings_editing_palette_idx = None;
                self.rebuild_editor_colors();
                self.canvas_cache.clear();
                self.save_settings();
            }
            Message::ResetThemeMapping => {
                self.theme_mapping = ThemeMapping::from_mode(self.theme_mode);
                self.rebuild_editor_colors();
                self.canvas_cache.clear();
                self.save_settings();
            }
            Message::ImportThemePalette => {
                let slug = self.theme_palette_slug.clone();
                return Task::perform(
                    async move { palette::fetch_lospec_palette(&slug) },
                    Message::ThemePaletteLoaded,
                );
            }
            Message::ThemePaletteSlugChanged(slug) => {
                self.theme_palette_slug = slug;
            }
            Message::ThemePaletteLoaded(result) => match result {
                Ok(p) => {
                    self.theme_palette_status = format!("Loaded: {}", p.name);
                    self.theme_palette.name = p.name;
                    for (i, &c) in p.colors.iter().take(5).enumerate() {
                        self.theme_palette.colors[i] = c;
                    }
                    self.settings_editing_palette_idx = None;
                    self.rebuild_editor_colors();
                    self.canvas_cache.clear();
                    self.save_settings();
                }
                Err(e) => {
                    self.theme_palette_status = e;
                }
            },
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
            &self.theme_palette,
            &self.theme_mapping,
            &self.theme_palette_slug,
            &self.theme_palette_status,
            self.settings_editing_palette_idx,
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
        crate::theme::iced_theme(&self.theme_palette, self.theme_mode)
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

    fn rebuild_editor_colors(&mut self) {
        self.editor_colors = EditorColors::from_palette(
            &self.theme_palette,
            self.theme_mode,
            &self.theme_mapping,
        );
    }

    fn save_settings(&self) {
        let (dark_palette, dark_mapping, light_palette, light_mapping) = match self.theme_mode {
            ThemeMode::Dark => (&self.theme_palette, &self.theme_mapping, &self.other_palette, &self.other_mapping),
            ThemeMode::Light => (&self.other_palette, &self.other_mapping, &self.theme_palette, &self.theme_mapping),
        };
        Settings::from_app(
            self.theme_mode,
            dark_palette,
            dark_mapping,
            light_palette,
            light_mapping,
            self.base_text_size,
            &self.grid,
        )
        .save();
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
