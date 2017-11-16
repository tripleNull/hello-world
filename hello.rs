
/* Trying different methods of output.
*/
fn main() {

    //constants
    let num1 = 10;
    let num2 = 1;
    
    //mutables
    let mut x = num2;
    let mut y = num2;

    //print out even numbers less than 10
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

    //print out all numbers less than 10
    while y <= num1 {
    	println!("{}", y);
    	y += num2;
    }

    let rand_string = "I am a random string";

    let mut indiv_word = rand_string.split_whitespace();
    let mut iter = indiv_word.next();

    //print out each word individually
    loop{
    	match iter {
    		Some(x) => println!("I have a word: {}", x),
    		None => break,
    	}

    	iter = indiv_word.next();
    }
}
