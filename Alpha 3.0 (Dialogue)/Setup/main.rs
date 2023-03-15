use std::io;

fn main() {
println!("Welcome to Heptaflip");
println!("~Zone One~");
    zone(false, false, false, false, false, false, false, 7, 1);
    zone(true, true, true, false, true, true, true, 1, 1);
    zone(false, true, true, true, true, true, false, 2, 1);
    zone(true, false, false, false, false, false, true, 5, 1);
    zone(true, true, false, false, false, true, true, 3, 1);
    zone(false, true, false, true, false, true, false, 4, 1);
    zone(true, false, true, true, false, true, false, 3, 1);
println!("~Zone Two~");
    zone(false, false, true, true, true, true, true, 1, 2);
    zone(true, true, true, true, true, false, false, 1, 2);
    zone(false, false, true, true, true, false, false, 2, 2);
    zone(true, false, false, false, false, false, false, 3, 2);
    zone(false, false, false, false, false, false, true, 3, 2);
    zone(false, false, false, false, true, false, false, 3, 2);
    zone(true, false, false, true, true, false, false, 2, 2);
println!("~Zone Three~");
    zone(false, true, false, false, false, false, false, 4, 2);
    zone(false, false, false, false, false, true, false, 4, 2);
    zone(false, true, false, true, false, true, false, 4, 2);
    zone(false, true, true, true, true, true, false, 6, 2);
    zone(false, true, true, true, false, true, true, 4, 2);
    zone(true, false, true, false, true, false, false, 3, 2);
    zone(true, true, false, true, true, true, false, 4, 2);
println!("~Zone Four~");
    zone(false, false, false, true, false, false, false, 2, 3);
    zone(true, true, false, false, false, true, true, 1, 3);
    zone(false, false, true, false, true, false, false, 3, 3);
    zone(false, true, false, false, false, true, false, 5, 3);
    zone(false, true, true, true, true, true, false, 4, 3);
    zone(true, true, false, true, true, false, true, 2, 3);
    zone(true, false, true, true, true, false, false, 3, 3);
println!("~Zone Five~");
    zone(true, true, false, true, false, true, true, 1, 4);
    zone(false, false, false, false, true, true, true, 2, 4);
    zone(true, true, false, false, false, false, true, 2, 4);
    zone(false, true, false, true, false, true, false, 2, 4);
    zone(false, false, false, true, false, false, false, 4, 4);
    zone(true, false, false, true, false, false, true, 3, 4);
    zone(false, false, true, true, true, false, false, 5, 4);
println!("~Zone Six~");
    zone(true, true, true, false, true, true, false, 1, 5);
    zone(true, false, false, true, false, false, true, 2, 5);
    zone(false, false, true, false, false, true, true, 2, 5);
    zone(false, false, false, false, false, false, true, 3, 5);
    zone(false, false, true, true, false, true, false, 3, 5);
    zone(false, true, true, true, true, true, false, 2, 5);
    zone(false, false, true, true, false, true, false, 3, 5);
println!("~Zone Seven~");
    zone(true, false, true, false, true, false, true, 1, 6);
    zone(false, true, false, true, false, true, true, 1, 6);
    zone(true, true, false, true, false, true, false, 1, 6);
    zone(false, false, false, false, false, false, true, 2, 6);
    zone(false, false, true, false, true, false, false, 3, 6);
    zone(false, true, true, true, true, true, false, 2, 6);
    zone(false, false, false, false, false, false, false, 1, 7);
println!("You win!!!!!!!")

}


fn zone(
     mut l1: bool,
     mut l2: bool,
     mut l3: bool,
     mut l4: bool,
     mut l5: bool,
     mut l6: bool,
     mut l7: bool,
     perfect: i32,
     moveset: i32,
)
{
print!("\nType numbers from 1 - 7 to change the lines\n\n");
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
        };//Create fn for interchangeable moves?

     match moveset
     {
	1 => {
        match input {

            7 => {l7 = !l7}
            6 => {l6 = !l6}
            5 => {l5 = !l5}
            4 => {l4 = !l4}
            3 => {l3 = !l3}
            2 => {l2 = !l2}
            1 => {l1 = !l1}
            _ => (),
        	   }
    	     }
	2 => {
        match input {

            6 => {l6 = !l6; l7 = !l7}
            5 => {l5 = !l5; l6 = !l6}
            4 => {l4 = !l4; l5 = !l5}
            3 => {l3 = !l3; l4 = !l4}
            2 => {l2 = !l2; l3 = !l3}
            1 => {l1 = !l1; l2 = !l2}
            _ => (),
		    }
             }
   3 => {
        match input {

            5 => {l5 = !l5; l6 = !l6; l7 = !l7}
            4 => {l4 = !l4; l5 = !l5; l6 = !l6}
            3 => {l3 = !l3; l4 = !l4; l5 = !l5}
            2 => {l2 = !l2; l3 = !l3; l4 = !l4}
            1 => {l1 = !l1; l2 = !l2; l3 = !l3}
            _ => (), 
                    }
             }
   4 => {
        match input {

            5 => {l5 = !l5; l7 = !l7}
            4 => {l4 = !l4; l6 = !l6}
            3 => {l3 = !l3; l5 = !l5}
            2 => {l2 = !l2; l4 = !l4}
            1 => {l1 = !l1; l3 = !l3}
            _ => (), 
                    }
             }
   5 => {
        match input {

            4 => {l4 = !l4; l7 = !l7}
            3 => {l3 = !l3; l6 = !l6}
            2 => {l2 = !l2; l5 = !l5}
            1 => {l1 = !l1; l4 = !l4}
            _ => (), 
                    }
             }
   6 => {
        match input {

            3 => {l3 = !l3; l5 = !l5; l7 = !l7}
            2 => {l2 = !l2; l4 = !l4; l6 = !l6}
            1 => {l1 = !l1; l3 = !l3; l5 = !l5}
            _ => (), 
                    }
             }
   7 => {
        match input {

            1 => {l1 = !l1; l2 = !l2; l3 = !l3; l4 = !l4; l5 = !l5; l6 = !l6; l7 = !l7}
            _ => (), 
                    }
             }

	_ => (),
     }

	turns += 1;
        convert_to_line(l7);
        convert_to_line(l6);
        convert_to_line(l5);
        convert_to_line(l4);
        convert_to_line(l3);
        convert_to_line(l2);
        convert_to_line(l1);
        print!("\n\nTurns: {turns}\n\n");

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
         println!("	--  --");
     } else {
         println!("	------");
     }
 }
