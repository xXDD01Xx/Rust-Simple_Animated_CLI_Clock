use chrono::{Local};    //declare chrono to variable "Local" (called below)

const DIGITS: [[&str; 11]; 7] = [
    ["┏━┓ ", "  ╻  ", " ┏━┓ ", " ┏━┓ ", " ╻ ╻ ", " ┏━┓ ", " ┏   ", " ┏━┓ ", " ┏━┓ ", " ┏━┓ ", "   "],
    ["┃ ┃ ", "  ┃  ", "   ┃ ", "   ┃ ", " ┃ ┃ ", " ┃   ", " ┃   ", " ┃ ┃ ", " ┃ ┃ ", " ┃ ┃ ", " ╻ "],
    ["┃ ┃ ", "  ┃  ", "   ┃ ", "   ┃ ", " ┃ ┃ ", " ┃   ", " ┃   ", "   ┃ ", " ┃ ┃ ", " ┃ ┃ ", "   "],
    ["┃ ┃ ", "  ┃  ", " ┏━┛ ", " ┣━┫ ", " ┗━┫ ", " ┗━┓ ", " ┣━┓ ", "   ┃ ", " ┣━┫ ", " ┗━┫ ", "   "],
    ["┃ ┃ ", "  ┃  ", " ┃   ", "   ┃ ", "   ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", "   "],
    ["┃ ┃ ", "  ┃  ", " ┃   ", "   ┃ ", "   ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", " ┃ ┃ ", "   ┃ ", " ╹ "],
    ["┗━┛ ", "  ╹  ", " ┗━━ ", " ┗━┛ ", "   ╹ ", " ┗━┛ ", " ┗━┛ ", "   ╹ ", " ┗━┛ ", " ┗━┛ ", "   "],
];



fn main()
{
    clock();
}


    fn clock()
    {
    print!("\x1b[2J");
    print!("\x1b[?25l");
    loop
    {
        let localTime = Local::now();
        // println!("{:?}", time);      //debug
        let time = localTime.format("%H:%M:%S").to_string();
        // println!("{:?}", time);
        for row in &DIGITS
        {
            for character in time.chars()
            {
                let column= match character
                {
                    '0'..= '9' => character as usize - '0' as usize,
                    ':' => 10,
                    _ => 10,
                };
                print!("{} ", row[column]);
            }
            println!();
        }
        std::thread::sleep(std::time::Duration::from_millis(999));
        print!("\x1b[7A");      //move the cursor up 7, to sync display
    }
}


