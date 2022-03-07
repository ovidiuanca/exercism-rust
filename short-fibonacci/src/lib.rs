/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    let empty_vector = vec![];

    empty_vector
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    let mut buffer = vec![];

    let mut index: usize = 0;

    while index < count {
        buffer.push(0);

        index += 1;
    }

    buffer
}

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut vec = vec![1, 1];
    let mut i: u8 = 1;
    let mut j: u8 = 1;

    let mut pos: u8 = 2;

    while pos < 5 {
        let new_elem: u8 = i + j;

        vec.push(new_elem);

        i = j;
        j = new_elem;

        pos += 1;
    }

    vec
}
