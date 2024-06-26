// Non Valid
fn main() {
    let r;            // ---------+-- 'a
                            //          |
    {                       //          |
        let x = 5;     // -+-- 'b  |
        r = &x;             //  |       |
    }                       // -+       |
                            //          |
    println!("r: {r}");     //          |
}                           // ---------+

// Valid
fn main_2() {
    let x = 5;         // ----------+-- 'b
                            //           |
    let r = &x;       // --+-- 'a  |
                            //   |       |
    println!("r: {r}");     //   |       |
                            // --+       |
}                           // ----------+