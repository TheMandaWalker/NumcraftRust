use crate::{
    constants::{ItemType, color_palette::GAMEUI_SLOT_COLOR},
    nadk::display::ScreenRect,
    hud::Hud,
    renderer::{misc::UnBoundedScreenRect, *},
};

impl Renderer {
    pub fn draw_hud(&mut self, hud: &Hud, frame_time: u64, tile_x: usize, tile_y: usize) {
        if tile_x == 0 && tile_y == 0 {
            if hud.show_debug {
                self.draw_string(format!("f:{frame_time}").as_str(), &Vector2::new(2, 2));

                self.draw_string(
                    format!("t:{}", self.triangles_to_render.len()).as_str(),
                    &Vector2::new(2, 22),
                );

                /*self.draw_string(
                    format!(
                        "{:.1},{:.1},{:.1}",
                        self.camera.get_pos().x,
                        self.camera.get_pos().y,
                        self.camera.get_pos().z
                    )
                    .as_str(),
                    &Vector2::new(10, 50),
                );*/
            }
        }

        let mut draw_cross = |x, y| {
            self.draw_image_negate(
                CROSS_DATA,
                Vector2::new(CROSS_WIDTH as isize, CROSS_HEIGHT as isize),
                Vector2::new(x, y),
            );
        };

        if tile_x == 1 && tile_y == 1 {
            draw_cross(
                (SCREEN_TILE_WIDTH - CROSS_WIDTH / 2) as isize,
                (SCREEN_TILE_HEIGHT - CROSS_HEIGHT / 2) as isize,
            )
        }
        if tile_x == 2 && tile_y == 1 {
            draw_cross(
                -((CROSS_WIDTH / 2) as isize),
                (SCREEN_TILE_HEIGHT - CROSS_HEIGHT / 2) as isize,
            )
        }
        if tile_x == 2 && tile_y == 2 {
            draw_cross(
                -((CROSS_WIDTH / 2) as isize),
                -((CROSS_HEIGHT / 2) as isize),
            );
        }
        if tile_x == 1 && tile_y == 2 {
            draw_cross(
                (SCREEN_TILE_WIDTH - CROSS_WIDTH / 2) as isize,
                -((CROSS_HEIGHT / 2) as isize),
            );

            //self.draw_slot_frame_buffer(Vector2::new(94, 85), hud, 1);
            //self.draw_slot_frame_buffer(Vector2::new(128, 85), hud, 2);
        }

        if tile_x == 0 && tile_y == 3 {
            self.draw_slot_frame_buffer(Vector2::new(60, 20), hud, 0);
        }

        if tile_x == 1 && tile_y == 3 {
            self.draw_slot_frame_buffer(Vector2::new(-20, 20), hud, 0);
            self.draw_slot_frame_buffer(Vector2::new(14, 20), hud, 1);
            self.draw_slot_frame_buffer(Vector2::new(48, 20), hud, 2);
        }
        if tile_x == 2 && tile_y == 3 {
            self.draw_slot_frame_buffer(Vector2::new(2, 20), hud, 3);
            self.draw_slot_frame_buffer(Vector2::new(36, 20), hud, 4);
        }

        if tile_x == 2 && tile_y == 3 {
            self.draw_slot_frame_buffer(Vector2::new(70, 20), hud, 5);
        }

        if tile_x == 3 && tile_y == 3 {
            self.draw_slot_frame_buffer(Vector2::new(-10, 20), hud, 5);
        }

        self.draw_breaking_indicator(tile_x, tile_y, hud);
        self.draw_survival_stats(tile_x, tile_y, hud);
    }

    /// Health (hearts) and hunger (drumsticks) rows, above the hotbar
    pub fn draw_survival_stats(&mut self, tile_x: usize, tile_y: usize, hud: &Hud) {
        if !hud.show_survival_stats || tile_y != 3 {
            return;
        }

        // Screen positions are moved into the tile, parts outside it are clipped
        let tile_origin_x = (tile_x * SCREEN_TILE_WIDTH) as isize;
        let tile_origin_y = (tile_y * SCREEN_TILE_HEIGHT) as isize;

        for icon in 0..10 {
            // One icon stands for 2 points of health or hunger
            let offset = icon as f32 * 2.;
            let icon_x = icon as isize * 11;

            self.draw_stat_icon(
                60 + icon_x - tile_origin_x,
                184 - tile_origin_y,
                hud.health - offset,
                Color565::from_rgb888(220, 40, 40),
            );
            self.draw_stat_icon(
                172 + icon_x - tile_origin_x,
                184 - tile_origin_y,
                hud.hunger - offset,
                Color565::from_rgb888(210, 140, 40),
            );
        }
    }

    fn draw_stat_icon(&mut self, x: isize, y: isize, value: f32, color: Color565) {
        let fill = (value / 2.).clamp(0., 1.);

        self.push_unbounded_rect_uniform_on_frame_buffer(
            UnBoundedScreenRect {
                x,
                y,
                width: 8,
                height: 8,
            },
            Color565::from_rgb888(60, 60, 60),
        );
        self.push_unbounded_rect_uniform_on_frame_buffer(
            UnBoundedScreenRect {
                x,
                y,
                width: (8. * fill) as isize,
                height: 8,
            },
            color,
        );
    }

