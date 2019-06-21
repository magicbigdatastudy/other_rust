//来自标准库的io
use std::io;
use rand::Rng;

//程序入口
fn main() {
    
    lesson06();

}

fn lesson06(){

}
fn first_word(s: &String) -> usize{
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }

    s.len()
}

//参考和引用
fn lesson05(){
    let s1 = String::from("hello");
    //请注意变量声明中的所有元组代码和函数返回值都消失了。
    // 二，请注意，我们通过&s1进入 calculate_length，
    // 并在它的定义，我们就&String不是 String。
    let len = calculate_length(&s1);
    //s->s1->堆内存
    println!("{}{}",s1,len);

    //不能直接修改不可变引用,添加mut
    let mut s1 = String::from("hello");
    test_2(&mut s1);
    // 可变引用有一个很大的限制：对于特定范围内的特定数据，
    // 只能有一个可变引用。此代码将错误
    //let r1 = &mut s1;
    //let r2 = &mut s1;
    //println!("{}, {}", r1, r2);这里会报错
    //与往常一样，我们可以使用花括号来创建一个新的范围，
    // 允许多个可变引用，而不是同时引用：
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    }
    let r2 = &mut s;

    //存在组合可变和不可变引用的类似规则，此代码导致错误
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    //println!("{},{},and {}",r1,r2,r3);

    //以下这种可以
    let mut s = String::from("hello");

    let r1 = &s;

    println!("{}",r1);

    let r2 = &mut s;

    println!("{}",r2);

    //函数不允许返回&xx类型的值，因为当执行完代码时，引用引用的变量将会
    //释放，那么返回的是一个无效的引用

    //总结
    //在任何时候，你可以拥有任何一个可变引用或任意数量不变引用。
    //引用必须始终有效。

}

fn test_3(a:i32)->&i32{
    return &a;
}


fn test_2(s:&mut String){
    s.push_str("world");
}

fn calculate_length(s:&String) -> usize{
    return s.len();
}

fn lesson04(){
    /*
    所有权的规则
    Rust中的每个值都有一个称为其所有者的变量。
    一次只能有一个所有者。
    当所有者超出范围时，该值将被删除。
    作为所有权的第一个例子，我们将研究一些变量的范围。
    范围是项目有效的程序范围。假设我们有一个看起来像这样的变量：
    */
    let s= "hello";
    //变量s引用字符串文字，其中字符串的值被硬编码到程序的文本中。
    // 变量从声明它的位置到当前范围的结尾有效。清单4-1包含注释变量s有效的注释。
    //当s谈到为范围，它是有效的。
    //它一直有效，直到它超出范围。
    let mut s = String::from("hello");

    s.push_str(",world!");

    println!("{}",s);
    //将值绑定5到x; 然后使该值的副本，x并将其绑定到y，两个5值被推入堆栈
    let x = 5;
    let y = x;

    // 而这里不是，是指向了同一块堆内存，指针指向第一个h
    // 这里复制了堆栈的指针
    let s1 = String::from("hello");
    let s2 = s1;//这里，s1将会不起作用，不再有效，为了防止两次清理雷村，造成内存损坏
    //会在超出变量的范围后调用drop函数，清理堆内存
    //这里s1无效的情况，是移动了，也就是s1移动到了s2
    println!("{}",s2);

    let s3 = s2.clone();//复制堆数据，s2依然有效

    let z = y;//堆栈数据的复制

    let s  =String::from("hello");

    takes_ownership(s);

    //takes_ownership(s) 这里会报错，因为s指向的堆内存已经清除


    let x = 5;

    makes_copy(x);

    //返回值可以转移所有权
    let mut s1 = String::from("hello");
    /*
    变量的所有权每次都遵循相同的模式：将值赋给另一个变量会移动它。
    当包含堆上数据的变量超出范围时，drop除非数据已被移动为另一个
    变量所拥有，否则将清除该值。
    */
    s1 = return_value(s1);
    println!("{}",s1);
    /*
        取得所有权，然后通过各种功能归还所有权有点单调乏味。
        如果我们想让一个函数使用一个值但不取得所有权，该怎么办？
        如果我们想要再次使用它们，除了我们可能想要返回的函数体所产生的任何数据之外，
        我们传入的任何内容都需要传回。
    */
    let str1 = String::from("hello1");

    let str2 = String::from("hello2");

    let (str1_1,str2_1) = return_value_str_str(str1,str2);

}

fn return_value_str_str(str1:String,str2:String)-> (String,String){
    (str1,str2)
}

fn return_value(str1:String) -> String{
    return str1;
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


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
    let i = loop{
        println!("again");

        if i>10{
            break i*2;//返回值
        }
        i+=1;
    };
    println!("{}",i);

    let mut number = 0;
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
        println!("{}",number);
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
