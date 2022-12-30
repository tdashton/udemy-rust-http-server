use std::collections::HashMap;

#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>), // Heap allocated array is a Vector in Rust
}

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>,
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        return self.data.get(key);
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();

        for sub_str in s.split('&') {
            // gives us an iterator
            let mut key = sub_str;
            let mut val = "";

            // return an Option w/ index (Ok / None)
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_val) => {
                        // let mut vec = Vec::new();
                        // vec.push(val);
                        // vec.push(&prev_val);

                        // the three above lines (create Vec and push values) is represented by this macro
                        // let mut vec = vec![prev_val, val];
                        // and we move this directly into the parameter for Multiple below

                        // this asterick is also dereferencing - adding it to the beginning means that Rust
                        // will take the new reference and replace the old one with it.
                        // "follow the pointer and write this new value over whatever it was pointing to before"
                        *existing = Value::Multiple(vec![prev_val, val]);
                    }
                    Value::Multiple(vec) => vec.push(val),
                })
                .or_insert(Value::Single(val));
        }

        return QueryString { data };
    }
}
