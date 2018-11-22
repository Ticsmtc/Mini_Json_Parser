
/*
    用来存储JSON解析出的值，用栈存储便于配对

    ValueType 存储具体值类型

*/

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
    type: StackValueType,
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
            type: StackValueType,
            next: t
        });
        self.top = newitem;
    }
    fn pop(&mut self) {
        
    }
}





