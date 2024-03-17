// Импорт константы PI из модуля f64::consts
use std::f64::consts::PI;

// Функция для вычисления длины окружности по радиусу
fn circle_length(radius: f64) -> f64 {
    2.0 * PI * radius
}

// Функция для вычисления площади окружности по радиусу
fn circle_area(radius: f64) -> f64 {
    PI * f64::powf(radius, 2.0)
}

// Функция для вычисления площади сектора окружности по радиусу и углу
fn sector_area(radius: f64, angle: f64) -> f64 {
    (angle / 360.0) * PI * f64::powf(radius, 2.0)
}


fn trapezoid_perimeter(base1: f64, base2: f64, side1: f64, side2: f64) -> f64 {
    base1 + base2 + side1 + side2
}

fn trapezoid_area(base1: f64, base2: f64, side1: f64, side2: f64) -> f64 {
    (base1 + base2) / 2.0 * (f64::powf(side1, 2.0) - f64::powf((f64::powf(base1 - base2, 2.0) + f64::powf(side1, 2.0) - f64::powf(side2, 2.0)) / (2.0 * (base1 - base2)), 2.0)).sqrt()
}

fn trapezoid_middleline(base1: f64, base2: f64) -> f64 {
    (base1 + base2) / 2.0
}

fn main() {
    let radius: f64 = 5.0;
    let angle: f64 = 90.0;
    
    println!("Длина окружности: {}", circle_length(radius));
    println!("Площадь окружности: {}", circle_area(radius));
    println!("Площадь сектора окружности: {}", sector_area(radius, angle));

    let base1: f64 = 6.0;
    let base2: f64 = 13.0;
    let side1: f64 = 7.0;
    let side2: f64 = 8.0;
    println!("Периметр трапеции: {}", trapezoid_perimeter(base1, base2, side1, side2));
    println!("Площадь трапеции: {}", trapezoid_area(base1, base2, side1, side2));
    println!("Средняя линия трапеции: {}", trapezoid_middleline(base1, base2));
}
