use std::io;

#[allow(dead_code)]
pub fn main() {
    // can be solved with greedy approach making sum half as both
    // array will have same len, but below is my solution i thought
    // first.
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let buffer = buffer.trim().parse::<usize>().unwrap();

    print_division(buffer);
}

fn print_division(mut input: usize) {
    let sum = (input * (input + 1)) / 2;
    if sum % 2 == 1 {
        println!("NO");
        return;
    }

    println!("YES");

    let mut vec1 = Vec::<usize>::new();
    let mut vec2 = Vec::<usize>::new();

    let mut val = 0;
    loop {
        if input == 1 {
            if vec2[vec2.len() - 1] > vec1[vec1.len() - 1] {
                vec1.push(input);
            } else {
                vec2.push(input);
            }
            break;
        }

        if val % 2 == 0 {
            vec2.push(input);
            vec1.push(input - 1);
        } else {
            vec1.push(input);
            vec2.push(input - 1);
        }
        input -= 2;

        if input == 0 {
            break;
        }

        val += 1;
    }

    print_vector(vec1);
    print_vector(vec2);
}

fn print_vector(vec: Vec<usize>) {
    let vec_len = vec.len();
    println!("{}", vec_len);

    for i in 0..vec_len {
        print!("{} ", vec[i]);
    }
    print!("\n");
}
