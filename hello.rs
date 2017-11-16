
/* Trying different methods of output.
*/
fn main() {
    let mut x = 1;
    let num1 = 10;
    let num2 = 1;
    loop {
    	if (x % 2) == 0 {
    		println!("{}",x);
    		x += num2;
    		continue;
    	}
    	if x > num1 {
    		break;
    	}
    	x += num2;
    	continue;
    }

    let mut y = 1;

    while y <= num1 {
    	println!("{}", y);
    	y += num2;
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
