use std::io;

fn main() {
println!("Welcome to Heptaflip");
println!("~Zone One~");
    level(false, false, false, false, false, false, false, 7);
    level(true, true, true, false, true, true, true, 1);
    level(false, true, true, true, true, true, false, 2);
    level(true, false, false, false, false, false, true, 5);
    level(true, true, false, false, false, true, true, 3);
    level(false, true, false, true, false, true, false, 4);
    level(true, false, true, true, false, true, false, 3);
println!("~Zone Two~");
    level2(false, false, true, true, true, true, true, 1);
    level2(true, true, true, true, true, false, false, 1);
    level2(false, false, true, true, true, false, false, 2);
    level2(true, false, false, false, false, false, false, 3);
    level2(false, false, false, false, false, false, true, 3);
    level2(false, false, false, false, true, false, false, 3);
    level2(true, false, false, true, true, false, false, 2);
println!("~Zone Three~");
    level2(false, true, false, false, false, false, false, 4);
    level2(false, false, false, false, false, true, false, 4);
    level2(false, true, false, true, false, true, false, 4);
    level2(false, true, true, true, true, true, false, 6);
    level2(false, true, true, true, false, true, true, 4);
    level2(true, false, true, false, true, false, false, 3);
    level2(true, true, false, true, true, true, false, 4);
println!("~Zone Four~");
    level3(false, false, false, true, false, false, false, 2);
    level3(true, true, false, false, false, true, true, 1);
    level3(false, false, true, false, true, false, false, 3);
    level3(false, true, false, false, false, true, false, 5);
    level3(false, true, true, true, true, true, false, 4);
    level3(true, true, false, true, true, false, true, 2);
    level3(true, false, true, true, true, false, false, 3);
println!("~Zone Five~");
    level4(true, true, false, true, false, true, true, 1);
    level4(false, false, false, false, true, true, true, 2);
    level4(true, true, false, false, false, false, true, 2);
    level4(false, true, false, true, false, true, false, 2);
    level4(false, false, false, true, false, false, false, 4);
    level4(true, false, false, true, false, false, true, 3);
    level4(false, false, true, true, true, false, false, 5);
println!("~Zone Six~");
    level5(true, true, true, false, true, true, false, 1);
    level5(true, false, false, true, false, false, true, 2);
    level5(false, false, true, false, false, true, true, 2);
    level5(false, false, false, false, false, false, true, 3);
    level5(false, false, true, true, false, true, false, 3);
    level5(false, true, true, true, true, true, false, 2);
    level5(false, false, true, true, false, true, false, 3);
println!("~Zone Seven~");
    level6(true, false, true, false, true, false, true, 1);
    level6(false, true, false, true, false, true, true, 1);
    level6(true, true, false, true, false, true, false, 1);
    level6(false, false, false, false, false, false, true, 2);
    level6(false, false, true, false, true, false, false, 3);
    level6(false, true, true, true, true, true, false, 2);
    level7(false, false, false, false, false, false, false, 1);


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
    perfect: i32,
) {
println!("Type numbers from 1 - 7 to change the lines");
    convert_to_line(l7);
    convert_to_line(l6);
    convert_to_line(l5);
    convert_to_line(l4);
    convert_to_line(l3);
    convert_to_line(l2);
    convert_to_line(l1);

    let mut turns: i32 = 0;

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
		if turns == perfect {println!("Perfect! The level was completed in {turns} turns, the fewest possible.")}
println!("~~~~~~~~~~~~~~~~~~~~~~~~");
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
    perfect: i32,
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
		if turns == perfect {println!("Perfect! The level was completed in {turns} turns, the fewest possible.")}
println!("~~~~~~~~~~~~~~~~~~~~~~~~");
                 break;
	}
    }
}



fn level3(
    mut l1: bool,
    mut l2: bool,
    mut l3: bool,
    mut l4: bool,
    mut l5: bool,
    mut l6: bool,
    mut l7: bool,
    perfect: i32,
) {
println!("Type numbers from 1 - 5 to change the lines");
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


            5 => {
                if l7 == true {l7 = false} else {l7 = true}
                if l6 == true {l6 = false} else {l6 = true}
                if l5 == true {l5 = false} else {l5 = true}
            }

            4 => {
                if l6 == true {l6 = false} else {l6 = true}
                if l5 == true {l5 = false} else {l5 = true}
                if l4 == true {l4 = false} else {l4 = true}
            }

            3 => {
                if l5 == true {l5 = false} else {l5 = true}
                if l4 == true {l4 = false} else {l4 = true}
                if l3 == true {l3 = false} else {l3 = true}
            }

            2 => {
                if l4 == true {l4 = false} else {l4 = true}
                if l3 == true {l3 = false} else {l3 = true}
                if l2 == true {l2 = false} else {l2 = true}
            }

            1 => {
                if l3 == true {l3 = false} else {l3 = true}
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
		if turns == perfect {println!("Perfect! The level was completed in {turns} turns, the fewest possible.")}
println!("~~~~~~~~~~~~~~~~~~~~~~~~");
                 break;
	}
    }
}


