#![allow(mixed_script_confusables)]

use nannou::prelude::*;
mod circ_harmonics;
mod spherical_harmonics;

use circ_harmonics::*;
use spherical_harmonics::SphericalHarmonics;

use std::f32::consts::PI;
const TWOPI: f32 = PI * 2.0;
// -------------------------------------------------------

struct Model {
    _window: window::Id,
    frame_count: usize,
    recording: bool,
    playing: bool,
    scene: usize,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        //.size(1000, 1200)
        //.size(2000, 900)
        //.size(1280, 900)
        .size(2900, 1300)
        .view(view)
        .build()
        .unwrap();

    Model {
        _window,
        frame_count: 0,
        recording: false,
        playing: false,
        scene: 1,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let num_keys = [
        (Key::Key1, 1),
        (Key::Key2, 2),
        (Key::Key3, 3),
        (Key::Key4, 4),
        (Key::Key5, 5),
        (Key::Key6, 6),
        (Key::Key7, 7),
        (Key::Key8, 8),
        (Key::Key9, 9),
    ];

    for k in num_keys {
        if app.keys.down.get(&k.0).is_some() {
            model.scene = k.1;
        }
    }

    // Rewind
    if app.keys.down.get(&Key::W).is_some() {
        model.recording = false;
        model.playing = false;
        model.frame_count = 0;
    }

    // Play
    if app.keys.down.get(&Key::P).is_some() {
        model.playing = true;
    }

    // Record
    if app.keys.down.get(&Key::R).is_some() {
        model.recording = true;
        model.playing = true;
    }

    if model.playing {
        model.frame_count += 1;
    }
}

struct Context {
    zoom: f32,
    wrap: f32,
    ch_stroke_width: f32,
    grid_stroke_width: f32,
    angle_multiplier: f32,
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let wrap = 0.5
        + ((model.frame_count + 500) as f32 * (1.0 / 60.0) * 0.5)
            .sin()
            .max(-0.8)
            .min(0.8)
            / 1.6;

    let ease_in_out_sine = |x: f32| -((PI * x).cos() - 1.0) * 0.5;

    let wrap = ease_in_out_sine(wrap);

    let radial_to_cartesian = |theta: f32, d: f32, ctx: &Context| -> (f32, f32) {
        let (s, c) = theta.sin_cos();
        let scaled_d = d * 1.5;
        let radial_p = (scaled_d * c, scaled_d * s);
        let cartesian_p = (theta - PI, d * 2.0);

        (
            (radial_p.0 * (1.0 - ctx.wrap) + cartesian_p.0 * ctx.wrap) * ctx.zoom,
            (radial_p.1 * (1.0 - ctx.wrap) + cartesian_p.1 * ctx.wrap) * ctx.zoom - 100.0,
        )
    };

    //let ch = ch_impulse.rotate(app.time * 0.3);

    let draw_ch = |ch: &CircularHarmonics, positive: bool, posx: f32, posy: f32, ctx: &Context| {
        let get_ch_points = |ch: &CircularHarmonics| {
            let point_count = 1000;
            let points: Vec<Vec2> = (0..=point_count)
                .map(|i| {
                    let angle = i as f32 / point_count as f32 * 2.0 * std::f32::consts::PI;
                    let d = ch.evaluate(angle * ctx.angle_multiplier);

                    let (x, y) = radial_to_cartesian(angle, d, ctx);
                    Vec2::new(posx + x, posy + y)
                })
                .collect();
            points
        };

        let color = if positive {
            rgba(0.9, 0.7, 0.2, 1.0)
        } else {
            rgba(0.2, 0.4, 0.8, 1.0)
        };
        draw.polyline()
            .color(color)
            .stroke_weight(ctx.ch_stroke_width)
            .points(get_ch_points(ch));
    };

    let grid_size = 10;
    let grid_subdivs = 15;

    let grid_color = Srgb::<u8>::new(200, 200, 200);

    let draw_grid_row = |row: usize, ctx: &Context, offsetx: f32, offsety: f32| {
        draw.polyline()
            .color(grid_color)
            .stroke_weight(ctx.grid_stroke_width)
            .x_y(0.0, 0.0)
            .points(
                (0..grid_size * grid_subdivs)
                    .map(|col| {
                        Point2::new(
                            col as f32 / (grid_size * grid_subdivs - 1) as f32 * TWOPI,
                            row as f32 / grid_size as f32,
                        )
                    })
                    .map(|p| {
                        let (x, y) = radial_to_cartesian(p.x, p.y, ctx);
                        Point2::new(x + offsetx, y + offsety)
                    })
                    .collect::<Vec<Point2>>(),
            )
    };

