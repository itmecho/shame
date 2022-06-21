const K: [u32; 64] = [
    0b01000010100010100010111110011000,
    0b01110001001101110100010010010001,
    0b10110101110000001111101111001111,
    0b11101001101101011101101110100101,
    0b00111001010101101100001001011011,
    0b01011001111100010001000111110001,
    0b10010010001111111000001010100100,
    0b10101011000111000101111011010101,
    0b11011000000001111010101010011000,
    0b00010010100000110101101100000001,
    0b00100100001100011000010110111110,
    0b01010101000011000111110111000011,
    0b01110010101111100101110101110100,
    0b10000000110111101011000111111110,
    0b10011011110111000000011010100111,
    0b11000001100110111111000101110100,
    0b11100100100110110110100111000001,
    0b11101111101111100100011110000110,
    0b00001111110000011001110111000110,
    0b00100100000011001010000111001100,
    0b00101101111010010010110001101111,
    0b01001010011101001000010010101010,
    0b01011100101100001010100111011100,
    0b01110110111110011000100011011010,
    0b10011000001111100101000101010010,
    0b10101000001100011100011001101101,
    0b10110000000000110010011111001000,
    0b10111111010110010111111111000111,
    0b11000110111000000000101111110011,
    0b11010101101001111001000101000111,
    0b00000110110010100110001101010001,
    0b00010100001010010010100101100111,
    0b00100111101101110000101010000101,
    0b00101110000110110010000100111000,
    0b01001101001011000110110111111100,
    0b01010011001110000000110100010011,
    0b01100101000010100111001101010100,
    0b01110110011010100000101010111011,
    0b10000001110000101100100100101110,
    0b10010010011100100010110010000101,
    0b10100010101111111110100010100001,
    0b10101000000110100110011001001011,
    0b11000010010010111000101101110000,
    0b11000111011011000101000110100011,
    0b11010001100100101110100000011001,
    0b11010110100110010000011000100100,
    0b11110100000011100011010110000101,
    0b00010000011010101010000001110000,
    0b00011001101001001100000100010110,
    0b00011110001101110110110000001000,
    0b00100111010010000111011101001100,
    0b00110100101100001011110010110101,
    0b00111001000111000000110010110011,
    0b01001110110110001010101001001010,
    0b01011011100111001100101001001111,
    0b01101000001011100110111111110011,
    0b01110100100011111000001011101110,
    0b01111000101001010110001101101111,
    0b10000100110010000111100000010100,
    0b10001100110001110000001000001000,
    0b10010000101111101111111111111010,
    0b10100100010100000110110011101011,
    0b10111110111110011010001111110111,
    0b11000110011100010111100011110010,
];

pub fn generate_hash<T: AsRef<[u8]>>(input: T) {
    // Initialise hash values
    let mut h0: u32 = 0b01101010000010011110011001100111;
    let mut h1: u32 = 0b10111011011001111010111010000101;
    let mut h2: u32 = 0b00111100011011101111001101110010;
    let mut h3: u32 = 0b10100101010011111111010100111010;
    let mut h4: u32 = 0b01010001000011100101001001111111;
    let mut h5: u32 = 0b10011011000001010110100010001100;
    let mut h6: u32 = 0b00011111100000111101100110101011;
    let mut h7: u32 = 0b01011011111000001100110100011001;

    // https://sha256algorithm.com/
    // Step 1
    // 64 bytes in a 512 message block
    // last 8 bytes are the length of the original message as big endian
    let input = input.as_ref();

    // Store original bit length of data
    let data_len = input.len();
    // Calculate the length in bits
    let data_len_bits = (data_len as u64) * 8;

    let mut data = input.to_owned();

    // Append 1
    data.push(1 << 7);

    // 56 is 64 bytes - the 8 length bytes

    let diff = 64 - ((data.len() + 8) % 64);
    // Add padding to reach 64 bytes
    if diff > 0 {
        let padding = vec![0; diff];
        data.extend(padding);
    }

    // Is be OK here?
    data_len_bits.to_be_bytes().map(|b| data.push(b));

    // Step 2
    let chunks = data.chunks(64);

    for chunk in chunks {
        let mut w = [0u32; 64];
        for (i, word) in chunk.chunks(4).enumerate() {
            let padded_word = match *word {
                [a, b, c, d] => [a, b, c, d],
                [a, b, c] => [a, b, c, 0],
                [a, b] => [a, b, 0, 0],
                [a] => [a, 0, 0, 0],
                _ => [0, 0, 0, 0],
            };
            w[i] = u32::from_be_bytes(padded_word);
        }
        for i in 16..=63 {
            let a0_idx = i - 15;
            let a0 = (w[a0_idx].rotate_right(7)) ^ (w[a0_idx].rotate_right(18)) ^ (w[a0_idx] >> 3);

            let a1_idx = i - 2;
            let a1 =
                (w[a1_idx].rotate_right(17)) ^ (w[a1_idx].rotate_right(19)) ^ (w[a1_idx] >> 10);

            w[i] = w[i - 16]
                .wrapping_add(a0)
                .wrapping_add(w[i - 7])
                .wrapping_add(a1);
        }

        // correct up to here
        // print_schedule(w.as_slice());

        let mut a = h0;
        let mut b = h1;
        let mut c = h2;
        let mut d = h3;
        let mut e = h4;
        let mut f = h5;
        let mut g = h6;
        let mut h = h7;

        for (idx, word) in w.into_iter().enumerate() {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ ((!e) & g);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);

            let tmp1 = h
                .wrapping_add(sum1)
                .wrapping_add(choice)
                .wrapping_add(K[idx]) //todo k consts
                .wrapping_add(word);

            let tmp2 = sum0.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(tmp1);
            d = c;
            c = b;
            b = a;
            a = tmp1.wrapping_add(tmp2);
        }

        h0 = h0.wrapping_add(a);
        h1 = h1.wrapping_add(b);
        h2 = h2.wrapping_add(c);
        h3 = h3.wrapping_add(d);
        h4 = h4.wrapping_add(e);
        h5 = h5.wrapping_add(f);
        h6 = h6.wrapping_add(g);
        h7 = h7.wrapping_add(h);
    }

    println!(
        "{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}",
        h0, h1, h2, h3, h4, h5, h6, h7
    );
}

#[allow(dead_code)]
fn print_schedule(s: &[u32]) {
    for (i, w) in s.iter().enumerate() {
        println!("w{} {:0>32b}", i, w);
    }
}

#[allow(dead_code)]
fn print_bin(name: &str, i: u32) {
    println!("{} {:0>32b}", name, i);
}
