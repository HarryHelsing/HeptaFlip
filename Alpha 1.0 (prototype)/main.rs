use std::io;

fn main() {
    let (mut one, mut two, mut three, mut four, mut five, mut six, mut seven) =
        (0, 0, 0, 1, 0, 0, 0);
    let (mut onel, mut twol, mut threel, mut fourl, mut fivel, mut sixl, mut sevenl) = (
        "--  --", "--  --", "--  --", "------", "--  --", "--  --", "--  --",
    );
    print!(
        "\nWelcome to Heptaflip!\nEnter a number between 1 and 5\nTo win, flip all the broken lines into solid lines\n"
    );
    print!("\n\t{sevenl}\n\t{sixl}\n\t{fivel}\n\t{fourl}\n\t{threel}\n\t{twol}\n\t{onel}\n");

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
                seven = (seven + 1) % 2;
                five = (five + 1) % 2;
            }
            2 => {
                six = (six + 1) % 2;
                four = (four + 1) % 2;
            }
            3 => {
                five = (five + 1) % 2;
                three = (three + 1) % 2;
            }
            4 => {
                four = (four + 1) % 2;
                two = (two + 1) % 2;
            }
            5 => {
                three = (three + 1) % 2;
                one = (one + 1) % 2;
            }
            _ => (),
        }

        if seven == 1 {
            sevenl = "------"
        };
        if seven == 0 {
            sevenl = "--  --"
        };
        if six == 1 {
            sixl = "------"
        };
        if six == 0 {
            sixl = "--  --"
        };
        if five == 1 {
            fivel = "------"
        };
        if five == 0 {
            fivel = "--  --"
        };
        if four == 1 {
            fourl = "------"
        };
        if four == 0 {
            fourl = "--  --"
        };
        if three == 1 {
            threel = "------"
        };
        if three == 0 {
            threel = "--  --"
        };
        if two == 1 {
            twol = "------"
        };
        if two == 0 {
            twol = "--  --"
        };
        if one == 1 {
            onel = "------"
        };
        if one == 0 {
            onel = "--  --"
        };

        print!("\n\t{sevenl}\n\t{sixl}\n\t{fivel}\n\t{fourl}\n\t{threel}\n\t{twol}\n\t{onel}\n");

        turns = turns + 1;

        if (one + two + three + four + five + six + seven) == 7 {
            if turns == 4 {
                println!("Congratulations! You finished Heptaflip in 4 turns, the fewest amount of turns possible!");
                break;
            }
        };
        if (one + two + three + four + five + six + seven) == 7 {
            print!("You Win!\nIt took {turns} turns\nTry again to see if you can complete it in fewer turns!\n");
            break;
        };
    }
}
