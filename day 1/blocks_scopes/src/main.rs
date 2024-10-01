// fn main() {
//     let z = 13;
//     let x = {
//         let y = 10;
//         println!("y: {y}");
//         z - y
//     };
//     println!("x: {x}");
// }

fn main() {
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}