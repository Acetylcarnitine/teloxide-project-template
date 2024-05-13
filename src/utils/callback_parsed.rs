use std::str::FromStr;

#[derive(Debug)]
pub struct CallbackParsed {
    pub base: String,
    pub route: Option<String>,
    pub params: Vec<String>
}

impl CallbackParsed {
    // parse input callback into struct like CallbackBuilder
    // (this need for better understanding of code)
    pub fn parse(call_data: &String) -> Self {
        if call_data.is_empty() {
            panic!("An empty callback")
        }

        // parse args and path (includes route and base)
        let data_split: Vec<&str> = call_data.split("?").collect();
        let call_path: Vec<&str> = data_split[0].split("/").collect();
        let base = call_path[0].to_string();

        // parse route (after base)
        let mut route = None;
        if let Some(tmp) = call_path.get(1) {
            route = Option::from("/".to_string() + tmp);
        }

        // parse params (optional)
        let mut params: Vec<String> = Vec::new();
        if let Some(args) = data_split.get(1) {
            args.split("&").for_each(
                |param| params.push(param.to_string())
            );
        }

        Self {
            base,
            route,
            params
        }
    }

    pub fn get_arg<T: FromStr>(&self, index: usize) -> Result<T, T::Err> {
        T::from_str(self.params[index].as_str())
    }

    pub fn get_str_arg(&self, index: usize) -> &str {
        &self.params[index]
    }
}