    pub fn draw_breaking_indicator(&mut self, tile_x: usize, tile_y: usize, hud: &Hud) {
        if let Some(progress) = hud.breaking_progress {
            let bar_len = (40. * progress) as u16;
            if tile_x == 1 && tile_y == 2 {
                self.push_rect_uniform_on_frame_buffer(
                    ScreenRect {
                        x: 58,
                        y: 18,
                        width: 22,
                        height: 9,
                    },
                    Color565::from_rgb888(100, 100, 100),
                );
                self.push_rect_uniform_on_frame_buffer(
                    ScreenRect {
                        x: 60,
                        y: 20,
                        width: bar_len.min(20),
                        height: 5,
                    },
                    Color565::from_rgb888(200, 200, 200),
                );
            }
            if tile_x == 2 && tile_y == 2 {
                self.push_rect_uniform_on_frame_buffer(
                    ScreenRect {
                        x: 0,
                        y: 18,
                        width: 22,
                        height: 9,
                    },
                    Color565::from_rgb888(100, 100, 100),
                );
                if bar_len > 20 {
                    self.push_rect_uniform_on_frame_buffer(
                        ScreenRect {
                            x: 0,
                            y: 20,
                            width: (bar_len - 20),
                            height: 5,
                        },
                        Color565::from_rgb888(200, 200, 200),
                    );
                }
            }
        }
    }

    pub fn draw_image_negate(
        &mut self,
        image: &[u8],
        image_size: Vector2<isize>,
        pos: Vector2<isize>,
    ) {
        for y in 0..image_size.y {
            if pos.y + y < 0 || pos.y + y >= SCREEN_TILE_HEIGHT as isize {
                continue;
            }
            for x in 0..image_size.x {
                let dest = Vector2::new(x, y) + pos;

                if dest.x < 0 || dest.x >= SCREEN_TILE_WIDTH as isize {
                    continue;
                }

                let pixel = image[(x + image_size.x * y) as usize];

                if pixel == 0 {
                    let frame_buff_index = (dest.x + dest.y * SCREEN_TILE_WIDTH as isize) as usize;
                    let components = self.tile_frame_buffer[frame_buff_index].get_components();

                    let inverted_color = Color565::new(
                        0b11111 - components.0,
                        0b111111 - components.1,
                        0b11111 - components.2,
                    );
                    self.tile_frame_buffer[frame_buff_index] = inverted_color;
                }
            }
        }
    }

    fn draw_scalled_tile_on_frame_buffer(
        &mut self,
        texture_id: u8,
        pos: Vector2<isize>,
        scale: isize,
    ) {
        let tileset_x = (texture_id % 16) as usize * 8;
        let tileset_y = (texture_id / 16) as usize * 8;

        for x in 0..8 {
            for y in 0..8 {
                let texture_pixel_index = ((tileset_x + x) + (tileset_y + y) * 128) * 2;
                let pixel = u16::from_be_bytes([
                    TILESET_DATA[texture_pixel_index],
                    TILESET_DATA[texture_pixel_index + 1],
                ]);

                self.push_unbounded_rect_uniform_on_frame_buffer(
                    UnBoundedScreenRect {
                        x: pos.x + (x as isize * scale),
                        y: pos.y + (y as isize * scale),
                        width: scale,
                        height: scale,
                    },
                    Color565 { value: pixel },
                );
            }
        }
    }

    fn draw_slot_frame_buffer(&mut self, pos: Vector2<isize>, hud: &Hud, slot_index: usize) {
        self.push_unbounded_rect_uniform_on_frame_buffer(
            UnBoundedScreenRect {
                x: pos.x,
                y: pos.y,
                width: 30,
                height: 30,
            },
            if hud.selected_slot == slot_index {
                Color565::from_rgb888(200, 50, 50)
            } else {
                GAMEUI_SLOT_COLOR
            },
        );
        let item_stack = hud.get_slots()[slot_index];
        let texture_id = item_stack.get_item_type().get_texture_id();

        if texture_id != 0 {
            self.draw_scalled_tile_on_frame_buffer(texture_id, pos + Vector2::new(3, 3), 3);
        }

        let item_type = item_stack.get_item_type();
        let max_item_count = item_type.get_max_stack_amount();

        if item_type != ItemType::Air && item_type.get_max_stack_amount() > 1 {
            let item_bar_lenght = 24 * item_stack.get_amount() as isize / max_item_count as isize;
            self.push_unbounded_rect_uniform_on_frame_buffer(
                UnBoundedScreenRect {
                    x: pos.x + 3,
                    y: pos.y + 24,
                    width: item_bar_lenght,
                    height: 3,
                },
                Color565::from_rgb888(100, 150, 255),
            );
            self.push_unbounded_rect_uniform_on_frame_buffer(
                UnBoundedScreenRect {
                    x: pos.x + 3 + item_bar_lenght,
                    y: pos.y + 24,
                    width: 24 - item_bar_lenght,
                    height: 3,
                },
                Color565::from_rgb888(200, 200, 200),
            );
        }
    }
}
