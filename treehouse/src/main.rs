use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote {
        note: String,
    },
    Refuse,
    Probation,
}

// The form of a type in Rust are structs. NOTE: the derive(Debug) allows for debug printing with :? and :#?
#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

// The functionality of a type are defined within an impl block
impl Visitor {
    // new() is the constructor syntax within Rust
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    // &self is a reference to the struct
    fn greeting(&self) {
        match &self.action {
            VisitorAction::Accept => {
                println!("Welcome, {}!", self.name);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}.", self.name);
                }
            },
            // NOTE: Here we are destructuring note from the AcceptWithNote type that contains data
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome, {}! {}", self.name, note);
                if self.age < 21 {
                    println!("Do not serve alcohol to {}.", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member.", self.name),
            VisitorAction::Refuse => println!("No, no, no! You must leave!"),
        }
    }
}

fn ask_for_name() -> String {
    let mut name = String::new();
    
    stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    name
        .trim()
        .to_lowercase()
}

fn main() {
    let mut visitor_list = vec![
        Visitor::new("bob", VisitorAction::Accept, 45),
        // NOTE: You could also use `note: "".to_string()`. This is an example of using String::from()
        Visitor::new("sally", VisitorAction::AcceptWithNote { note: String::from("Lactose-free milk is in the fridge") }, 15),
        Visitor::new("joe", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello, what's your name? (Leave empty and press ENTER to quit)");
    
        let name = ask_for_name();
    
        let known_visitor = visitor_list
            .iter()
            .find(|v| v.name == name);
    
        match known_visitor {
            Some(visitor) => visitor.greeting(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            },
        }
    }

    println!("The final list of visitors:");
    println!("{:#?}", visitor_list);
}
