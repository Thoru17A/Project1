// Импорт константы PI из модуля f64::consts
use std::f64::consts::PI;

// Функция для вычисления длины окружности по радиусу
fn circle_length(radius: f64) -> Option<f64> {
    if radius < 0.0 {
        eprintln!("Ошибка: радиус не может быть отрицательным.");
        return None;
    }
    Some(2.0 * PI * radius)
}

// Функция для вычисления площади окружности по радиусу
fn circle_area(radius: f64) -> Option<f64> {
    if radius < 0.0 {
        eprintln!("Ошибка: радиус не может быть отрицательным.");
        return None;
    }
    Some(PI * f64::powf(radius, 2.0))
}

// Функция для вычисления площади сектора окружности по радиусу и углу
fn sector_area(radius: f64, angle: f64) -> Option<f64> {
    if radius < 0.0 || angle < 0.0  {
        eprintln!("Ошибка: радиус и угол не могут быть отрицательными.");
        return None;
    }
    if  angle > 360.0  {
        eprintln!("Ошибка: угол не может быть больше 36.");
        return None;
    }
    Some((angle / 360.0) * PI * f64::powf(radius, 2.0))
}

fn main() {
    let radius: f64 = 25.0;
    let angle: f64 = 370.0;

    if let Some(length) = circle_length(radius) {
        println!("Длина окружности: {}", length);
    }
    if let Some(area) = circle_area(radius) {
        println!("Площадь окружности: {}", area);
    }
    if let Some(sector_area) = sector_area(radius, angle) {
        println!("Площадь сектора окружности: {}", sector_area);
    }
}
//hy