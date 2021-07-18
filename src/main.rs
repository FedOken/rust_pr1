use std::io;
use geometry::{calculate_area, calculate_volume, Circle, Rectangle, Sphere, Parallelepiped};

pub fn main() {
    let figure_types: [&str; 4] = ["circle", "rectangle", "sphere", "parallelepiped"];

    print_type_selection_question(figure_types);

    let figure_type_num = get_input().trim().parse::<i64>().unwrap();

    let mut area = 0.0;
    let mut volume = 0.0;
    // TODO: Make on this visibility level variable figure, that can contain different struct.
    // TODO: Area and volume calculation should be behind match loop.

    match figure_type_num {
        0 | 2 => {
            println!("Input radius:");
            let radius = get_input().trim().parse::<f64>().unwrap();

            if figure_type_num == 2 {
                area = calculate_area(&Sphere{ radius });
                volume = calculate_volume(&Sphere{ radius });
            } else {
                area = calculate_area(&Circle{ radius });
                volume = calculate_volume(&Circle{ radius });
            }
        },
        1 | 3 => {
            println!("Input length:");
            let length = get_input().trim().parse::<f64>().unwrap();

            println!("Input width:");
            let width = get_input().trim().parse::<f64>().unwrap();

            if figure_type_num == 3 {
                println!("Input height:");
                let height = get_input().trim().parse::<f64>().unwrap();

                area = calculate_area(&Parallelepiped{ length, width, height });
                volume = calculate_volume(&Parallelepiped{ length, width, height });
            } else {
                area = calculate_area(&Rectangle{ length, width });
                volume = calculate_volume(&Rectangle{ length, width });
            }
        },
        _ => {
            println!("Invalid figure type number!");
            panic!("Exit!");
        },
    }

    println!("Calculated {} figure. Area is {}. Volume is {}.", figure_types[figure_type_num as usize], area, volume);
}

fn print_type_selection_question(figure_types: [&str; 4]) {
    let mut base_str: String = "Calculate the area of figures, select the type of figure: ".to_owned();

    for index in 0..figure_types.len() {
        let index_string: String = index.to_string();
        let index_str: &str = &index_string;

        base_str.push_str(index_str);
        base_str.push_str(" - ");
        base_str.push_str(figure_types[index]);
        if index == figure_types.len() - 1 {
            base_str.push_str("...");
        } else {
            base_str.push_str(", ");
        }
    }
    println!("{}", base_str);
}

fn get_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line!");
    input
}