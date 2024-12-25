

pub fn proceed() {
    let calculation_function = |arg1: i32| {
        let closure = move |arg2: i32| {
            arg1 + arg2
        };
        closure
    };

    let arg2 = 2;

    let result = (calculation_function)(arg2)(3);
    println!("Result: {}", result);
}