extern crate Mini_Json_Parser;
use Mini_Json_Parser::PlainReader;

fn main(){
    println!("this is json parser ! ");
    println!("from myfunction {} " ,Mini_Json_Parser::myfunction());







    let pr = PlainReader::from("example string form class");
    println!("{}",pr.raw_text());












}