fn main() {
    let number = Some(13);
    /*
    If Some does not = 13, it does nothing

    match number {
        Some(13) => println!("thirteen"),
        _ => ()
    }

    There is a simpler way to write expressions with a single condition... if let
    */

    if let Some(13) = number {
        println!("thirteen");
    }
}
