use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    let mut soldiers = 41;

    if args.len() > 1 {
        soldiers = args[1].parse().unwrap();
    }

    let mut murder: Vec<usize> = Vec::new();

    let mut ptr : usize = 0;

    for i in 0..soldiers {
        murder.push(i+1);
    }

    loop {
        if murder.len() == 2 {
            break;
        }

        if ptr+1 < murder.len() {
            // println!("Killed {}",murder[ptr+1]);
            murder.remove(ptr+1);
            ptr += 1;
            if ptr == murder.len() {
                ptr = 0;
            }
        } else if ptr == murder.len() {
            // println!("Killed {}",murder[murder.len() -1]);
            murder.pop();
            ptr = 0;
        } else {
            // println!("Killed {}",murder[0]);
            murder.remove(0);
            ptr = 0;
        }

    }

    println!("{:?}",murder);


}
