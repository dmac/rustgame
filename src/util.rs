use rsfml::graphics::{FloatRect};

use world::{Direction, North, East, South, West};

/// If r1 and r2 intersect, returns the new position of r1 when "bounced" out of r2
/// opposite to the specified direction. Returns None if r1 and r2 don't intersect.
pub fn collide_rects(r1: &FloatRect, r2: &FloatRect, direction: Direction) -> Option<(f32, f32)> {
    let intersects = FloatRect::intersects(r1, r2, &FloatRect::new(0.,0.,0.,0.));
    if !intersects { return None }

    let mut new_x = r1.left as i32;
    let mut new_y = r1.top as i32;
    match direction {
        North => {
            while new_y < r2.top as i32 + r2.height as i32 {
                new_y += 1;
            }
        }
        East => {
            while new_x + r1.width as i32 > r2.left as i32 {
                new_x -= 1;
            }
        }
        South => {
            while new_y + r1.height as i32 > r2.top as i32 {
                new_y -= 1;
            }
        }
        West => {
            while new_x < r2.left as i32 + r2.width as i32 {
                new_x += 1;
            }
        }
    }
    Some((new_x as f32, new_y as f32))
}
