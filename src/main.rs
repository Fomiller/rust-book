enum IsOld {
    Yes(u32),
    No,
}

fn main() {
    let short = "Bob";
    let mut my_struct: MyStruct;
    {
        let long = "Forrest";
        let s = longest(short, long);
        hello_world(s);
        my_struct = MyStruct::new(String::from("Forrest"), 28);
        println!("{:#?}", my_struct);
        println!("name = {}", my_struct.name);
        println!("age = {}", my_struct.age);
        my_struct.have_birthday();
        println!("age = {}", my_struct.age);
    }
    match my_struct.is_old() {
        IsOld::Yes(age) => println!("Lmao you are {} years old!", age),
        IsOld::No => println!("Nice you still got it."),
    }
    my_struct.set_favorite_animal(String::from("Chuck"));
    if let Some(animal) = my_struct.favorite_animal() {
        println!("{} favorite animal is {}", my_struct.name, animal);
    } else {
        println!("{} has no favorite animal", my_struct.name)
    }
}

fn hello_world(s: &str) {
    println!("hello, {}", s)
}

fn longest<'a>(s: &'a str, l: &'a str) -> &'a str {
    if s.len() > s.len() {
        return s;
    } else {
        return l;
    }
}

#[derive(Debug)]
struct MyStruct {
    name: String,
    age: u32,
    favorite_animal: Option<String>,
}

impl MyStruct {
    fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            favorite_animal: None,
        }
    }

    fn have_birthday(&mut self) {
        self.age += 1;
    }

    fn is_old(&self) -> IsOld {
        if self.age > 30 {
            IsOld::Yes(self.age)
        } else {
            IsOld::No
        }
    }

    fn favorite_animal(&self) -> Option<&String> {
        self.favorite_animal.as_ref()
    }

    fn set_favorite_animal(&mut self, animal: String) {
        self.favorite_animal = Some(animal);
    }
}
