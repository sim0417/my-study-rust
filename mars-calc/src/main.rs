// 표준 라이브러리를 사용하기 위해 std 패키지를 임포트한다.
use std::io;

fn main() {
    // String 구조체를 사용하여 문자열을 생성한다. 구조체의 인스턴스를 생성하려면 ::new() 메서드를 호출한다.
    let mut input = String::new();
    // let mut copy_input = input; // 이주석을 풀면 10번라인에서 에러가 발생한다. copy_input 을 선언하면서 소유권이 이동되면서 input 은 더이상 사용할 수 없다.

    // &mut 는 소유권을 빌려오는 것이다.
    io::stdin().read_line(&mut input);

    // rust는 아래의 세가지 소유권 규칙을 따른다.
    // 1. 러스트에서 각가의 값은 변수가 소유한다. (Each value in Rust has an owner)
    // 2. 소유자가 범위를 벗어나면 그 값은 해제된다. (When the owner goes out of scope, the value will be deallocated)
    // 3. 한번에 하나의 소유자만 존재할 수 있다. (There can only be One owner at a time)

    // 소유권 참조 케이스
    // 참조는 소유권을 빌려오는 것이다. 참조를 하는 경우는 immutable 참조와 mutable 참조가 있다.
    // immutable 참조는 데이터를 읽기만 할 수 있고 mutable 참조는 데이터를 읽고 쓸 수 있다.
    let copy_1 = &mut input;
    println!("{}", copy_1); // 이 시점에서 copy_1의 lifetime은 끝나서 copy_2 가 소유권을 빌려올 수 있다.
    // Non-Lexical Lifetimes. Rust 2018부터 reference의 lifetime이 실제 사용되는 범위까지로 제한된다.
    // 스코프 끝까지가 아니라 마지막 사용 지점까지 참조할 수 있다.

    let copy_2 = &input;
    println!("{}", copy_2);

    // 소유권 참조 에러 케이스
    // let copy_3 = &mut input;
    // let copy_4 = &mut input;
    // 하나의 mutable reference 만 존재 가능하기 때문에 에러가 발생한다.
    // 또한 mutable reference와 immutable reference는 동시에 존재할 수 없는 규칙이 있기 때문에 에러가 발생한다.
    // 위 규칙으로 데이터레이스 문제를 방지할 수 있다.
    // println!("{} {}", copy_3, copy_4);

    // 소유권 참조 에러가 아닌 경우.
    let copy_5 = &input;
    let copy_6 = &input;
    // 에러가 나지 않는 이유는 immutable 참조는 여러개 존재할 수 있기 때문이다.
    println!("this is the end {} {}", copy_5, copy_6);

    // 느낌표가 있는 것은 매크로 함수이다.
    // 매크로 함수는 다수의 인수를 받을 수 있다.
    // cargo-expand 패키지를 통해 매크로 함수의 코드를 확인할 수 있다.
    println!("Hello, mars-calc!");

    // 변수는 let 키워드를 사용한다.
    // 컴파일러가 자동으로 변수의 유형을 추론하지만 명시적으로 유형을 지정할 수 있다.
    let name = "Hugo";
    let weight: f32 = 80.5;

    // 변수는 기본적으로 불변이다. 변경하려면 mut 키워드를 사용해야 한다.
    let mut mars_weight = calculate_weight_on_mars(weight);

    println!("your weight is {}kg and your name is {}", weight, name);

    println!("your weight on mars is {}kg", mars_weight);

    mars_weight = mars_weight * 1000.0;

    println!("also your weight on mars is {}g", mars_weight);
}

// 함수의 매개변수, 반환값의 유형을 명시해야 한다. 그것이 rust 세계의 규칙.
// 모든 함수의 마지막 표현식에서 끝에 세미콜론을 붙이지 않는다.
fn calculate_weight_on_mars(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

// mutable reference 를 사용하는 예시.
fn some_fn(s: &mut String) {
    s.push_str("hello");
}
