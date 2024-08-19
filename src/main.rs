
use std::env;
use rand::Rng;
fn main() {
    let args: Vec<String> = env::args().collect();
    let roll = "roll".to_string();   
    for i in 0..args.len(){
        if args[i] == roll {
            parsing(tokenize(&args[i+1]))
        }
    }
    
   
}
fn parsing(tokens: Vec<Token>){
    for token in tokens{
        println!("Token: {:?} {}",token.kind, token.lexeme)
    }
}
#[derive(Debug)]
enum TokenKind {
    // Single-character tokens.
    LEFT_PAREN, 
    RIGHT_PAREN, 
    LEFT_BRACE, 
    RIGHT_BRACE,
    COMMA, 
    DOT, 
    MINUS, 
    PLUS, 
    SEMICOLON, 
    SLASH, 
    STAR,
    DIE, // d
  
    // One or two character tokens.
    BANG, 
    BANG_EQUAL,
    EQUAL, 
    EQUAL_EQUAL,
    GREATER, 
    GREATER_EQUAL,
    LESS, 
    LESS_EQUAL,
  
    // Literals.     
    STRING, 
    NUMBER,
  
    // Keywords.
    ROLL, // roll
  
    
  }
struct Token{
    kind: TokenKind,
    lexeme: String,

}
impl Token {
    fn new() -> Token {
        Token{kind:TokenKind::STRING, lexeme:"".to_string()}
    }
    
}


fn tokenize(source: &String) -> Vec<Token> {
    //I need a match statement to tie TokenKind to its prospective String value
    let char_source: Vec<char> = source.chars().collect();
    let mut tokens: Vec<Token> = Vec::new();
    let mut numstring = "".to_string();
    let mut token = Token::new(); 
    for i in 0 .. char_source.len(){ 
              
        if char_source[i].is_numeric() {            
                numstring.push_str(&char_source[i].to_string());                
               
        } else {
            numstring = "".to_string();
        }

        if numstring != "".to_string(){
            token = Token { kind: TokenKind::NUMBER, lexeme: numstring.clone() };
            tokens.push(token);
        }

        if i < char_source.len(){
            match char_source[i] {
                '+' => {token = Token { kind: TokenKind::PLUS, lexeme: "+".to_string() };},
                '-' => {token = Token { kind: TokenKind::MINUS, lexeme: "-".to_string() };},
                'd' => {                    
                    token = Token { kind: TokenKind::DIE, lexeme: "d".to_string() };
                    
                } ,                          
                _ => continue,
            }        
            tokens.push(token);
            
        }
        
    }
    tokens
    
    
}
 
struct Die {    
    pip: i32,
    modifier: i32,
    sign: char,
}
impl Die {
    fn roll(&self, count: i32){
        for _i in 0 ..count{
            let mut total = 0;        
            let result: i32 = rand::thread_rng().gen_range(1..=self.pip);        
            match self.sign {
                '+' => {total = 1 * result + self.modifier;},
                '-' => {total = 1 * result - self.modifier;},
                _ => continue, //printerr("invalid sign"),
            }
            println!("{}", total);
        }
        
    } 
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

 
