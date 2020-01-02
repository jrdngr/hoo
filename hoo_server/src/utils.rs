pub fn next_path_component<'a>(path: &'a str) -> Option<(&'a str, &'a str)> {
    let path = path.trim_start_matches('/');
    let split_path: Vec<&str> = path.splitn(2, '/').collect();
    if split_path.is_empty() || split_path[0].is_empty() {
        None
    } else if split_path.len() > 1 {
        Some((split_path[0], split_path[1]))
    } else {
        Some((split_path[0], ""))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_path() {
        let path = "/one/two/three";
        
        let (one, rest) = next_path_component(path).unwrap();
        assert_eq!(one, "one");

        let (two, rest) = next_path_component(rest).unwrap();
        assert_eq!(two, "two");
        
        let (three, rest) = next_path_component(rest).unwrap();
        assert_eq!(three, "three");

        assert_eq!(rest, "");

        let done = next_path_component(rest);
        assert_eq!(done, None);
    }

    #[test]
    fn empty_path() {
        let path = "";
        let result = next_path_component(path);
        assert_eq!(result, None);
    }

    #[test]
    fn just_slash() {
        let path = "/";
        let result = next_path_component(path);
        assert_eq!(result, None);
    }
}
