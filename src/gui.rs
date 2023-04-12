use crate::breakpoints::Breakpoints;
use crate::disassembler::Disassembler;
use crate::watchpoints::Watchpoints;
use crate::z80::Z80;
use egui::{ClippedPrimitive, Context, TexturesDelta};
use egui_memory_editor::MemoryEditor;
use egui_wgpu::renderer::{Renderer, ScreenDescriptor};
use egui_winit::winit::event_loop::EventLoopWindowTarget;
use egui_winit::winit::window::Window;
use pixels::{wgpu, PixelsContext};

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: Context,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    renderer: Renderer,
    paint_jobs: Vec<ClippedPrimitive>,
    textures: TexturesDelta,

    // State for the GUI
    gui: Gui,
}

/// Example application state. A real application will need a lot more state than this.
struct Gui {
    /// Only show the egui window when true.
    mem_editor: MemoryEditor,
    mem_editor_open: bool,
    tvram_editor: MemoryEditor,
    tvram_editor_open: bool,
    breakpoints_open: bool,
    disassembler_open: bool,
    watchpoints_open: bool,
    controls_open: bool,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        width: u32,
        height: u32,
        scale_factor: f32,
        pixels: &pixels::Pixels,
    ) -> Self {
        let max_texture_size = pixels.device().limits().max_texture_dimension_2d as usize;

        let egui_ctx = Context::default();
        let mut egui_state = egui_winit::State::new(event_loop);
        egui_state.set_max_texture_side(max_texture_size);
        egui_state.set_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [width, height],
            pixels_per_point: scale_factor,
        };
        let renderer = Renderer::new(pixels.device(), pixels.render_texture_format(), None, 1);
        let textures = TexturesDelta::default();
        let gui = Gui::new();

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            renderer,
            paint_jobs: Vec::new(),
            textures,
            gui,
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &egui_winit::winit::event::WindowEvent) {
        let _ = self.egui_state.on_event(&self.egui_ctx, event);
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.size_in_pixels = [width, height];
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f64) {
        self.screen_descriptor.pixels_per_point = scale_factor as f32;
    }

    /// Prepare egui.
    pub(crate) fn prepare(
        &mut self,
        window: &Window,
        cpu: &mut Z80<crate::IO>,
        disassembler: &Disassembler,
        breakpoints: &mut Breakpoints,
        watchpoints: &mut Watchpoints,
    ) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let output = self.egui_ctx.run(raw_input, |egui_ctx| {
            // Draw the demo application.
            egui::TopBottomPanel::top("menubar_container").show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    self.gui.ui(egui_ctx, ui, cpu, disassembler, breakpoints, watchpoints);
                    cpu.io.video.ui(egui_ctx, ui);
                    cpu.io.fdc.ui(egui_ctx, ui);
                });
            });
        });

        self.textures.append(output.textures_delta);
        self.egui_state
            .handle_platform_output(window, &self.egui_ctx, output.platform_output);
        self.paint_jobs = self.egui_ctx.tessellate(output.shapes);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) {
        // Upload all resources to the GPU.
        for (id, image_delta) in &self.textures.set {
            self.renderer
                .update_texture(&context.device, &context.queue, *id, image_delta);
        }
        self.renderer.update_buffers(
            &context.device,
            &context.queue,
            encoder,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Render egui with WGPU
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: render_target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.renderer
                .render(&mut rpass, &self.paint_jobs, &self.screen_descriptor);
        }

        // Cleanup
        let textures = std::mem::take(&mut self.textures);
        for id in &textures.free {
            self.renderer.free_texture(id);
        }
    }
}

impl Gui {
    /// Create a `Gui`.
    fn new() -> Self {
        Self {
            mem_editor: MemoryEditor::new()
                .with_address_range("All", 0..0xFFFF)
                .with_window_title("Memory Editor"),
            mem_editor_open: false,
            tvram_editor: MemoryEditor::new()
                .with_address_range("All", 0..0x800)
                .with_window_title("TVRAM Editor"),
            tvram_editor_open: false,
            breakpoints_open: true,
            disassembler_open: true,
            watchpoints_open: true,
            controls_open: true,
        }
    }

    /// Create the UI using egui.
    fn ui(
        &mut self, 
        ctx: &Context,
        ui: &mut egui::Ui,
        cpu: &mut Z80<crate::IO>, 
        disassembler: &Disassembler, 
        breakpoints: &mut Breakpoints,
        watchpoints: &mut Watchpoints,
    ) {
        ui.menu_button("Tools", |ui| {
            if ui.button("Memory Editor").clicked() {
                self.mem_editor_open = true;
                ui.close_menu();
            };
            if ui.button("TVRAM Editor").clicked() {
                self.tvram_editor_open = true;
                ui.close_menu();
            };
            if ui.button("Disassembly").clicked() {
                self.disassembler_open = true;
                ui.close_menu();
            };
            if ui.button("Breakpoints").clicked() {
                self.breakpoints_open = true;
                ui.close_menu();
            };
            if ui.button("Watchpoints").clicked() {
                self.watchpoints_open = true;
                ui.close_menu();
            };
            if ui.button("Controls").clicked() {
                self.controls_open = true;
                ui.close_menu();
            };
        });

        self.mem_editor.window_ui(
            ctx,
            &mut self.mem_editor_open,
            &mut cpu.io.mem,
            |mem, address| if address < 0x10000 {Some(mem[address])} else {None},
            |mem, address, val| (mem[address] = val),
        );

        self.tvram_editor.window_ui(
            ctx,
            &mut self.tvram_editor_open,
            &mut cpu.io.video.tvram,
            |mem, address| if address < 0x800 {Some(mem[address])} else {None},
            |mem, address, val| {mem[address] = val},
        );

        egui::Window::new("Disassembly")
            .open(&mut self.disassembler_open)
            .show(ctx, |ui| {
                disassembler.display(ui, cpu);
            });

        egui::Window::new("Breakpoints")
            .open(&mut self.breakpoints_open)
            .show(ctx, |ui| {
                breakpoints.display(ui);
            });

        egui::Window::new("Watchpoints")
            .open(&mut self.watchpoints_open)
            .show(ctx, |ui| {
                watchpoints.display(ui);
            });

        egui::Window::new("Controls")
            .open(&mut self.watchpoints_open)
            .show(ctx, |ui| {
                if ui.button("Pause").clicked() {
                    cpu.io.pause_pressed = true;
                }
                if ui.button("Step").clicked() {
                    cpu.io.step_pressed = true;
                }
            });
    }
}
