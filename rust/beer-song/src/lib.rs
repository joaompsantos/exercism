#[allow(unused_parens)]

pub fn verse(n: i32) -> String {   

    match n {
        2 => String::from("2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),   
        _ => format!("{0} bottles of beer on the wall, {0} bottles of beer.\nTake one down and pass it around, {1} bottles of beer on the wall.\n", n, n-1),
    }

}

pub fn sing(start: i32, end: i32) -> String {
    
    let mut song: String = String::new();
    let mut aux = start;

    while aux >= end {
        if !song.is_empty() {
            song += "\n";
        }
        song += &verse(aux);
        aux -= 1;
    }

    return song;

}
