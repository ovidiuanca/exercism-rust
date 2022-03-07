use enum_iterator::IntoEnumIterator;

#[derive(Debug, PartialEq, IntoEnumIterator)]
pub enum ResistorColor {
    Black,
    Brown,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Violet,
    Grey,
    White,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange => 3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9
    }
}

pub fn value_to_color_string(value: usize) -> String {
    match value {
        0 => String::from("Black"),
        1 => String::from("Brown"),
        2 => String::from("Red"),
        3 => String::from("Orange"),
        4 => String::from("Yellow"),
        5 => String::from("Green"),
        6 => String::from("Blue"),
        7 => String::from("Violet"),
        8 => String::from("Grey"),
        9 => String::from("White"),
        _ => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    let mut colors:Vec<ResistorColor> = vec![];

    for color in ResistorColor::into_enum_iter() {
        colors.push(color)
    }

    return colors
}
