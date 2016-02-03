use piston::input::*;
use opengl_graphics::GlGraphics;
use object::*;

pub struct App {
    pub gl: GlGraphics, // OpenGL drawing backend.
    pub tiles: Vec<Land>,
    pub player: Player,
    pub velocity: f64,
}

impl App {
    fn generate_tiles(&mut self, width: f64, height: f64) {
        // TODO: Update Height Info by resize.... TT
        let mut offset: f64 = 0.0;
        let tile_num = self.tiles.len();
        if tile_num != 0 {
            let lastTile = &self.tiles[tile_num - 1].get_render_info();
            offset = lastTile.offset.0 + lastTile.size.0;
        }

        while offset < width {
            // TODO: Random Enum Type 
            self.tiles.push(Land::new(offset, height, LandType::small_));
            offset += 50.0;
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        self.generate_tiles(args.width as f64, args.height as f64);

        // TODO: Render Player && PointObj
        let tiles = &self.tiles;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            for tile in tiles {
                let tile = tile.get_render_info().get_rectangle_info();
                //let transform = c.transform.trans(x, y).trans(-25.0, -25.0);
                rectangle(RED, tile, c.transform, gl);
            }

            // Draw a box rotating around the middle of the screen.
            //rectangle(RED, square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let offset_diff = self.velocity * args.dt;
        let position = self.tiles.iter_mut()
            .map(|x| x.update_render_info(offset_diff))
            .rposition(|v| v < 0.0);

        if position.is_some() {
            println!("TILE IS REMOVED! {}", position.unwrap());
            self.tiles = self.tiles.split_off(position.unwrap() + 1);
        }
    }
}

