#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod vstack;  //解析器使用的栈定义
mod jparser; //解析器状态机
mod jobject; //解析后生成的Json Object对象



impl PlainReader {
    pub fn from(input: &str) -> PlainReader {
        let input = input.to_string();
        PlainReader{
            plain_text: input.clone()
        }
    }
    pub fn raw_text(&self) -> String {
        self.plain_text.clone()
    }
}

