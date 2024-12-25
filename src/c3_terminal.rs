


pub fn proceed() {
    println!("CLI App");
    let function: dyn Fn(&str) -> () = |input: &str| {
        println!("You entered: {}", input);
    };

    let consumer: dyn Fn(&str) -> () = |input: &str| {
        function(input);
    };

    let predicate: dyn Fn(&str) -> bool = |input: &str| {
        input.len() > 0
    };

    let supplier: dyn Fn() -> String = || {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };

    let function_closure = |input: &str| {
        println!("You entered: {}", input);
        input
    };

    let consumer_closure = |input: &str| {
        function_closure(input);
    };

    let predicate_closure = |input: &str| {
        input.len() > 0
    };

    let supplier_closure = || {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    };
}