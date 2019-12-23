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

    // without this the circles have an ugly single dot on top and bottom.
    if width == 0 && aspect_ratio > 1 {
        radius / (aspect_ratio * 2)
    } else {
        width
    }
}
