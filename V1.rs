use std::f64::consts::PI;

fn circle_length(radius: f64) -> f64 {
    2.0 * PI * radius
}

fn circle_area(radius: f64) -> f64 {
    PI * f64::powf(radius, 2.0)
}

fn sector_area(radius: f64, angle: f64) -> f64 {
    (angle / 360.0) * PI * f64::powf(radius, 2.0)
}

fn main() {
    let radius: f64 = 5.0;
    let angle: f64 = 90.0;
    
    println!("Длина окружности: {}", circle_length(radius));
    println!("Площадь окружности: {}", circle_area(radius));
    println!("Площадь сектора окружности: {}", sector_area(radius, angle));
}
