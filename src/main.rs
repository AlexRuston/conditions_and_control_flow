fn main() {
    
    // conditions

    // returns true
    let cond = 2 <= 3;
    println!("{}", cond);

    // below will error because they're different types
    /*
        let cond2 = 2 <= 2.2;
     */

     // instead we could write
    let cond2 = (2 as f32) <= 2.2;
    println!("{}", cond2);


    // compound conditions

    // && -> and
    // check multiple variables for bool condition
    // returns true
    let cond3 = true && cond;
    println!("{}", cond3);
    
    // returns false 
    let cond4 = false && cond;
    println!("{}", cond4);

    // || -> or
    let cond5 = false || cond;
    println!("{}", cond5);

    // ! -> not
    let cond6 = false && !cond;
    println!("{}", cond6);

    
    // control flow
    let food = "cookie";
    let is_cookie: bool;

    if food == "cookie" {
        is_cookie = true;
    }else {
        is_cookie = false;
    }

    println!("{}", is_cookie);

}

