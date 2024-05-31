#[cfg(test)]
mod tests;

pub fn base64_encode(input: String) -> String {
    let bits = get_bits(input);

    let mut res: Vec<char> = Vec::new();
    for i in 0..bits.len() / 6 {
        let slice = &bits[i*6..i*6+6];
        let base64_idx = u8::from_str_radix(slice, 2).expect("Not a binary");
        res.push(get_base_64_alphabet(base64_idx.into()));
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

fn get_base_64_alphabet(idx: usize) -> char {
    ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P','Q'
        ,'R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f','g'
        ,'h','i','j','k','l','m','n','o','p','q','r','s','t','u','v','w'
        ,'x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'][idx]
}