fn level4(
    mut l1: bool,
    mut l2: bool,
    mut l3: bool,
    mut l4: bool,
    mut l5: bool,
    mut l6: bool,
    mut l7: bool,
    perfect: i32,
) {
println!("Type numbers from 1 - 5 to change the lines");
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


            5 => {
                if l7 == true {l7 = false} else {l7 = true}
                if l5 == true {l5 = false} else {l5 = true}
            }

            4 => {
                if l6 == true {l6 = false} else {l6 = true}
                if l4 == true {l4 = false} else {l4 = true}
            }

            3 => {
                if l5 == true {l5 = false} else {l5 = true}
                if l3 == true {l3 = false} else {l3 = true}
            }

            2 => {
                if l4 == true {l4 = false} else {l4 = true}
                if l2 == true {l2 = false} else {l2 = true}
            }

            1 => {
                if l3 == true {l3 = false} else {l3 = true}
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
		if turns == perfect {println!("Perfect! The level was completed in {turns} turns, the fewest possible.")}
println!("~~~~~~~~~~~~~~~~~~~~~~~~");
                 break;
	}
    }
}



fn level5(
    mut l1: bool,
    mut l2: bool,
    mut l3: bool,
    mut l4: bool,
    mut l5: bool,
    mut l6: bool,
    mut l7: bool,
    perfect: i32,
) {
println!("Type numbers from 1 - 4 to change the lines");
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



            4 => {
                if l7 == true {l7 = false} else {l7 = true}
                if l4 == true {l4 = false} else {l4 = true}
            }

            3 => {
                if l6 == true {l6 = false} else {l6 = true}
                if l3 == true {l3 = false} else {l3 = true}
            }

            2 => {
                if l5 == true {l5 = false} else {l5 = true}
                if l2 == true {l2 = false} else {l2 = true}
            }

            1 => {
                if l4 == true {l4 = false} else {l4 = true}
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
		if turns == perfect {println!("Perfect! The level was completed in {turns} turns, the fewest possible.")}
println!("~~~~~~~~~~~~~~~~~~~~~~~~");
                 break;
	}
    }
}


fn level6(
    mut l1: bool,
    mut l2: bool,
    mut l3: bool,
    mut l4: bool,
    mut l5: bool,
    mut l6: bool,
    mut l7: bool,
    perfect: i32,
) {
println!("Type numbers from 1 - 3 to change the lines");
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




            3 => {
                if l7 == true {l7 = false} else {l7 = true}
                if l5 == true {l5 = false} else {l5 = true}
                if l3 == true {l3 = false} else {l3 = true}
            }

            2 => {
                if l6 == true {l6 = false} else {l6 = true}
                if l4 == true {l4 = false} else {l4 = true}
                if l2 == true {l2 = false} else {l2 = true}
            }

            1 => {
                if l5 == true {l5 = false} else {l5 = true}
                if l3 == true {l3 = false} else {l3 = true}
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
		if turns == perfect {println!("Perfect! The level was completed in {turns} turns, the fewest possible.")}
println!("~~~~~~~~~~~~~~~~~~~~~~~~");
                 break;
	}
    }
}





fn level7(
    mut l1: bool,
    mut l2: bool,
    mut l3: bool,
    mut l4: bool,
    mut l5: bool,
    mut l6: bool,
    mut l7: bool,
    perfect: i32,
) {
println!("Type numbers from 1 - 1 to change the lines...");
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





            1 => {
                if l7 == true {l7 = false} else {l7 = true}
                if l6 == true {l6 = false} else {l6 = true}
                if l5 == true {l5 = false} else {l5 = true}
                if l4 == true {l4 = false} else {l4 = true}
                if l3 == true {l3 = false} else {l3 = true}
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
		if turns == perfect {println!("Perfect! The level was completed in {turns} turns, the fewest possible.")}
println!("~~~~~~~~~~~~~~~~~~~~~~~~");
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
