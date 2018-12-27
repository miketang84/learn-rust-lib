use rand::seq::SliceRandom;


fn main () {
    let mut rng = &mut rand::thread_rng();
    let sample = "Hello, audience!".as_bytes();

    // collect the results into a vector:
    let v: Vec<u8> = sample.choose_multiple(&mut rng, 3).cloned().collect();
    println!("{:?}", String::from_utf8(v));

    // store in a buffer:
    let mut buf = [0u8; 5];
    for (b, slot) in sample.choose_multiple(&mut rng, buf.len()).zip(buf.iter_mut()) {
        *slot = *b;
    }
    println!("{:?}", String::from_utf8(buf.to_vec()));

}
