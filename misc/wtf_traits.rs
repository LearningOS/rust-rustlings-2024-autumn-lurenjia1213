trait Op {
    fn play_genish(self) -> Self;
    fn download_genish()->Self;
}

struct Man; // 定义 Man 结构体
struct Woman; // 定义 Woman 结构体

impl Op for Man {
    fn play_genish(self) -> Self {
        println!("A man is op, he plays genish");
        self // 返回自身
    }
    fn download_genish()->Man{
        Man
    }
}

impl Op for Woman {
    fn play_genish(self) -> Self {
        println!("A woman is op, she plays genish");
        self // 返回自身
    }
    fn download_genish()->Woman{
        Woman
    }
}

fn main() {
    let woman = Woman::download_genish(); // 创建 Woman 实例
    woman.play_genish(); // 调用 play_genish 方法
    let man=Man;
    man.play_genish();
    
}
