

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
#[derive(Debug)]
pub struct JsonObject {
    top: Option<Box<JsonPair>>,
    /*
        拥有队列头部JsonPair的对象
    */
}
/*
    一个Json Object的定义
*/
#[derive(Debug)]
pub struct JsonPair{
    key: String,
    value: ValueType,
    next: Option<Box<JsonPair>>,
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
            next: None,
        }
    }
}

impl JsonObject {
    fn new() -> JsonObject { JsonObject{top:None,rear:None} }
    fn add_json_pair(&mut self,key:String,value:ValueType) {
        let mut new_pair = JsonPair::new(key,value);
        let mut stack_top = self.top.take();
        match stack_top {
            None => {
               stack_top = Some(Box::new(new_pair));
                /*
                    队列为空的情况
                */
            },
            Some(_) => {
                new_pair.next = stack_top.take();
                stack_top = Some(Box::new(new_pair));
            }
        }
        self.top = stack_top.take();
    }
}