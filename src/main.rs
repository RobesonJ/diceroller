
use std::env;
use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();   
    
    for i in 0..args.len(){
        match args[i].to_lowercase().as_str(){            
            "roll" => make_die(&args[i+1]),
            &_ => continue,
        }
        break;
    }
}
struct Die {    
    pip: i32,
    modifier: i32,
    sign: char,
}
fn make_die(arg: &str){
    let mut die = Die{        
        pip: 0,
        modifier: 0,
        sign: ' ',
    };
    let mut count = 0;  
    let get_int_array: Vec<&str> = arg.split(&['d', '+', '-']).collect();
    let get_operator_array: Vec<&str>  = arg.matches(&['+', '-']).collect();
    let intlen = get_int_array.len();
    let operlen: usize = get_operator_array.len();
    for i in 0..intlen {
        match i {
            0 => { count = get_int_array[i].parse::<i32>().expect("NaN");},
            1 => {die.pip = get_int_array[i].parse::<i32>().expect("NaN");}
            2 => {die.modifier = get_int_array[i].parse::<i32>().expect("NaN");}
            _ => continue,
        }
    }
    if operlen == 1 {
        die.sign = get_operator_array[0].parse::<char>().expect("oops");
    }
    
    die.roll(count);
    
    
    
}
impl Die {
    fn roll(&self, count: i32){
        for _i in 0 ..count{
            let mut total = 0;        
            let result: i32 = rand::thread_rng().gen_range(1..=self.pip);        
            match self.sign {
                '+' => {total = 1 * result + self.modifier;},
                '-' => {total = 1 * result - self.modifier;},
                _ => printerr("invalid sign"),
            }
            println!("{}", total);
        }
        
    } 
}
 
fn printerr(e: &str){
    println!("{}", e);
}
