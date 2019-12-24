use crate::Data;

pub fn data_angles(total: f32, data: &[Data]) -> Vec<f32> {
    let mut angle = 0.0;
    data.iter()
        .map(|d| d.value / total)
        .map(|pct| {
            let val = pct * 360.0;
            angle += val;
            angle
        })
        .collect()
}

pub fn calculate_width(radius: i32, y: i32, aspect_ratio: i32) -> i32 {
    let val = radius.pow(2) - y.pow(2);
    let width = ((val * aspect_ratio) as f32).sqrt().round() as i32;

    // the next code applies to the first and last line of the circle.
    // without it the circles have an ugly single dot on top and bottom.
    // I just experimented until it looked alright for all combinations of radius and aspect_ratio
    match (width, aspect_ratio) {
        (0, 1) => width,
        (0, 2) => radius / (aspect_ratio * 2),
        (0, _) => radius / aspect_ratio,
        _ => width,
    }
}

pub fn calculate_center_x(radius: i32, aspect_ratio: i32) -> i32 {
    let aspect_ratio_sqrt = (aspect_ratio as f32).sqrt();

    (radius as f32 * aspect_ratio_sqrt).round() as i32
}
