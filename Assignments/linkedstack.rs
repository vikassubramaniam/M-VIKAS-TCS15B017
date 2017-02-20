fn main()
{let mut flag=0;
use std::collections::LinkedList;
use std::io;
let mut stack=LinkedList::new();
let mut string = String::new();
println!("Enter a string of paranthesis");
io::stdin().read_line(&mut string)
        .expect("Failed to read line");
for s in string.chars()
{if s=='('
{stack.push_front('(');
}
else if s==')'
{if stack.front()==Some(&'('){
stack.pop_front();
}
else
{println!("Paranthesis are imperfectly matched.");
flag=1;
break;
}
}
}
if stack.front()==None && flag==0 {
println!("Paranthesis match perfectly!")
    }
}