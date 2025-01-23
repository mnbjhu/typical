pub type GeneircArgs = Vec<String>;

pub trait GeneircArgsExt {
    fn get_string(&self) -> String;
}

impl GeneircArgsExt for GeneircArgs {
    fn get_string(&self) -> String {
        if self.is_empty() {
            return "".to_string();
        }
        let mut result = "[".to_string();
        for (index, arg) in self.iter().enumerate() {
            if index != 0 {
                result.push_str(", ");
            }
            result.push_str(arg);
        }
        result.push(']');
        result
    }
}
