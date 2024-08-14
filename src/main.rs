fn main(){
    let my_array = [12, 3, 21, 43, 1, 3];
    println!("{:?}", my_array);
    println!("Enter an index to pick it from the array");

    loop{
        let mut index = String::new();
        io::stdin().read_line(&mut index)
            .expect("Failed to read lines");
        
        let index: Result<usize, _> = index.trim().parse();

        match index{
            Ok(i) if i < my_array.len() => {
                let element = my_array[i];
                println!("The number for this index {:?} is {:?}", i, element);
                break;
            }
            _ => {
                println!("You are out of scope, enter a numer from 0 to {:?}", my_array.len()-1);

            }
        }

    }
}