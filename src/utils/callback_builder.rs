use std::fmt::Display;

pub struct CallbackBuilder {
    base: String,
    route: Option<String>,
    params: Vec<String>
}

impl CallbackBuilder {
    pub fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            route: None,
            params: vec![],
        }
    }

    pub fn route(self, route: &str) -> Self {
        if route.chars().find(|char| char.eq(&'&')).is_none() {
            Self {
                route: Option::from(route.to_string()),
                ..self
            }
        } else {
            panic!("You can not use '&' in callback route")
        }
    }

    pub fn bind<T: Display>(mut self, param: T) -> Self {
        let tmp = param.to_string();
        if tmp.chars().find(|char| char.eq(&'&')).is_none() {
            self.params.push(tmp);
            Self {
                params: self.params,
                ..self
            }
        } else {
            panic!("You can not use '&' in callback params");
        }
    }

    pub fn build(self) -> String {
        let mut res = self.base.clone();
        if !self.route.is_none() {
            res += self.route.unwrap().as_str();
        }

        if !self.params.is_empty() {
            res += "?";
            for i in 0..self.params.len() {
                let mut tmp = String::from(&self.params[i]);
                if i != self.params.len() - 1 {
                    tmp += "&"
                }
                res += &tmp;
            }
        }

        res
    }
}