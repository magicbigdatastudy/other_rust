//来自标准库的io
use std::io;
use rand::Rng;


//程序入口
fn main() {

lesson03();

}


fn lesson03(){
    let number = 3;

    if number<5{
        println!("true");
    }else{
        println!("false");
    }

    let number = 6;

    if number%4==0{
        println!("4");
    }else if number%3==0{
        println!("3")
    }

    //if的let语句，if支持有返回值

    let _b = true;

    let number = if _b{
        5
    }else{
        6
    };
    //类型必须相同

    //循环
    //永远死循环，直到手动退出
    let mut i = 0;
    loop{
        println!("again");

        if i>10{
            break i*2;//返回值
        }
        i+=1;
    }
    println!(i);

    let number = 0;
    //有条件while
    while number>10{
        number+=1;
    }

    //for循环
    let a = [10,20,30,40,50];

    for element in a.iter(){
        println!("{}",element);
    }
    //1-4反转
    for number in (1..4).rev(){
        println!(number);
    }



}
//Rust代码使用snake case作为函数和变量名的常规样式。
// 所有字母都是小写字母，并且下划线分隔单词。这是一个包含示例函数定义的程序：
//函数
fn lesson02(){
    first_function();
    second_function(2);
    third_function(3,4);

    let x = 5;
    // 是一个块，在这种情况下，评估为4。该值y 作为let声明的一部分受到约束。
    // 注意最后x + 1没有分号的行，这与你到目前为止看到的大多数行不同。
    // 表达式不包括结尾分号。如果在表达式的末尾添加分号，
    // 则将其转换为语句，然后不返回值。在您接下来探索函数返回值和表达式时，请记住这一点。
    let y = {
        let x = 3;
        x+1
    };
    println!("{}",y);//4
    println!("{}",five());
}
//具有返回值的函数
//我们没有命名返回值，但是我们在箭头（->）之后声明它们的类型。
fn five() -> i32{
    5
}

fn plus_one(x:i32) -> i32{
    //x+1; 改语句直接返回此会报错,默认是空元组类型()
    // ,而不是i32类型，x+1才是i32，而x+1;不是
    x+1
}

fn first_function(){
    println!("Another");
}

fn second_function(x:i32){
    println!("{}",x);
}

fn third_function(x:i32,y:i64){
    println!("{}",x);
    println!("{}",y);
}



//变量和数据类型
fn lesson01(){
    //变量默认不可变
    let num1 = 1;
    //可变的变量
    let mut num2 = 2;

    println!("{}",num2);

    num2 = 12;

    println!("{}",num2);

    //常量，常量和不可变变量不同，是绑定了一个值

    const MAX_POINTS:u32 = 100_000;

    //可以声明一个与前一个变量同名的新变量

    //第一个变量会被第二个变量遮蔽
    /*
    和mut不同：
    可以更改值的类型！
    */
    let x =5;

    let x = x+1;//6

    let x = x*2;//12

    let x = "11";

    println!("The value of x is {}",x);

    /*数据类型*/
    let guess: u32 = "42".parse().expect("Not a Number");

    println!("{}",guess);

    /*
    每个变体可以是有符号或无符号的，
    并且具有明确的大小。 签名和未签名是
    指数字是否可能为负数或正数 - 换句话说，
    数字是否需要与其签名（签名）或者是否只有
    正数，因此可以表示没有符号（无符号）。
    这就像在纸上写数字：当符号重要时，
    数字会带有加号或减号; 然而，当假设数字
    为正数是安全的时，它显示没有任何迹象。
    使用二进制补码表示存储有符号数。
    您可以使用表3-2中所示的任何表单编写整数文字。请注意，
    除字节文字之外的所有数字文字都允许使用类型后缀，例如 57u8
    ，以及_作为可视分隔符，例如1_000。
        长度	签	无符号
        8位	i8	u8
        16位	i16	u16
        32位	i32	u32
        64位	i64	u64
        128位	i128	u128
        -	isize	usize
        isize和usize类型取决于运行程序的计算机类型：
        64位（如果您使用的是64位体系结构）和32位（如果您使用的是32位体系结构）。
    整数类型默认为i32：这种类型通常是最快的，即使在64位系统上也是如此。
    您使用isize或usize在索引某种集合时的主要情况。
    */

    let a1 = 0;
    let a2 = 0x0000_0001;
    let a3 = 64i64;
    let a4 = 1000_100;
    let a5 = -1009209222;

    /*
        Rust还有两种原始类型的浮点数，它们是带小数点的数字。Rust的浮点类型是f32和f64，
        分别是32位和64位。默认类型是f64 因为在现代CPU上它的速度大致相同f32但能够更精确。
    */
    let x = 2.0;

    let y:f32 = 3.0;
    //浮点数根据IEEE-754标准表示，f32类型是单精度浮点数，f64具有双精度

    /*
        数字运算
    */
    let sum = 5+10;
    let difference = 99.5 - 4.3;
    let product = 4*30;

    let quotient = 56.7/32.2;

    let remainder = 43 % 5;

    //布尔类型
    let t = true;
    let f:bool = false;

    //字符类型

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    //元组
    let tup:(i32,f64,u8) = (500,6.4,1);
    let tup1 = (500,1,6.4);
    let (x,y,z) = tup1;//解构

    println!("{}",x);

    let t1 = tup1.0;

    let t2 = tup1.1;

    //数组类型,数组必须相同类型
    let a = [1,2,3,4,5];

    let months:[&str;12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    //数组是在堆栈上分配的单个内存块

    let first = months[0];

    let second = months[1];

    // 数组越界时 编译没有产生任何错误，但程序导致 运行时错误，并且没有成功退出。当您尝试使用索引访问元素时，
    // Rust将检查您指定的索引是否小于数组长度。如果索引大于或等于数组长度，Rust将会发生混乱。


}

fn start(){
    // !是个宏定义
    println!("Guess the number!");
    /*
        该行中的::语法::new表示该类型new的关联函数String。
        在这种情况下String，相关函数在类型上实现，
        而不是在a的特定实例上实现String。
        有些语言称之为静态方法。
        此new函数创建一个新的空字符串。你会new在很多类型上找到一个函数，
        因为它是一个函数的通用名称，可以创建某种新值。
    */
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("you guessed:{}",guess);
    //随机数
    let guessed:i32 = rand::random();
    println!("{}",guessed);

    //声明一个新变量,mut可变，默认不可变
    let mut a = 0;
    a=1;

}
