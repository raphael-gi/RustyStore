#[cfg(test)]
mod tests;

use base64_kenji::base64_encode;

pub struct JWT {
    headers: Vec<[String;2]>,
    payload: Vec<[String;2]>,
    secret: String
}

impl JWT {
    pub fn new(secret: String) -> Self {
        JWT {
            headers: Vec::new(),
            payload: Vec::new(),
            secret
        }
    }

    fn build_json(values: Vec<[String;2]>) -> String {
        let mut res: String = values.into_iter().map(|[key, value]| {
            format!(r#""{}":"{}","#, key, value)
        }).collect::<String>();

        res.pop();

        format!("{{{}}}", res)
    }

    pub fn build(self) -> String {
        let header = base64_encode(Self::build_json(self.headers));
        let payload = base64_encode(Self::build_json(self.payload));
        let secret = self.secret;

        let hash_input = format!("{}.{}.{}", header, payload, secret);

        format!("{}.{}.{}", header, payload, hash(hash_input, secret))
    }

    pub fn add_header(mut self, key: &str, value: &str) -> Self {
        self.headers.push([key.to_string(), value.to_string()]);
        self
    }
    pub fn add_payload(mut self, key: &str, value: &str) -> Self {
        self.payload.push([key.to_string(), value.to_string()]);
        self
    }
}

fn hash(input: String, secret: String) -> String {
    let mut hash_bytes: [u8;30] = [0;30];
    let input_bytes = input.into_bytes();
    let secret_bytes = secret.into_bytes();

    let mut input_idx = 0;
    let mut secret_idx = 0;

    for i in 0..30 {
        if input_idx >= input_bytes.len() {
            input_idx = 0;
        }
        if secret_idx >= secret_bytes.len() {
            secret_idx = 0;
        }

        hash_bytes[i] = (input_bytes[input_idx] + secret_bytes[secret_idx]) % 128;

        input_idx += 1;
        secret_idx += 1;
    }

    String::from_utf8(hash_bytes.to_vec()).unwrap()
}

