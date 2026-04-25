use std::io;

#[allow(dead_code)]
pub fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<usize>().unwrap();

    let mut grid = vec![vec![0; n]; n];
    for i in 0..n {
        grid[0][i] = i;
        grid[i][0] = i;
    }

    for i in 1..n {
        for j in 1..n {
            if i == j {
                grid[i][j] = 0;
                continue;
            }

            // println!("rc -> r{} c{}", i, j);

            let mut left_vals = Vec::<usize>::new();
            let mut up_vals = Vec::<usize>::new();

            for lv in 0..j {
                // println!("lv -> {}", grid[i][lv]);
                left_vals.push(grid[i][lv]);
            }
            for uv in 0..i {
                // println!("uv -> {}", grid[uv][j]);
                up_vals.push(grid[uv][j]);
            }

            let mut up_left_joint = Vec::<usize>::new();

            for &x in &left_vals {
                up_left_joint.push(x);
            }
            for &x in &up_vals {
                up_left_joint.push(x);
            }

            up_left_joint.sort_unstable();
            // println!("join -> {:?}", up_left_joint);

            let mut val = 0;

            if *up_left_joint.get(0).unwrap() == 0 {
                let l = up_left_joint.len();
                for i in 0..up_left_joint.len() - 2 {
                    if up_left_joint
                        .get(i)
                        .unwrap()
                        .abs_diff(*up_left_joint.get(i + 1).unwrap())
                        != 1
                    {
                        val = *up_left_joint.get(i).unwrap() + 1;
                        break;
                    } else {
                        val = *up_left_joint.get(l - 1).unwrap() + 1;
                    }
                }
            }

            // println!("val -> {}", val);

            grid[i][j] = val;
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", grid[i][j]);
        }
        println!();
    }
}
