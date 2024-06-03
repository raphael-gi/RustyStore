use std::u8;

#[cfg(test)]
mod tests;

pub fn base64_encode(input: String) -> String {
    let bits = get_bits(input);

    let mut res: Vec<char> = Vec::new();
    for i in 0..bits.len() / 6 {
        let slice = &bits[i*6..i*6+6];
        let base64_idx = u8::from_str_radix(slice, 2).expect("Not a binary");
        res.push(get_base_64_character(base64_idx.into()));
    }

    res.into_iter().collect::<String>()
}

fn get_bits(input: String) -> String {
    let mut bits = input.into_bytes().into_iter().map(|x| {
        format!("{x:08b}")
    }).collect::<String>();

    to_sixes(&mut bits);
    bits
}

fn to_sixes(bits: &mut String) {
    let input_length = bits.len();
    let mut counter = 0;
    while (input_length + counter) % 6 != 0 {
        *bits += "0";
        counter += 1;
    }
}

pub fn base64_decode(encoded: String) -> String {
    let binaries = encoded.chars().into_iter().map(|character| {
        let idx = get_base_64_index(character).unwrap();
        format!("{idx:06b}")
    }).collect::<String>();

    let mut bits: Vec<u8> = Vec::new();

    for i in 0..binaries.len() / 8 {
        let slice = &binaries[i*8..i*8+8];
        let binary_character = u8::from_str_radix(slice, 2).expect("Not a binary");
        bits.push(binary_character);
    }

    String::from_utf8(bits).unwrap()
}

fn get_base_64_character(idx: usize) -> char {
    get_alphabet()[idx]
}

fn get_base_64_index(character: char) -> Result<usize, isize> {
    for (i, c) in get_alphabet().iter().enumerate() {
        if character == *c {
            return Ok(i);
        }
    }
    return Err(-1);
}

fn get_alphabet() -> [char;64] {
    ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q' ,'R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g' ,'h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w' ,'x','y','z','0','1','2','3','4','5','6','7','8','9','+','/']
}
