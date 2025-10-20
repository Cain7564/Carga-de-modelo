use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::math::Vector3;

pub fn line(a: &Vertex, b: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let start = a.transformed_position;
    let end = b.transformed_position;

    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = if dx > dy { dx / 2 } else { -dy / 2 };

    loop {
        // Interpolate parameter t along the primary axis to approximate distance along the line
        let t = if (end.x - start.x).abs() > (end.y - start.y).abs() {
            if end.x != start.x { (x0 as f32 - start.x) / (end.x - start.x) } else { 0.0 }
        } else {
            if end.y != start.y { (y0 as f32 - start.y) / (end.y - start.y) } else { 0.0 }
        };
        let z = start.z + (end.z - start.z) * t;

        // Interpolate color between the vertex colors
        let start_color = a.color;
        let end_color = b.color;
        let color = Vector3::new(
            start_color.x + (end_color.x - start_color.x) * t,
            start_color.y + (end_color.y - start_color.y) * t,
            start_color.z + (end_color.z - start_color.z) * t,
        );

        fragments.push(Fragment::new(x0 as f32, y0 as f32, color, z));

        if x0 == x1 && y0 == y1 { break; }

        let e2 = err;
        if e2 > -dx {
            err -= dy;
            x0 += sx;
        }
        if e2 < dy {
            err += dx;
            y0 += sy;
        }
    }

    fragments
}
