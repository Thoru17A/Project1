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


fn trapezoid_perimeter(base1: f64, base2: f64, side1: f64, side2: f64) -> Option <f64> {
    if base1 <= 0.0 || base2 <= 0.0 || side1 <= 0.0 || side2 <= 0.0
    {
        eprintln!("Ошибка: значение сторон трапеции не может быть отрицательным");
        return None;
    }

    Some(base1 + base2 + side1 + side2)
}

fn trapezoid_area(base1: f64, base2: f64, side1: f64, side2: f64) ->Option <f64> {
    if base1 <= 0.0 || base2 <= 0.0 || side1 <= 0.0 || side2 <= 0.0
    {
        eprintln!("Ошибка: значение сторон трапеции не может быть отрицательным");
        return None;
    }
    Some((base1 + base2) / 2.0 * (f64::powf(side1, 2.0) - f64::powf((f64::powf(base1 - base2, 2.0) + f64::powf(side1, 2.0) - f64::powf(side2, 2.0)) / (2.0 * (base1 - base2)), 2.0)).sqrt())
}

fn trapezoid_middleline(base1: f64, base2: f64) -> Option <f64> {
    if base1 <= 0.0 || base2 <= 0.0
    {
        eprintln!("Ошибка: значение сторон трапеции не может быть отрицательным");
        return None;
    }
    Some((base1 + base2) / 2.0)
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
    if let Some(trap_perimeter) = trapezoid_perimeter(base1, base2, side1, side2)
    {
        println!("Периметр трапеции: {}", trap_perimeter);
    }
    if let Some(trap_area) = trapezoid_area(base1, base2, side1, side2)
    {
        println!("Площадь трапеции: {}", trap_area);
    }
    if let Some(trap_midline) = trapezoid_middleline(base1, base2)
    {
        println!("Средняя линия трапеции: {}", trap_midline);
    }
}