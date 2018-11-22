

pub enum StateType {
    Except_Json_Object, 
    /*
        准备接受一个Json对象,
        { => 跳转 Except_String,准备接受一个字符串作为key值
            将 { 压入栈 并且赋值
    */
    Except_String,
    /*
        准备接受一个字符串
        " => 跳转至Inside_String,处于字符串的内部
                将 " 压入栈中 设为 None 
    */
    Inside_String,
    /*
        处于一个字符串的内部
        公共计数器 栈顶 " 内 开始计数

        a-z => Current_String += [a-z]
        A-Z => Current_String += [A-Z]
        " " => Current_String += " "
                跳转至Inside_String
        "   => 结束计数，生成字符串
                弹出栈顶 " 生成字符串
                检查栈顶
                { => 说明为key值，存留于栈中
                        => Empty
                : => 说明为value值，栈顶3个元素应该分别为
                        1(:) 2(<String>) 3({)
                        弹出1,与2合并生成JsonPair并弹出
                        3接受JsonPair
                        操作完毕后，栈顶为 {
                            => Empty
                , => 说明是一个单独的value值，栈顶元素应该为
                    1(,) 2([)
                    弹出栈顶元素,将字符串插入2([)的Array中
                        => Excpet_Value
                [ => 说明是一个单独的value值，插入栈顶的 [ Array 中
                        => Excpet_Value


    */
    Empty,
    /*
        状态机初始状态
        , =>  检查栈顶
                { => , 压入栈
                        { 中有JsonPair => Except_Value
                [ => , 压入栈
                        [ 中有value => Except_Value
        ] =>  检查栈顶
            [ => Array 结束 生成 Array 弹出
                 检查栈顶：
                 ：=>   说明为value值，栈顶3个元素应该分别为
                        1(:) 2(<String>) 3({)
                        弹出1,与2合并生成JsonPair并弹出
                        3接受JsonPair
                        操作完毕后，栈顶为 {
                            => Empty
                 , =>   说明是一个单独的value值，栈顶元素应该为
                        1(,) 2([)
                        弹出栈顶元素,插入2([)的Array中
                            => Excpet_Value
        } =>  检查栈顶
                { => 生成Json_Object 弹出
                     检查栈顶
                       : => 说明为value值，栈顶3个元素应该分别为
                            1(:) 2(<String>) 3({)
                            弹出1,与2合并生成JsonPair并弹出
                            3接受JsonPair
                            操作完毕后，栈顶为 {
                                => Empty
                       [    => 
                       ,    =>
                            说明为Array中的值，处理压入
                                => Empty
                        空  => 
                            Json处理结束，返回Json Object
        : =>  压入栈中，等待后面value
        
        { =>  
    */

    Except_Value,
    /*
        准备接受一个value
        " => Except_String
        [ => 将 [ 压入栈设为None，跳转 Except_Value
        { => 将 { 压入栈设为None, 跳转 Except_String 

    */
}