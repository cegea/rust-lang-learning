// fn main() {
//     let mut x = 200;
//     while x >= 10 {
//         x = x / 2;
//     }
//     println!("Final x: {x}");

//     for x in 1..=5 {
//         println!("x: {x}");
//     }

//     for elem in [1, 2, 3, 4, 5] {
//         println!("elem: {elem}");
//     }

//     let mut i = 0;
//     loop {
//         i += 1;
//         println!("{i}");
//         if i > 100 {
//             break;
//         }
//     }

//     i = 0;
//     loop {
//         i += 1;
//         if i > 5 {
//             break;
//         }
//         if i % 2 == 0 {
//             continue;
//         }
//         println!("{}", i);
//     }

// }

fn main() {
    let s = [[5, 6, 7], [8, 9, 10], [21, 15, 32]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    print!("elements searched: {elements_searched}");
}