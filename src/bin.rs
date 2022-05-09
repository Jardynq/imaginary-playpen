mod math;
use macro_derive::*;
use math::Complex as C;
use math::Extended;
use math::interpreter::*;
use math::serializer::*;


#[derive(Clone, Debug)]
enum Variable {
    x,
}
impl KeywordType for Variable {
    fn serialize(name: &str) -> u32 {
        match name {
            "x" => 0,
            _ => 0,
        }
    }
}


#[derive(Clone, Debug)]
enum Function {
    sin,
    cos,
}
impl KeywordType for Function {
    fn serialize(name: &str) -> u32 {
        match name {
            "sin" => 0,
            "cos" => 1,
            _ => 0,
        }
    }
}



fn main() {
    println!("none:     {}", C::new(5.3, 7.8).to_string());
    println!("none:     {}", C::new(5.3, 7.8).to_string_phasor());
    println!("none:     {}", C::new(5.3, 7.8).to_string_polar());
    println!("none:     {}", C::new(5.3, 7.8).to_string_exponential());
    println!("sin:      {}", C::new(5.3, 7.8).sin());
    println!("cos:      {}", C::new(5.3, 7.8).cos());
    println!("tan:      {}", C::new(5.3, 7.8).tan());
    println!("pow5:     {}", C::new(5.3, 7.8).pow(C::new(5.0, 0.0)));
    println!("pow3.74:  {}", C::new(5.3, 7.8).pow(C::new(3.74, 0.0)));
    println!("powC:     {}", C::new(5.3, 7.8).pow(C::new(2.4, 6.2)));
    println!("squared:  {}", C::new(5.3, 7.8).pow(C::new(5.3, 7.8)));
    println!("pow0:     {}", C::new(5.3, 7.8).pow(C::new(0.0, 0.0)));
    println!("powi:     {}", C::new(5.3, 7.8).pow(C::new(0.0, 1.0)));
    println!("log:      {}", C::new(5.3, 7.8).log());
    println!("logr:     {}", C::new(5.3, 0.0).log());
    println!("loh:      {}", 5.25_f64.log_hypot(4.5));


    let input1 = "(.395         +56__2_i ) + 7 ^ (3 * 5 + 5)";
    let input2 = "++++++(.2+++6)";
    let input3 = "x y + 7";
    let input5 = "---5(2+8) + 5awd - x(8)";
    let input6 = "16^(1/2)";
    let input7 = "cos(  1 + \n 5+7)* 23    9 2. ";
    let input8 = "2.|-.3| + |2|";
    let input9 = "(2)+|-3|(10)";
    let input10 = "0+8)24"; // TODO: Returns after ')'
    let input11 = "(10 + 6.2i) * 2";
    // TODO: potential errors
    // 2+_, _*2, 2*_, _),
    // TODO: allow empty delimiters by setting their left to None,
    // then in serialize and the intepetrer interpret that as do nothing.
    
    let mut expr = math::Expression::parse("cos(2.0i)", &math::Keyword::funcs(
        &vec!["sin", "cos"]
    ));
    
    println!("{}", expr.pretty_error());
    println!("{}", expr.tree.clone().unwrap());
    expr.optimize();
    println!("{}", expr.tree.clone().unwrap());
    println!("{:#?}", expr.tree.clone().unwrap().serialize::<Variable, Function>());


    let mut intep = math::Intepreter::new();
    intep.repl_handler(|expr| {
        vec![ReplCommand::Evaluate]
    });
}