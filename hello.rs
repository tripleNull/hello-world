
/* Trying different methods of output.
*/
fn main() {
    let mut x = 1;
    let num1 = 10;
    loop {
    	if (x % 2) == 0 {
    		println!("{}",x);
    		x += 1;
    		continue;
    	}
    	if x > num1 {
    		break;
    	}
    	x += 1;
    	continue;
    }

    let mut y = 1;

    while y <= num1 {
    	println!("{}", y);
    	y += 1;
    }

    let rand_string = "I am a random string";

    let mut indiv_word = rand_string.split_whitespace();
    let mut iter = indiv_word.next();

    loop{
    	match iter {
    		Some(x) => println!("I have a word: {}", x),
    		None => break,
    	}

    	iter = indiv_word.next();
    }
}
