fn main() {
    println!("Hello, world!");
    println!("{}", sing(1,0));
}

fn get_correct_word(n: u32) -> &'static str {
    match n {
        1 => " bottle",
        _ => " bottles"
    }
}

pub fn verse(n: u32) -> String {
    let mut verse_string = String::from("");
    if n > 0 {
        let begin_number = n.to_string();
        let after_number = (n-1).to_string();
        let bottle_word = get_correct_word(n);
        verse_string.push_str(begin_number.as_str());
        verse_string.push_str(bottle_word);
        verse_string.push_str(" of beer on the wall, ");  
        verse_string.push_str(begin_number.as_str());
        if n == 1 {
            verse_string.push_str(" bottle of beer.\nTake it down and pass it around, ");
        } else {
            verse_string.push_str(" bottles of beer.\nTake one down and pass it around, ");        
        }
        match n - 1 {
            0 => {
                verse_string.push_str("no more");
            },
            _ => {
                verse_string.push_str(after_number.as_str());
            }
        }
        verse_string.push_str(get_correct_word(n - 1));
        verse_string.push_str(" of beer on the wall.\n");        
    } else {
        verse_string.push_str("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }
    verse_string
}

pub fn sing(start: u32, end: u32) -> String {
    let mut result = vec![];
    let mut index = start;
    while index >= end {
        let verse_str = verse(index);
        result.push(verse_str.clone());
        if index == 0 {
            break;
        }
        index -= 1;
    }
    result.join("\n")
}
