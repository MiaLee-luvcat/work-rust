use std::env;  // Rust에서 기본으로 제공하는 std::env라는 라이브러리를 사용하겠다는 의미
// 어떤 값을 전달하고 싶을 땐 main 함수의 파라미터로 받지 않는다.

// 열거형 Result 타입을 알아야 한다.
// cargo run abc.txt target.txt 로 실행하면 되고, 실제로 파일이 없어도 된다.
// 입력된 파일명을 가지고 실제 파일을 열려고하는 작업을 하지 않기 때문이다.
// 그냥 파일명을 입력받고 그대로 출력하는 코드이기에, 실제 파일이 없어도 문제없이 동작한다.
fn main() -> Result<(), i32> {
    // env::args().collect()를 사용해서 입력값을 받는다.
    let args: Vec<String> = env::args().collect();

    // &args[1]에서 앞에 붙는 &는 벡터의 원소를 참조자 형태로 받겠다는 의미다.
    // 즉, 벡터의 1번 인덱스에 있는 원소의 주소를 가르키는 변수가 된다.
    // 주의할 것은 &args[1]이 첫 번째 입력값이라는 점이다. &args[0]에는 프로그램 이름이 들어가 있게 된다.
    let src_file = &args[1];
    let tgt_file = &args[2];

    // 메시지를 출력하고 나서 줄을 바꾸는 것은 println!, 줄 바꿈 없는 것은 print!다.
    println!("Source File: {}", src_file);
    println!("Target File: {}", tgt_file);
    // 변수없이 그냥 메시지만 출력하는 것은 println!("hello"); 혹은 줄바꿈이 없다면 print!("hello")
    print!("hello");
    println!("hello"); // 이렇게 하면 hellohello 가 나옴

    // 변수 값을 출력하는 것은 println!("a+b = {}, sum"와 같이 {}를 사용한다.
    // {}자리에 sum 값이 들어가서 a+b = 100과 같이 출력되는 것
    // println!("a+b={sum}");과 같이 중괄호 안에 직접 변수명을 넣어도 된다.
    let sum = 100;
    println!("a+b={}",sum); // a+b=100
    println!("a+b={sum}");  // a+b=100

    // 벡터와 같은 복합형 변수는 {:?}를 써야한다. println!("{:?}", v);
    let v = vec![1,2,3];  // vec!도 매크로다. 벡터를 쉽게 만들어주는 매크로다.
    println!("{:?}",v);  //[1, 2, 3]

    Ok(())
}