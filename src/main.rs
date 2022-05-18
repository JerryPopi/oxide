mod tools;
mod macros;
use eframe::egui;
use egui::{RichText, TextStyle, Style, FontId, Visuals, SidePanel, Grid, Vec2, Layout, Rect, Stroke, Color32, Button, ImageButton, Sense, Image, Align, Separator, style::Spacing, Frame};
use egui_extras::RetainedImage;
use tools::{ToolIcons, Tools};

fn main() {
    let options = eframe::NativeOptions {
        // Hide the OS-specific "chrome" around the window:
        // decorated: false,
        // maximized: true,
        resizable: true,
        // To have rounded corners we need transparency:
        transparent: true,
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Oxide", // unused title
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

struct MyApp {
    tool_icons: ToolIcons,
    selected_tool: Tools,
    image: RetainedImage
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            tool_icons: ToolIcons::new(),
            selected_tool: Tools::None,
            image: RetainedImage::from_image_bytes("test_image.png", include_bytes!("test_image.png")).unwrap(),
        }
    }
}

impl eframe::App for MyApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> egui::Rgba {
        egui::Rgba::TRANSPARENT // Make sure we don't paint anything behind the rounded corners
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        custon_window_frame(ctx, frame, "egui with custom frame", |ui| {
            ui.centered_and_justified(|ui| {
                // let image = self.image.show(ui);
                let image = ui.add_sized(self.image.size_vec2(), Image::new(self.image.texture_id(ctx), self.image.size_vec2()));
                // let image = ui.image(self.image.texture_id(ctx), self.image.size_vec2());
                


                ui.painter().rect_stroke(image.rect, 10.0, Stroke::new(3.0, Color32::BLACK));
            });
            // ui.painter().rect_stroke(Rect::from_min_max(image.rect.min, image.rect.max), 10.0, Stroke::new(3.0, Color32::BLACK));

            SidePanel::left("tool_panel_left").min_width(20.0).resizable(false).show(ctx, |ui| {
                ui.add_space(5.0);
                ui.with_layout(Layout::top_down(Align::Min), |ui| {
                    // Frame::none().fill(Color32::RED).show(ui, |ui| {
                    //     ui.centered_and_justified(|ui| {
                    //         ui.add(Image::new(self.tool_icons.drag.texture_id(ctx), Vec2::splat(20.0)));
                    //     });
                    // });
                    if ui.add(ImageButton::new(self.tool_icons.drag.texture_id(ctx), Vec2::splat(20.0)).selected(if self.selected_tool == Tools::Drag {true} else {false})).clicked() {
                        self.selected_tool = Tools::Drag;
                    }
                    if ui.add(ImageButton::new(self.tool_icons.select.texture_id(ctx), [20.0, 17.0]).selected(if self.selected_tool == Tools::Select {true} else {false})).clicked() {
                        self.selected_tool = Tools::Select;
                    }
                });
            });
        });
    }
}

fn custon_window_frame(
    ctx: &egui::Context,
    frame: &mut eframe::Frame,
    title: &str,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    use egui::*;
    
    // let mut style: egui::Style = (*ctx.style()).clone();
    // let new_vis = Some(Visuals::dark());
    // // style.visuals = Visuals::dark();
    // // style.visuals.dark_mode = true;
    // if let Some(visuals) = new_vis {
    //     ctx.set_visuals(visuals);
    //     // ctx.set_style(style);
    // }

    CentralPanel::default()
        .frame(Frame::none())
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            let painter = ui.painter();
            // Paint the frame:
            painter.rect(
                rect.shrink(0.0),
                0.0,
                ctx.style().visuals.window_fill(),
                Stroke::none()
            );

            let mut content_ui = ui.child_ui(rect.shrink(5.0), *ui.layout());
            add_contents(&mut content_ui);
        });
}