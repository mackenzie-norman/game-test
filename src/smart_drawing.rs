use console_engine::pixel;
use console_engine::pixel::{ Pixel};
use console_engine::rect_style::BorderStyle;
use console_engine::{Color, MouseButton};
use console_engine::ConsoleEngine;

/// Draws a triangle of the provided character using three sets of coordinates
///
/// usage:
/// ```
/// use console_engine::pixel;
/// // ...
/// screen.triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
/// ```
#[allow(clippy::too_many_arguments)]
pub fn triangle(
    engine: &mut ConsoleEngine,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    character: Pixel,
) {
    line(engine,x1, y1, x2, y2, character);
    line(engine,x2, y2, x3, y3, character);
    line(engine, x3, y3, x1, y1, character);
}
 
/// Fill a triangle of the provided character using three sets of coordinates
/// see: [rustyPixelGameEngine Repository](https://github.com/mattbettcher/rustyPixelGameEngine)
///
/// usage:
/// ```
/// use console_engine::pixel;
/// // ...
/// screen.fill_triangle(8,8, 4,6, 9,2, pixel::pxl('#'));
/// ```
#[allow(clippy::too_many_arguments)]
pub fn fill_triangle(
    engine: &mut ConsoleEngine,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    character: Pixel,
) {
    triangle(engine,x1, y1, x2, y2, x3, y3, character);
    // we use tuples for this for now
    let v0 = (x1, y1);
    let mut v1 = (x2, y2);
    let mut v2 = (x3, y3);

    // algorithm only fills counter clockwise triangles, so swap as needed
    // For a triangle A B C, you can find the winding by computing the cross product (B - A) x (C - A). For 2d tri's, with z=0, it will only have a z component.
    // To give all the same winding, swap vertices C and B if this z component is negative.
    let cross = (v1.1 - v0.1) * (v2.0 - v1.0) - (v1.0 - v0.0) * (v2.1 - v1.1);
    if cross > 0 {
        std::mem::swap(&mut v1, &mut v2)
    }

    // Compute triangle bounding box and clip to screen bounds
    let min_x = std::cmp::max(std::cmp::min(std::cmp::min(v0.0, v1.0), v2.0), 0);
    let max_x = std::cmp::min(
        std::cmp::max(std::cmp::max(v0.0, v1.0), v2.0),
        engine.get_width() as i32 - 1,
    );
    let min_y = std::cmp::max(std::cmp::min(std::cmp::min(v0.1, v1.1), v2.1), 0);
    let max_y = std::cmp::min(
        std::cmp::max(std::cmp::max(v0.1, v1.1), v2.1),
        engine.get_height() as i32 - 1,
    );

    // Triangle setup
    let a01 = v0.1 - v1.1;
    let b01 = v1.0 - v0.0;
    let a12 = v1.1 - v2.1;
    let b12 = v2.0 - v1.0;
    let a20 = v2.1 - v0.1;
    let b20 = v0.0 - v2.0;

    // Determine edges
    let is_top_left = |v0: (i32, i32), v1: (i32, i32)| -> bool { v0.1 > v1.1 };

    // We follow fill rules and add a bias
    let bias0 = if is_top_left(v1, v2) { 0 } else { -1 };
    let bias1 = if is_top_left(v2, v0) { 0 } else { -1 };
    let bias2 = if is_top_left(v0, v1) { 0 } else { -1 };

    // Determine barycentric coordinates
    let orient2d = |a: (i32, i32), b: (i32, i32), c: (i32, i32)| -> i32 {
        (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
    };

    let mut p = (min_x, min_y);
    let mut w0_row = orient2d(v1, v2, p) + bias0;
    let mut w1_row = orient2d(v2, v0, p) + bias1;
    let mut w2_row = orient2d(v0, v1, p) + bias2;

    // Rasterize
    for y in min_y..max_y {
        p.1 = y;
        // Barycentric coordinates at start of row
        let mut w0 = w0_row;
        let mut w1 = w1_row;
        let mut w2 = w2_row;

        for x in min_x..max_x {
            p.0 = x;
            // If p is on or inside all edges, render pixel.
            if (w0 | w1 | w2) >= 0 {
                smart_set_pxl(engine,p.0, p.1, character);
            }

            // One step to the right
            w0 += a12;
            w1 += a20;
            w2 += a01;
        }
        // One row step
        w0_row += b12;
        w1_row += b20;
        w2_row += b01;
    }
}
/**/

pub fn h_line(engine: &mut ConsoleEngine, start_x: i32, start_y: i32, end_x: i32, character: Pixel) {
        let start = if start_x > end_x { end_x } else { start_x };
        let end = if start_x > end_x {
            start_x + 1
        } else {
            end_x + 1
        };
        for i in start..end {
            smart_set_pxl(engine,i, start_y, character);
        }
    }

    /// Optimized vertical line drawing
    /// Automatically called by [line](#method.line) if needed
    pub fn v_line(engine: &mut ConsoleEngine, start_x: i32, start_y: i32, end_y: i32, character: Pixel) {
        let start = if start_y > end_y { end_y } else { start_y };
        let end = if start_y > end_y {
            start_y + 1
        } else {
            end_y + 1
        };
        for j in start..end {
            smart_set_pxl(engine,start_x, j, character);
        }
    }
/// draws a line of the provided character between two sets of coordinates  
/// see: [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
///
/// Note : Your line can start or end out of bounds. These pixels won't be drawn
///
/// usage:
/// ```
/// use console_engine::pixel;
/// // ...
/// screen.line(0, 0, 9, 9, pixel::pxl('#'));
/// ```
pub fn line(engine: &mut ConsoleEngine, start_x: i32, start_y: i32, end_x: i32, end_y: i32, character: Pixel) {
    let delta_x = end_x - start_x;
    let delta_y = end_y - start_y;
    // use optimized functions for pure horizontal or vertical lines
    if delta_y == 0 {
        h_line(engine, start_x, start_y, end_x, character);
        return;
    }
    if delta_x == 0 {
        v_line(engine,start_x, start_y, end_y, character);
        return;
    }

    // Bresenham's line algorithm
    let mut line_low = |engine: &mut ConsoleEngine ,x0: i32, y0: i32, x1: i32, y1: i32| {
        let dx: i32 = x1 - x0;
        let mut dy: i32 = y1 - y0;
        let mut yi = 1;
        if dy < 0 {
            yi = -1;
            dy = -dy;
        }
        let mut d = 2 * dy - dx;
        let mut y = y0;

        for x in x0..x1 + 1 {
            smart_set_pxl(engine,x, y, character);
            if d > 0 {
                y += yi;
                d -= 2 * dx;
            }
            d += 2 * dy;
        }
    };

    let mut line_high = |engine: &mut ConsoleEngine, x0: i32, y0: i32, x1: i32, y1: i32| {
        let mut dx = x1 - x0;
        let dy = y1 - y0;
        let mut xi = 1;
        if dx < 0 {
            xi = -1;
            dx = -dx;
        }
        let mut d = 2 * dx - dy;
        let mut x = x0;

        for y in y0..y1 + 1 {
            smart_set_pxl(engine,x, y, character);
            if d > 0 {
                x += xi;
                d -= 2 * dy;
            }
            d += 2 * dx;
        }
    };

    if (end_y - start_y).abs() < (end_x - start_x).abs() {
        if start_x > end_x {
            line_low( engine,end_x, end_y, start_x, start_y);
        } else {
            line_low(  engine,start_x, start_y, end_x, end_y);
        }
    } else if start_y > end_y {
        line_high( engine, end_x, end_y, start_x, start_y);
    } else {
        line_high( engine, start_x, start_y, end_x, end_y);
    }
}
pub fn smart_set_pxl(engine: &mut ConsoleEngine, x: i32, y: i32, character: Pixel) {
        if x >= 0 && y >= 0 && x < engine.get_width() as i32 && y < engine.get_height() as i32 {
        let my_pxl =engine.get_pxl(x, y);
        if my_pxl.is_ok(){
            engine.set_pxl(x,y,pixel::pxl_fbg(character.chr, character.fg, my_pxl.unwrap().bg));
        }else{
            engine.set_pxl(x,y,character);
        }
    }
}