    let draw_grid_col = |col: usize, ctx: &Context, offsetx: f32, offsety: f32| {
        draw.polyline()
            .color(grid_color)
            .stroke_weight(ctx.grid_stroke_width)
            .x_y(0.0, 0.0)
            .points(
                (0..grid_size * grid_subdivs)
                    .map(|row| {
                        Point2::new(
                            col as f32 / grid_size as f32 * TWOPI,
                            row as f32 / (grid_size * grid_subdivs - 1) as f32,
                        )
                    })
                    .map(|p| {
                        let (x, y) = radial_to_cartesian(p.x, p.y, ctx);
                        Point2::new(x + offsetx, y + offsety)
                    })
                    .collect::<Vec<Point2>>(),
            )
    };

    match model.scene {
        // Shape of the first 6 bands
        1 => {
            let ctx = Context {
                zoom: 150.0,
                wrap,
                ch_stroke_width: 4.0,
                grid_stroke_width: 3.0,
                angle_multiplier: 1.0,
            };
            let bands = 6;
            let get_coeffs = |i: usize, v: f32| -> Vec<f32> {
                (0..(bands * 2 - 1))
                    .map(|c| if c == i { v } else { 0.0 })
                    .collect()
            };

            for coef in 0..(bands * 2 - 1) {
                let band = (coef + 1) / 2;
                let x = 400.0 * band as f32 - 800.0;

                for v in [1, -1] {
                    let fv = v as f32;
                    let ch_coeffs = get_coeffs(coef, fv);
                    let ch = CircularHarmonics::from_coeffs(ch_coeffs);
                    draw_ch(&ch, v == 1, x, 400.0 * (coef % 2) as f32, &ctx);
                }

                if coef % 2 == 0 {
                    draw.text(&format!("Band: {}", band))
                        .color(BLACK)
                        //.font(model.font.clone())
                        .font_size(50)
                        .x_y(x, -280.0);
                }
            }
        }

        // Impulse CH
        2 => {
            let ctx = Context {
                zoom: 200.0,
                wrap,
                ch_stroke_width: 4.0,
                grid_stroke_width: 3.0,
                angle_multiplier: 1.0,
            };

            for bands in 2..=5 {
                let ch_impulse = CircularHarmonics::from_impulse(bands, PI, 0.7);
                let offsetx = bands as f32 * 700.0 - 2300.0;
                for i in 0..=grid_size {
                    draw_grid_row(i, &ctx, offsetx, 0.0);
                    draw_grid_col(i, &ctx, offsetx, 0.0);
                }
                draw_ch(&ch_impulse, true, offsetx, 0.0, &ctx);
                draw.text(&format!("Bands: {}", bands))
                    .color(BLACK)
                    //.font(model.font.clone())
                    .font_size(50)
                    .x_y(offsetx, -280.0);
            }
        }

        // Box function CH
        3 => {
            let ctx = Context {
                zoom: 200.0,
                wrap: 0.0,
                ch_stroke_width: 7.0,
                grid_stroke_width: 5.0,
                angle_multiplier: 1.0,
            };

            let bands_opts = [2, 4, 6, 8, 12, 16, 32, 64];
            let angle = PI * 0.6 + 0.5 * PI * (model.frame_count as f32 * 0.02).sin();
            for (i, bands) in bands_opts.iter().enumerate() {
                let ch_pulse = CircularHarmonics::from_pulse(*bands, angle, 1.0).rotate(PI * 0.5);
                let offsetx = (i % 4) as f32 * 700.0 - 1050.0;
                let offsety = 500.0 - (i / 4) as f32 * 800.0;

                for i in 0..=grid_size {
                    draw_grid_row(i, &ctx, offsetx, offsety);
                    draw_grid_col(i, &ctx, offsetx, offsety);
                }
                draw_ch(&ch_pulse, true, offsetx, offsety, &ctx);
                draw.text(&format!("Bands: {}", bands))
                    .color(BLACK)
                    //.font(model.font.clone())
                    .font_size(55)
                    .no_line_wrap()
                    .x_y(offsetx, offsety - 450.0);
            }
            draw.text(&format!("θ = {:.2}", angle))
                .color(BLACK)
                //.font(model.font.clone())
                .font_size(50)
                .x_y(0.0, 0.0);
        }

        4 => {
            let w = 80;
            let h = 80;
            let radius = 600.0;

            let x_y_to_sphere = |sh: &SphericalHarmonics, x: i32, y: i32| -> Vec3 {
                let fx = x as f32 / (w as f32 - 1.0);
                let fy = y as f32 / (h as f32 - 1.0);

                let θ = fx * TWOPI;
                let φ = (fy - 0.5) * PI;

                let v = pt3(φ.sin() * θ.cos(), φ.sin() * θ.sin(), φ.cos());

                let dist = sh.evaluate(v);
                let r = radius * dist;

                v * r
            };

            let render_sh =
                |sh: &SphericalHarmonics, band: usize, xoffset: f32, yoffset: f32, color: Vec3| {
                    let tris = (0..w * h)
                        .flat_map(|i| {
                            let x = i % w;
                            let y = i / w;

                            let p0 = x_y_to_sphere(sh, x, y);
                            let p1 = x_y_to_sphere(sh, x + 1, y);
                            let p2 = x_y_to_sphere(sh, x + 1, y + 1);
                            let p3 = x_y_to_sphere(sh, x, y + 1);

                            geom::Quad([p0, p1, p2, p3]).triangles_iter()
                        })
                        .map(|tri| {
                            let n = (tri.0[2] - tri.0[0]).cross(tri.0[2] - tri.0[1]).normalize();
                            let d = 0.2 + Vec3::new(0.3, 0.6, -0.4).dot(n).abs();
                            // Color the vertices based on their amplitude.
                            tri.map_vertices(|v| {
                                let color = srgba(d * color.x, d * color.y, d * color.z, 1.0);
                                (v.extend(0.0), color)
                            })
                        });

                    let scale = 1.0 + ((band / 2) as f32).pow(2.0) + ((band / 4) as f32) * 5.0;

                    draw.x_y(xoffset, yoffset)
                        .scale(scale * 0.20)
                        .pitch(model.frame_count as f32 / 60.0 * PI)
                        .yaw(model.frame_count as f32 / 60.0 * PI)
                        .mesh()
                        .tris_colored(tris);
                };

            let render_sh_term = |i: usize, xoffset: f32, yoffset: f32, band: usize| {
                let mut sh = spherical_harmonics::SphericalHarmonics::from_terms(4, vec![0.0; 26]);
                sh.terms[i] = 1.0;

                render_sh(&sh, band, xoffset, yoffset, Vec3::new(1.0, 0.6, 0.3));
                sh.terms[i] = -1.0;
                render_sh(&sh, band, xoffset, yoffset, Vec3::new(0.3, 0.6, 0.8));
            };

            let terms = vec![
                vec![0],
                vec![1, 2, 3],
                vec![4, 5, 6, 7, 8],
                vec![9, 10, 11, 12, 13, 14, 15],
                vec![16, 17, 18, 19, 20, 21, 22, 23, 24, 25],
            ];

            let w = 350.0;
            for (band, band_terms) in terms.iter().enumerate() {
                let yoffset = w * 2.0 - w * band as f32;
                for (band_term_index, term) in band_terms.iter().enumerate() {
                    let xoffset = -w * (band_terms.len() - 1) as f32 / 2.0
                        + w * band_term_index as f32
                        + (band / 4) as f32 * w * 0.5;

                    render_sh_term(*term, xoffset, yoffset, band);
                }
            }
        }
        5 => {
            let bands = 6;
            let get_coeffs = |i: usize, v: f32| -> Vec<f32> {
                (0..(bands * 2 - 1))
                    .map(|c| if c == i { v } else { 0.0 })
                    .collect()
            };

            for coef in 1..(bands * 2 - 1) {
                let band = (coef + 1) / 2;
                let x = 400.0 * band as f32 - 1200.0;

                let lerp = |a, b, x| b * x + a * (1.0 - x);

                let ctx = Context {
                    zoom: 150.0,
                    wrap: 0.0,
                    ch_stroke_width: 4.0,
                    grid_stroke_width: 3.0,
                    angle_multiplier: lerp(1.0, 1.0 / band as f32, wrap),
                };

                for v in [1, -1] {
                    let fv = v as f32;
                    let ch_coeffs = get_coeffs(coef, fv);
                    let ch = CircularHarmonics::from_coeffs(ch_coeffs);
                    draw_ch(&ch, v == 1, x, 400.0 * (coef % 2) as f32, &ctx);
                }

                if coef % 2 == 0 {
                    draw.text(&format!("Band: {}", band))
                        .color(BLACK)
                        //.font(model.font.clone())
                        .font_size(50)
                        .x_y(x, -280.0);

                    draw.text(&format!("Scale: {:0.3}", 1.0 / ctx.angle_multiplier))
                        .color(BLACK)
                        .font_size(50)
                        .no_line_wrap()
                        .x_y(x, -350.0);
                }
            }
        }

        // CH Rotation
        6 => {
            let ctx = Context {
                zoom: 200.0,
                wrap: 0.0,
                ch_stroke_width: 3.0,
                grid_stroke_width: 2.0,
                angle_multiplier: 1.0,
            };

            let angle = PI * 0.1 + 0.9 * PI * wrap;
            let ch_pulse = CircularHarmonics::from_pulse(20, angle, 1.0).rotate(PI * 0.5);
            let rotation = wrap * TWOPI;
            let ch_pulse = ch_pulse.rotate(rotation);
            let offsetx = 0.0;
            let offsety = 180.0;

            for i in 0..=grid_size {
                draw_grid_row(i, &ctx, offsetx, offsety);
                draw_grid_col(i, &ctx, offsetx, offsety);
            }
            draw_ch(&ch_pulse, true, offsetx, offsety, &ctx);

            draw.text(&format!("θ = {:.2}, φ = {:.2}", angle, rotation))
                .color(BLACK)
                //.font(model.font.clone())
                .font_size(50)
                .x_y(0.0, offsety - 500.0);
        }

        // CH Composition (Add/Sub/Mul)
        7 => {
            let ctx = Context {
                zoom: 200.0,
                wrap: 0.0,
                ch_stroke_width: 4.0,
                grid_stroke_width: 3.0,
                angle_multiplier: 1.0,
            };

            let angle = model.frame_count as f32 * TWOPI / 300.0;
            //let ch_pulse1 = CircularHarmonics::from_pulse(20, 2.0*PI, 1.0).rotate(PI * 0.5);
            let ch_pulse1 = CircularHarmonics::from_pulse(20, PI, 1.0).rotate(PI * 0.5);
            let ch_pulse2 = CircularHarmonics::from_pulse(20, PI * 0.25, 1.0).rotate(angle);

            let ch_pulse = &ch_pulse1 + &ch_pulse2;
            //let ch_pulse = &ch_pulse1 - &ch_pulse2;
            // let ch_pulse = &ch_pulse2.convolve(&ch_pulse1);

            let offsetx = -250.0;

            let draw_ch_and_grid =
                |ctx: &Context, ch: &CircularHarmonics, offsetx: f32, offsety: f32| {
                    for i in 0..=grid_size {
                        draw_grid_row(i, ctx, offsetx, offsety);
                        draw_grid_col(i, ctx, offsetx, offsety);
                    }
                    draw_ch(ch, true, offsetx, offsety, &ctx);
                };

            draw_ch_and_grid(&ctx, &ch_pulse1, offsetx - 800.0, 0.0);
            draw_ch_and_grid(&ctx, &ch_pulse2, offsetx, 0.0);
            draw_ch_and_grid(&ctx, &ch_pulse, offsetx + 1000.0, 0.0);

            //draw.text("+")
            draw.text("-")
                .color(BLACK)
                //.font(model.font.clone())
                .font_size(130)
                .x_y(-650.0, -80.0);

            draw.text("=")
                .color(BLACK)
                //.font(model.font.clone())
                .font_size(130)
                .x_y(250.0, -80.0);
        }

        // Fourier unwrapping
        9 => {
            let ctx = Context {
                zoom: 150.0,
                wrap,
                ch_stroke_width: 3.0,
                grid_stroke_width: 2.0,
                angle_multiplier: 1.0,
            };
            let bands = 19;
            let ch_impulse = CircularHarmonics::from_impulse(bands, PI, 0.2);
            let ch_pulse = CircularHarmonics::from_pulse(20, PI, 1.0).rotate(PI * 0.5);

            for i in 0..=grid_size {
                draw_grid_row(i, &ctx, 0.0, 300.0);
                draw_grid_col(i, &ctx, 0.0, 300.0);
            }
            draw_ch(&ch_impulse, true, 0.0, 300.0, &ctx);

            for i in 0..=grid_size {
                draw_grid_row(i, &ctx, 0.0, -250.0);
                draw_grid_col(i, &ctx, 0.0, -250.0);
            }
            draw_ch(&ch_pulse, true, 0.0, -250.0, &ctx);
        }

        _ => {}
    }

    if model.recording {
        if model.frame_count % 2 == 0 {
            let filename = format!("capture_{}.png", model.frame_count / 2);
            app.main_window().capture_frame(&filename);
            println!("Captured image to {}", filename);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
