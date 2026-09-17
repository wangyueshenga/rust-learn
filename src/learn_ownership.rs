#[allow(unused)]
/*
#[warn(unused)] = "我要看这类警告" → 警告出现

#[allow(unused)] 或 #[allow(dead_code)] = "别烦我" → 警告消失
unused 分组里有哪些 lint
unused 是一个分组,包含很多具体 lint:

unused_variables —— 未使用的变量

unused_imports —— 未使用的 use

dead_code —— 从未被使用的代码(就是你踩的这个)

unused_mut —— 多余的 mut

unused_must_use —— 忽略了 #[must_use] 返回值

……

所以 #[warn(unused)] = 把这一整组都设成"显示"。
*/
pub struct Node{
    value: String,
    children: Vec<Node>,
}

impl Node {
    pub fn new(value: &str) -> Self {
        Node {
            #[warn(dead_code)]
            value: value.to_string(),
            #[warn(dead_code)]
            children: Vec::new(),
        }
    }
    
    pub fn add_child(&mut self, child: Node){
        self.children.push(child);
    }
}