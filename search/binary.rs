use std::env;

// Simple binary search on i32 arrays.
// Simple function signature is:
// fn name(arg1: type1, arg2: type2) -> return_type
fn binary_search(a: &[i32], x: i32) -> i32 {
    // By default all variables are non mutable.
    let mut min: i32 = 0;
    // rust indexes are of type usize , a platform dependent integer.
    // so it may need to be converted to i32 in some cases. I could
    // directly use usize as well instead of i32. (See fn below).
    let mut max = a.len() as i32 - 1;
    while min < max {
        let mid = (min + max) / 2;
        // Or let t = mid as i32;
        let t: i32 = a[mid as usize];
        if x < t {
            max = mid - 1;
        } else if x > t {
            min = mid + 1;
        } else {
            return mid;
        }
    }
    return -min - 1;
}

// Same with usize used as indexes instead of i32
// Add directive below rust compiler to not complain.
// This is probably the better way to write this function.
#[allow(dead_code)]
fn binary_search_usize(a: &[i32], x: i32) -> i32 {
    let mut min: usize = 0;
    let mut max = a.len() - 1; // max is usize as well now.
    while min < max {
        let mid = (min + max) / 2;
        // Or let t = mid as i32;
        if x < a[mid] {
            max = mid - 1;
        } else if x > a[mid] {
            min = mid + 1;
        } else {
            return mid as i32;
        }
    }
    return -(min as i32) - 1;
}

fn main() {
    // Get args. First argument is always program name so we skip it.
    let args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Please provide x as an argument.");
        ::std::process::exit(0);
    }
    // Accessing vectors is tricky. &args[0] returns a
    // &String (reference to a String struct) Acceesing with args.get(0)
    // Returns an Option<&String> There is no null (I think)
    // Because it returns a reference, if we want to have the int we use a *
    // At the beginning. A slightly cleaner way would be
    // let x = &args[1].parse::<i32>().unwrap(); and use binary_search(&a, *x);
    let x: i32 = *(&args[0].parse().unwrap());
    let a: [i32; 3] = [1, 2, 3];
    let i = binary_search(&a, x);
    println!("index: {}", i);
}
