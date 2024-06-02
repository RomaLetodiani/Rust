fn main() {

    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("{:?}", vec);

    let mut vec2 = vec![4, 5, 6];

    println!("First Element of vec2 {:?}", vec2[0]);

    for i in &vec {
        println!("{}", i);
    }

    for i in &mut vec2 {
        *i += 50;
    }

    if let Some(third) = vec.get(2) {
        println!("Third element of vec: {}", third);
    }

    if let Some(fourth) = vec.get(3) {
        println!("Fourth element of vec: {}", fourth);
    } else {
        println!("No fourth element in vec");
    }

    let slice = &vec2[1..3];
    println!("Slice of &vec2[1..3]: {:?}", slice); // [55, 56]

    let slice = &vec2[1..];
    println!("Slice of &vec2[1..]: {:?}", slice); // [55, 56]

    let slice = &vec2[..2];
    println!("Slice of &vec2[..2]: {:?}", slice); // [54, 55]

    let slice = &vec2[..];
    println!("Slice of &vec2[..]: {:?}", slice); // [54, 55, 56]

    let slice = &vec2;
    println!("Slice of &vec2: {:?}", slice); // [54, 55, 56]

    let slice = &vec2[1..=2];
    println!("Slice &vec2[1..=2]: {:?}", slice); // [55, 56]

    // let slice = &vec2[1..=3];
    // println!("Slice &vec2[1..=3]: {:?}", slice); // Panic

    // pop returns an Option and removes the last element from the vector
    if let Some(last) = vec2.pop() {
        println!("Last element of vec2: {}", last);
    }

    println!("vec2: {:?}", vec2);
}
