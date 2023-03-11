use std::io;

fn main() {
println!("Welcome to Heptaflip");
println!("~Zone One~");
    level(false, false, false, false, false, false, false);
    level(true, true, true, false, true, true, true);
    level(false, true, true, true, true, true, false);
    level(true, false, false, false, false, false, true);
    level(true, true, false, false, false, true, true);
    level(false, true, false, true, false, true, false);
    level(true, false, true, true, false, true, false);
println!("~Zone Two~");
    level2(false, false, true, true, true, true, true);
    level2(true, true, true, true, true, false, false);
    level2(false, false, true, true, true, false, false);
    level2(true, false, false, false, false, false, false);
    level2(false, false, false, false, false, false, true);
    level2(false, false, false, false, true, false, false);
    level2(true, false, false, true, true, false, false);
println!("~Zone Three~");
    level2(false, true, false, false, false, false, false);
    level2(false, false, false, false, false, true, false);
    level2(false, true, false, true, false, true, false);
    level2(false, true, true, true, true, true, false);
    level2(false, true, true, true, false, true, true);
    level2(true, false, true, false, true, false, false);
    level2(true, true, false, true, true, true, false);


println!("You win!!!!!!!")
}

fn level(
    mut l1: bool,
    mut l2: bool,
    mut l3: bool,
    mut l4: bool,
    mut l5: bool,
    mut l6: bool,
    mut l7: bool,
) {
println!("Type numbers from 1 - 7 to change the lines");
    convert_to_line(l7);
    convert_to_line(l6);
    convert_to_line(l5);
    convert_to_line(l4);
    convert_to_line(l3);
    convert_to_line(l2);
    convert_to_line(l1);

    let mut turns = 0;

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match input {
            7 => {
                if l7 == true {
                    l7 = false
                } else {
                    l7 = true
                }
            }

            6 => {
                if l6 == true {
                    l6 = false
                } else {
                    l6 = true
                }
            }

            5 => {
                if l5 == true {
                    l5 = false
                } else {
                    l5 = true
                }
            }

            4 => {
                if l4 == true {
                    l4 = false
                } else {
                    l4 = true
                }
            }

            3 => {
                if l3 == true {
                    l3 = false
                } else {
                    l3 = true
                }
            }

            2 => {
                if l2 == true {
                    l2 = false
                } else {
                    l2 = true
                }
            }

            1 => {
                if l1 == true {
                    l1 = false
                } else {
                    l1 = true
                }
            }

            _ => (),
        }
        turns = turns + 1;

        convert_to_line(l7);
        convert_to_line(l6);
        convert_to_line(l5);
        convert_to_line(l4);
        convert_to_line(l3);
        convert_to_line(l2);
        convert_to_line(l1);
        println!("Turns: {turns}");

if (l1, l2, l3, l4 ,l5, l6, l7) == (true, true, true, true, true, true, true) {
                 println!("Congratulations!");
                 break;
	}
    }
}


fn level2(
    mut l1: bool,
    mut l2: bool,
    mut l3: bool,
    mut l4: bool,
    mut l5: bool,
    mut l6: bool,
    mut l7: bool,
) {
println!("Type numbers from 1 - 6 to change the lines");
    convert_to_line(l7);
    convert_to_line(l6);
    convert_to_line(l5);
    convert_to_line(l4);
    convert_to_line(l3);
    convert_to_line(l2);
    convert_to_line(l1);

    let mut turns = 0;

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match input {

            6 => {
                if l7 == true {l7 = false} else {l7 = true}
                if l6 == true {l6 = false} else {l6 = true}
            }

            5 => {
                if l5 == true {l5 = false} else {l5 = true}
                if l6 == true {l6 = false} else {l6 = true}
            }

            4 => {
                if l5 == true {l5 = false} else {l5 = true}
                if l4 == true {l4 = false} else {l4 = true}
            }

            3 => {
                if l4 == true {l4 = false} else {l4 = true}
                if l3 == true {l3 = false} else {l3 = true}
            }

            2 => {
                if l3 == true {l3 = false} else {l3 = true}
                if l2 == true {l2 = false} else {l2 = true}
            }

            1 => {
                if l2 == true {l2 = false} else {l2 = true}
                if l1 == true {l1 = false} else {l1 = true}
            }

            _ => (),
        }
        turns = turns + 1;

        convert_to_line(l7);
        convert_to_line(l6);
        convert_to_line(l5);
        convert_to_line(l4);
        convert_to_line(l3);
        convert_to_line(l2);
        convert_to_line(l1);
        println!("Turns: {turns}");

if (l1, l2, l3, l4 ,l5, l6, l7) == (true, true, true, true, true, true, true) {
                 println!("Congratulations!");
                 break;
	}
    }
}









fn convert_to_line(var: bool) {
    if var == false {
        println!("--  --");
    } else {
        println!("------");
    }
}
