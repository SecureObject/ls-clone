use std::io;
fn main(){
    let mut input = String::new();
        println!("Enter the directory name:");
   io::stdin().read_line(&mut input).expect("Failed to read line");
    check_return(input.trim());
}

fn check_return(input: &str) -> io::Result<()>{
   for value in std::fs::read_dir(input).unwrap(){
        println!("{:?}",value?.file_name());
        }
    Ok(())
   }