fn main()
{
let mut done=false;
use std::collections::LinkedList;
use std::io;
let mut queue=LinkedList::new();


while !done
{
	println!("Choose one of the following options:");
	println!("1.Insert");
	println!("2.Delete");
	println!("3.Display");
	println!("4.Exit");
	let mut string = String::new();
	io::stdin().read_line(&mut string)
        .expect("Failed to read line");
    let trimmed = string.trim();
    //let string: i32 = string.trim().parse().expect("valid input");
	//let my_int = string.parse::<i32>().unwrap();	
    match trimmed.parse::<u32>()
	{
        
		Ok(1) => {println!("Enter a number");
				let mut temp = String::new();
				io::stdin().read_line(&mut temp).expect("Failed to read line");
				let temp: i32 = temp.trim().parse().expect("invalid input");
				queue.push_back(temp);
				
			 }
		,
		Ok(2) => {queue.pop_front();},
		Ok(3) =>{
				let mut it=queue.iter();
				if queue.is_empty()
					{println!("Queue is empty.");
					}
				else
				{
				loop
				{
					match it.next()
					{
						Some(x)=> {print!(" {} ",x);},
						None => {break}
					}
				}
				}
				
				
			}
		,
		Ok(4) => done=true,
		Ok(_) => {
				println!("this was not an integer: {}",string);
				done=true;
			  },
		Err(_) => {
				println!("this was not an integer: {}",string);
				done=true;
			  }

    
	}

}
}
