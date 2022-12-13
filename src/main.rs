use std::cmp::Ordering;
use rand::Rng;
use std::io;

fn main() {

    // 当前回合数
    let mut count = 0;

    // 需要几轮游戏
    let rounds = 10;

    // 用了存储分数
    let mut score = 0;

    // 生成两个随机数 默认值都为 0
    let mut addend: u32 = 0;
    let mut adding: u32 = 0;


    println!("😄: 让我们开始游戏吧！ 一轮游戏为10个回合，每题10分，总分100分！");

    // 这里我们使用 while 制造一个循环
    while count < rounds {

        // 开始生成题目并且出题，将随机数加起来计算总和数
        let sum = next_math(&mut addend, &mut adding);

        question(&mut addend, &mut adding, &mut count);

        // 用来存储用户控制台输入的变量
        let mut guess = String::new();

        // 从控制台上读入输入字符串
        io::stdin()
            .read_line(&mut guess)
            .expect("😠: 你能不能好好输！请输入正数！");

        // 解析输入的值
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            // 如果非法设置默认值
            Err(_) => 0,
        };

        match guess.cmp(&sum) {
            Ordering::Less => {
                println!("❌: 你的答案太小了！正确答案是 {sum}！", sum = sum);
            }
            Ordering::Greater => {
                println!("❌: 你的答案太大了！正确答案是 {sum}！", sum = sum);
            }
            Ordering::Equal => {
                score += 10;
                println!("✅: 恭喜你！答案正确！加10分！");
            }
        };

        // 添加轮数将其加一
        count += 1;
    }


    println!("🥳: 本轮游戏结束！你的分数为 {score} ！{exp}",
             score = score,
             exp = if score >= 60
             { "成绩合格！" }
             else
             { "成绩不合格！" })
}


fn next_math(addend: &mut u32, adding: &mut u32) -> u32 {
    *addend = rand::thread_rng().gen_range(1..=100);
    *adding = rand::thread_rng().gen_range(1..=100);
    // 直接返回2个数的和方便结果判断
    *addend + *adding
}

// 向用户展示计算公式要求计算结构
fn question(addend: &mut u32, adding: &mut u32, index: &mut u32) {
    println!("🤔: 第{index}题为 {addend} + {adding} = ？ 请输入正确结果？", index = *index + 1, addend = addend, adding = adding);
}