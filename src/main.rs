use std::io;

fn main(){
    println!("Ingrese expresión matematica: ");

    let mut exp:String = String::new();
    io::stdin().read_line(&mut exp).expect("Failed to read line");

    exp = solve_products(&exp);
    println!("Resultado: {}", exp);

}

fn solve_products(exp:&String) -> String {
    let mut chars1: Vec<char> = exp.chars().collect();
    let mut chars = exp.chars().collect::<Vec<char>>();
    for (i,c) in chars.iter().enumerate() {
        let product : char = '.';
        if *c == product{
            let mut before = chars[i-1];
            let mut count:usize = 0;
            while before == ' '{
                count = count + 1;
                before = chars[i - count]
            }

            let mut after:char = chars[i+1];
            let mut count2 = 0;

            while after == ' '{
                count2 = count2 + 1;
                after = chars[i + count2]
            }
            let mut resultado:u32 = 0;

            if let (Some(before_digit), Some(after_digit)) = (before.to_digit(10), after.to_digit(10)) {
                resultado = before_digit * after_digit;
                println!("Resultado: {}", resultado);
            }

            let resultado_string = resultado.to_string();
            let mut resultado_vector: Vec<char> = Vec::new();
            for digit_char in resultado_string.chars(){
                resultado_vector.push(digit_char);
            }
            chars1.splice(i-1-count..i+2+count2,resultado_vector);

        }

    }
    return chars1.iter().collect();
}