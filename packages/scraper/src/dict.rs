use std::collections::HashMap;

pub fn from(attr: &[String]) -> HashMap<String, String> {
    attr.chunks(2)
        .filter_map(|chunk| {
            let key = chunk.get(0)?;
            let val = chunk.get(1)?;
            Some((key.to_string(), val.to_string()))
        })
        .collect::<HashMap<String, String>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn doesnt_crash(attr: Vec<String>) {
            from(&attr);
        }
    }
}
