
extern crate Mini_Json_Parser;
use Mini_Json_Parser::jobject::JsonObject;


enum StackValueType {
    Left_Brace(Option<JsonObject>),     //左花括号
    Right_Brace,                        //右花括号
    Double_Quotation(Option<String>),   //双引号
    Open_Bracket,                       //左方括号
    Colon,                              //冒号
    Comma,                              //逗号

    vString(String),                    //字符
}

pub struct ValueStack {
    top: Option<Box<ValueNode>>,
}
/*
    解析Json使用的栈定义
*/
pub struct ValueNode {
    type: Option<Box<StackValueType>>,
    next: Option<Box<ValueNode>>,
}
/*
    解析Json使用的栈中元素的定义
*/



impl ValueStack {
    fn new() -> ValueStack {
        ValueStack{ top: None }
    }
    fn insert(&mut self , input: StackValueType) {
        let t = self.top.take();
        let newitem = Some(ValueNode {
            type: Some(Box::new(input)),
            next: t
        });
        self.top = newitem;
    }
    fn pop(&mut self) -> Option<Box<StackValueType>> {
        let mut t = self.top.take();
    
        match t {
            None => { None },
            Some(ref mut x) => {
                let re = x.type.take();
                self.top = x.next;
                re 
            }
        }
    }

    fn check_top(&mut self) &Option<Box<StackValueType>> {
        match self.top {
            None => None,
            Some(ref x) => {
                &(x.type)
            }
        }
    }
}





