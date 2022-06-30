fn main() {
    println!("The Twelve Days of Christmas");

    let days: [&str; 12] = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let lyrics: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a laying",
        "Seven swans are swimming",
        "Eight maids a milking",
        "Nine drummers drumming",
        "Ten pipers piping",
        "Eleven ladies dancing",
        "Twelve Lords a leaping"
    ];

    for day in 0..12 {
        println!("On the {} day of christmas, my true love sent to me: ", days[day]);
        for lyric_index in (0..day + 1).rev() {
            if lyric_index == 0 && day > 0 {
                println!(" And ");
            }

            println!(" {}", lyrics[lyric_index]);

            if lyric_index > 0 {
                println!(",");
            }else {
                println!(".");
            }
        }
        println!();
    }
}
