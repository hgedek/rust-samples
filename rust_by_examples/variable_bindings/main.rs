fn main() {
    let immutable_var = 1;
    let mut var = 1;

    println!("{}", immutable_var);
    println!("{}", var);
    var = 2;
    println!("{}", var);

    let decl_first_var;

    {
        let x = 2;
        decl_first_var = x * x;
    }

    println!("{}", decl_first_var);

    // frozen
    let mut _frozen = 0;
    {
        // this will not change 
        // same name 
        let _frozen = _frozen;
        // frozen = 1;
    }

    _frozen = 2;
}
