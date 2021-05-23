use std::io;

struct Aliment{
	name: String,
	kal: u32,
}

fn main() {
	read_calories_from_keybord();
	let mut v: Vec<Aliment> = Vec::new();
	let apple=Aliment { name: String::from("apple"), kal:55,};
	let orange=Aliment { name: String::from("orange"), kal:60,};
	let sanwich=Aliment { name: String::from("sanwich"), kal:100,};
	let juice=Aliment { name: String::from("juice"), kal:120,};
	v.push(apple);
	v.push(orange);
	v.push(sanwich);
	v.push(juice);
	let mut n=v.len();
	println!("{}",n);
	let mut i=0;
	
	 while i<2{ 
		let mut indx1={ let i=i;
						i+1};
		let mut indx2={ let n=v.len();
						n-1};
		println!("Numele alimentului {}",v[n].name);
		
	}
}

fn read_calories_from_keybord(){
	println!("Welcome!");
	
	loop{
		
		println!("Enter the number of calories:");

        let mut cal = String::new();

        io::stdin()
            .read_line(&mut cal)
            .expect("Failed to read line");

        let cal: u32 = match cal.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("This is not a valid number");
                continue;
            }
        };

        println!("The number of calories you entered are: {}", cal);
		break;
	}
}

fn build_list_of_aliments(){
	//let a:[Aliment;10];
	//a[0]=Aliment { name: String::from("apple"), kal:55,};
}
