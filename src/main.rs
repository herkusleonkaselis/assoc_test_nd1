use std::{cell::Cell, collections::HashMap};

fn main() {
    let cayley: HashMap<u32, HashMap<u32, u32>> = HashMap::from([
        (0, HashMap::from([(0, 2), (1, 0), (2, 2), (3, 0), (4, 4)])),
        (1, HashMap::from([(0, 0), (1, 1), (2, 2), (3, 3), (4, 4)])),
        (2, HashMap::from([(0, 2), (1, 2), (2, 2), (3, 2), (4, 4)])),
        (3, HashMap::from([(0, 0), (1, 3), (2, 2), (3, 3), (4, 4)])),
        (4, HashMap::from([(0, 4), (1, 4), (2, 4), (3, 4), (4, 4)])),
    ]);

    let op = |a: u32, b: u32| -> u32 { *cayley.get(&a).unwrap().get(&b).unwrap() };
    (0..=4).for_each(|a: u32| {
        println!("> a={a}");
        let mut cayley_dot: Vec<Cell<Vec<u32>>> = Vec::with_capacity(5);
        (0..=4).for_each(|_| {
            let mut v: Vec<u32> = Vec::with_capacity(5);
            (0..=4).for_each(|_| v.push(0));
            cayley_dot.push(Cell::new(v));
        });
        let mut cayley_circle: Vec<Cell<Vec<u32>>> = Vec::with_capacity(5);
        (0..=4).for_each(|_| {
            let mut v: Vec<u32> = Vec::with_capacity(5);
            (0..=4).for_each(|_| v.push(0));
            cayley_circle.push(Cell::new(v));
        });
        (0..=4).for_each(|x: u32| {
            let row_dot = &mut cayley_dot[x as usize]; 
            let row_circle  = &mut cayley_circle[x as usize];
            (0..=4).for_each(|y: u32| {
                let dot = op(x, op(a, y));
                let circle = op(op(x, a), y);
                row_dot.get_mut()[y as usize] = dot;
                row_circle.get_mut()[y as usize] = circle;

                if dot != circle {
                    println!("Dot and circle functions derived from Cayley produce a mismatch. Operation is not associative!");
                    return;
                }

                println!("x={x}, y={y} -> associative connection");
            });
        });

        print!("Cayley for dot operation of a={a}\n");
        for x in cayley_dot {
            print!("\n");
            for y in x.take() {
                print!("{y} ")
            }
        }
        print!("\n");
        
        print!("Cayley for circle operation of a={a}\n");
        for x in cayley_circle {
            print!("\n");
            for y in x.take() {
                print!("{y} ")
            }
        }
        print!("\n");
    });

    println!("Operation succeeded. Operation is associative.");
}
