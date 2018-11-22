enum ValueType {
    vString(String),
    vNumber(i64),
    vObject(JsonObject),
    vArray(Option<Box<ValueType>>),
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

pub struct JsonObject {
    top: Option<Box<JsonPair>>,
}
/*
    一个Json Object的定义
*/
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