use crate::fragment::Fragment;
use crate::vertex::Vertex;
use raylib::math::Vector3;

// Filled triangle rasterization using barycentric coordinates.
pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    let p1 = v1.transformed_position;
    let p2 = v2.transformed_position;
    let p3 = v3.transformed_position;

    // Bounding box
    let min_x = p1.x.min(p2.x).min(p3.x).floor() as i32;
    let max_x = p1.x.max(p2.x).max(p3.x).ceil() as i32;
    let min_y = p1.y.min(p2.y).min(p3.y).floor() as i32;
    let max_y = p1.y.max(p2.y).max(p3.y).ceil() as i32;

    // Helper: edge function (signed area)
    let edge = |a_x: f32, a_y: f32, b_x: f32, b_y: f32, c_x: f32, c_y: f32| {
        (c_x - a_x) * (b_y - a_y) - (c_y - a_y) * (b_x - a_x)
    };

    // Triangle area (used as denominator)
    let area = edge(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
    if area.abs() < std::f32::EPSILON {
        return fragments; // degenerate triangle
    }

    // For each pixel in bounding box, test if inside triangle
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            // sample at pixel center
            let px = x as f32 + 0.5;
            let py = y as f32 + 0.5;

            let w1 = edge(px, py, p2.x, p2.y, p3.x, p3.y) / area;
            let w2 = edge(px, py, p3.x, p3.y, p1.x, p1.y) / area;
            let w3 = edge(px, py, p1.x, p1.y, p2.x, p2.y) / area;

            // Allow small negative epsilon due to float precision
            if w1 >= -1e-4 && w2 >= -1e-4 && w3 >= -1e-4 {
                // Interpolate depth
                let depth = w1 * p1.z + w2 * p2.z + w3 * p3.z;
                // Interpolate color from vertex colors
                let c1 = v1.color;
                let c2 = v2.color;
                let c3 = v3.color;
                let color = Vector3::new(
                    w1 * c1.x + w2 * c2.x + w3 * c3.x,
                    w1 * c1.y + w2 * c2.y + w3 * c3.y,
                    w1 * c1.z + w2 * c2.z + w3 * c3.z,
                );
                fragments.push(Fragment::new(px, py, color, depth));
            }
        }
    }

    fragments
}
