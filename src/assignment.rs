fn main() {
    let vec0 = Vec::new();
    // Problem 1: change something here to make this compile:
    let vec1 = fill_vec(vec0);
    // Do not change any lines until the next comment
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);

    let vec0 = Vec::new();
    // Problem 2: change something here to make this compile:
    let mut vec1 = fill_vec(vec0);
    // Do not change any lines until the next comment
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let vec0 = Vec::new();
    let mut vec1 = fill_vec2(&vec0);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let mut vec0 = Vec::new();
    fill_vec3(&mut vec0);
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    let vec0 = Vec::new();
    let mut vec1 = fill_vec4(vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    let mut vec1 = fill_vec5();
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec
}

// Problem 3: change something here to make this compile:
fn fill_vec2(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    // Do not change any lines until the next comment
    vec.push(22);
    vec
}

// Problem 4: change something here to make this compile:
fn fill_vec3(vec: Vec<i32>) -> Vec<i32> {
    // Do not change any lines until the next comment
    vec.push(22);
}

// Problem 5: change something here to make this compile:
fn fill_vec4(vec: Vec<i32>) -> Vec<i32> {
    // Do not change any lines until the next comment
    vec.push(22);
    vec
}

fn fill_vec5() -> Vec<i32> {
    // Problem 6: change something here to make this compile:
    let mut vec = vec;
    // Do not change any lines until the next comment
    vec.push(22);
    vec
}
