pub mod areas_volumes;
use areas_volumes::*;

pub fn area_fit(
    (x, y): (usize, usize),
    kind: GeometricalShapes,
    times: usize,
    (a, b): (usize, usize),
) -> bool {
    let area = rectangle_area(x, y);
    match kind {
        GeometricalShapes::Triangle => triangle_area(a, b) * (times as f64) <= area as f64,
        GeometricalShapes::Rectangle => rectangle_area(a, b) * times <= area,
        GeometricalShapes::Circle => circle_area(a) * (times as f64) <= area as f64,
        GeometricalShapes::Square => square_area(a) * times <= area,
    }
}

pub fn volume_fit(
    (x, y, z): (usize, usize, usize),
    kind: GeometricalVolumes,
    times: usize,
    (a, b, c): (usize, usize, usize),
) -> bool {
    let boxx = parallelepiped_volume(x, y, z);
    match kind {
        GeometricalVolumes::Cube => cube_volume(a) * times <= boxx,
        GeometricalVolumes::Cone => cone_volume(a, b) * times as f64 <= boxx as f64,
        GeometricalVolumes::TriangularPyramid => {
            triangular_pyramid_volume(a as f64, b) * times as f64 <= boxx as f64
        }
        GeometricalVolumes::Sphere => sphere_volume(a) * times as f64 <= boxx as f64,
        GeometricalVolumes::Parallelepiped => parallelepiped_volume(a, b, c) * times <= boxx,
    }
}
