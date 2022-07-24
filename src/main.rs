use macroquad::prelude::*;

struct Bezier {
    points: Vec<Vec2>,
}

impl Bezier {
    fn draw(&mut self) {
        if is_key_down(KeyCode::L) {
            info!("{}", self.points[1].x);
            if let Some(vec2) = self.points.get_mut(1) {
                *vec2 = Vec2::new(vec2.x + 0.1, vec2.y);
                vec2.x += 0.1;
            }
        }
        self.points.iter().enumerate().for_each(|(i, v)| {
            let camera_scale = (screen_width() / 2.).max(screen_height() / 2.);
            let mouse_relative_x = (mouse_position().0 - screen_width() / 2. - 2.5) / camera_scale;
            let mouse_relative_y = (screen_height() / 2. - mouse_position().1 + 2.5) / camera_scale;
            if Circle::new(v.x, v.y, 0.02).contains(&vec2(mouse_relative_x, mouse_relative_y)) {
                info!("intersect {}", i);
            };
            draw_circle_lines(v.x, v.y, 0.02, 0.006, WHITE);
        });

        self.points.windows(2).for_each(|v| {
            let t = (get_time() % 2.) as f32 / 2.;
            draw_line(v[0].x, v[0].y, v[1].x, v[1].y, 0.001, WHITE);
            draw_circle_lines(
                v[0].x - (v[0].x - v[1].x) * t,
                v[0].y - (v[0].y - v[1].y) * t,
                0.015,
                0.006,
                BLUE,
            );
        });

        self.points.windows(3).for_each(|v| {
            let t = (get_time() % 2.) as f32 / 2.;
            let x1 = v[0].x - (v[0].x - v[1].x) * t;
            let y1 = v[0].y - (v[0].y - v[1].y) * t;
            let x2 = v[1].x - (v[1].x - v[2].x) * t;
            let y2 = v[1].y - (v[1].y - v[2].y) * t;
            draw_line(x1, y1, x2, y2, 0.001, BLUE);

            let pos_x = x1 - (x1 - x2) * t;
            let pos_y = y1 - (y1 - y2) * t;

            draw_circle_lines(pos_x, pos_y, 0.015, 0.006, WHITE);

            let x1 = |t: f32| v[0].x - (v[0].x - v[1].x) * t;
            let y1 = |t: f32| v[0].y - (v[0].y - v[1].y) * t;
            let x2 = |t: f32| v[1].x - (v[1].x - v[2].x) * t;
            let y2 = |t: f32| v[1].y - (v[1].y - v[2].y) * t;

            let segments = 30;
            for i in 0..segments {
                let t = i as f32 / segments as f32;
                let t1 = (i as f32 + 1.) / segments as f32;
                if false {
                    draw_line(
                        x1(t) - (x1(t) - x2(t)) * t,
                        y1(t) - (y1(t) - y2(t)) * t,
                        x1(t1) - (x1(t1) - x2(t1)) * t1,
                        y1(t1) - (y1(t1) - y2(t1)) * t1,
                        0.008,
                        BLUE,
                    );
                }

                draw_circle(
                    x1(t) - (x1(t) - x2(t)) * t,
                    y1(t) - (y1(t) - y2(t)) * t,
                    0.01,
                    RED,
                );
            }
        })
    }
}

#[macroquad::main("BezierStudy")]
async fn main() {
    loop {
        let mut bezier = Bezier {
            points: vec![vec2(0.4, -0.3), vec2(0.1, 0.3), vec2(-0.4, -0.3)],
        };
        let zoom = 1.;

        draw_checkerboard();

        set_camera(&Camera2D {
            zoom: vec2(zoom, screen_width() / screen_height()),
            ..Default::default()
        });
        bezier.draw();

        //draw_circle(
        //(mouse_position().0 - screen_width() / 2. - 2.5) / camera_scale,
        //(screen_height() / 2. - mouse_position().1 + 2.5) / camera_scale,
        //0.01,
        //WHITE,
        //);

        //points.iter().for_each(|p| {
        //draw_circle(p.x, p.y, 0.01, WHITE);
        //});

        //if is_mouse_button_pressed(MouseButton::Left) {
        //points.push(vec2(
        //(mouse_position().0 - screen_width() / 2. - 2.5) / camera_scale,
        //(screen_height() / 2. - mouse_position().1 + 2.5) / camera_scale,
        //));
        //}

        set_default_camera();

        next_frame().await
    }
}

pub fn draw_checkerboard() {
    for i in 0..=(screen_width() / 20.) as u32 {
        for j in 0..=(screen_height() / 20.) as u32 {
            draw_rectangle(
                i as f32 * 20. - 10.,
                j as f32 * 20. - 10.,
                20.,
                20.,
                match (i + j) % 2 {
                    0 => Color::from_rgba(43, 46, 51, 255),
                    _ => Color::from_rgba(59, 62, 67, 255),
                },
            )
        }
    }
}
