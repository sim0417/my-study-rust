struct Pug {
    name: String,
}
impl Pug {
    fn new() -> Pug {
        Pug {
            name: String::from("Pug"),
        }
    }
}

fn main() {
    let my_dog = Pug::new();

    walk_dog(&my_dog); // 소유권이 이동이 아닌 참조를 사용하여 값을 복사한다.

    println!("my dog's name is {}", my_dog.name); // 소유권이 이동되지 않았기 때문에 값을 사용할 수 있다.
}

fn walk_dog(dog: &Pug) {
    // 불변 참조를 사용하여 객체를 사용할 수 있다.
    println!("Walking {}", dog.name);
}

// fn rename_dog(dog: &mut Pug, new_name: String) {
//     // 가변 참조를 사용하여 객체를 사용할 수 있다.
//     dog.name = new_name;
// }
