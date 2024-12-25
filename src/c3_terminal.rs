


pub fn proceed<'a>() {
    println!("CLI App");
    let function: Box<dyn Fn(&str)> = Box::new( move |input: &str| {
        println!("You entered: {}", input);
    });

    let consumer: Box<dyn Fn(&str)> = Box::new( move |input: &str| {
        function(input);
    });

    let predicate: Box<dyn Fn(&str) -> bool> = Box::new( move |input: &str| {
        input.len() > 0
    });

    let supplier: Box<dyn Fn() -> String> = Box::new( move || {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    });

    let function_closure = |input: &'a str| {
        println!("You entered: {}", input);
        input
    };

    let consumer_closure = |input: &'a str| {
        function_closure(input);
    };

    let predicate_closure = |input: &'a str| {
        input.len() > 0
    };

    let supplier_closure = || {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };
}