
use std::rc::Rc;
use std::cell::RefCell;
use std::clone::Clone;

#[derive(Debug,Clone)]
enum ValueType {
    vString(String),
    vNumber(i64),
    vObject(JsonObject),
    vArray(Vec<ValueType>),
    vTrue,
    vFalse,
    vNull,
}
/*
    对于一个Json Object的value值的类型定义
    Json Object:
    {
        "xxxx" : value,
        "xxxx" : value,
        .....
    }
*/
#[derive(Debug,Clone)]
pub struct JsonObject {
    //top:    Rc<Option<RefCell<ValueType>>>,
    //rear:   Rc<Option<RefCell<valueType>>>,
    /*
        分别指向队列的头部和尾部
    */
    json_pair_list: Vec<JsonPair>,
}
/*
    一个Json Object的定义
*/
#[derive(Debug,Clone)]
pub struct JsonPair{
    key: String,
    value: ValueType,
}
/*
    Json Object中一对值的定义
    Json Object:
    {
        JsonPair,
        JsonPair,
        ....
    }
*/
impl JsonPair {
    fn new(key: String,value: ValueType) -> JsonPair {
        JsonPair{
            key: key,
            value: value,
        }
    }
}

impl JsonObject {
    fn new() -> JsonObject { JsonObject{json_pair_list: Vec::new()} }
    fn add_json_pair(&mut self,key:String,value:ValueType) {
        let mut new_pair = JsonPair::new(key,value);
        self.json_pair_list.push(new_pair);
    }
}



// 处理Json字符串的栈结构
#[derive(Clone)]
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
    top: Rc<Option<RefCell<ValueNode>>>,
}
/*
    解析Json使用的栈定义
*/
pub struct ValueNode {
    value: StackValueType,
    next: Rc<Option<RefCell<ValueNode>>>,
}
/*
    解析Json使用的栈中元素的定义
*/
impl ValueNode {
    fn new(value: StackValueType) -> Self {
        ValueNode{ value , next: Rc::new(None) }
    }
}


impl ValueStack {
    fn new() -> Self {
        ValueStack{ top: Rc::new(None)}
    }

    fn push(&mut self, value: StackValueType) {
        let mut new_item = ValueNode::new(value);
        let stack_top = Rc::clone(&self.top);

        match *stack_top {
            None => {
                self.top = Rc::new(Some(RefCell::new(new_item)));
            },
            Some(ref value_node) => {
                new_item.next = Rc::clone(&self.top);
                //value_node.borrow_mut().next = Rc::new(Some(RefCell::new(new_item)));
                self.top = Rc::new(Some(RefCell::new(new_item)));
            },
        }
    }

    fn pop(&mut self) -> Option<StackValueType> {
        let stack_top = Rc::clone(&self.top);
        if let Some(ref refcell) = *stack_top {
            Some(refcell.borrow().value.clone())
        } else {
            None
        }
    }

}


//