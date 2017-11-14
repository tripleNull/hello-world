

fn main() {
    let mut x = 1;
    loop {
    	if (x % 2) == 0{
    		println!("{}",x);
    		x += 1;
    		continue;
    	}
    	if x > 10{
    		break;
    	}
    	x += 1;
    	continue;
    }
}
