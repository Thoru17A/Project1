# Project1

Этот проект представляет собой программу на языке программирования Rust, предназначенную для выполнения различных геометрических вычислений.

## Описание

Программа содержит реализацию нескольких функций, позволяющих производить следующие вычисления:

- **Вычисление длины окружности**: Функция `circle_length(radius: f64) -> f64` принимает радиус окружности в качестве аргумента и возвращает её длину.

- **Вычисление площади окружности**: Функция `circle_area(radius: f64) -> f64` принимает радиус окружности в качестве аргумента и возвращает её площадь.

- **Вычисление площади сектора окружности**: Функция `sector_area(radius: f64, angle: f64) -> f64` принимает радиус и угол сектора окружности в качестве аргументов и возвращает его площадь.

- **Вычисление периметра трапеции**: Функция `trapezoid_perimeter(base1: f64, base2: f64, side1: f64, side2: f64) -> f64` принимает основания и боковые стороны трапеции и возвращает её периметр.

- **Вычисление площади трапеции**: Функция `trapezoid_area(base1: f64, base2: f64, side1: f64, side2: f64) -> f64` принимает основания и боковые стороны трапеции и возвращает её площадь.

- **Вычисление средней линии трапеции**: Функция `trapezoid_middleline(base1: f64, base2: f64) -> f64` принимает основания трапеции и возвращает её среднюю линию.

## Использование

Программа предоставляет пример использования функций для вычисления геометрических параметров и вывода результатов в стандартный вывод.

# Запуск

Для запуска программы требуется установленный компилятор Rust. Выполните следующие шаги:

Склонируйте репозиторий на свой компьютер.
Откройте терминал и перейдите в директорию с проектом.
Выполните команду cargo run.

