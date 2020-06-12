fn parse_expr(expr: String) -> i32 {
    return 0;
}

fn main() {
    parse_expr(String::from("1a1"));
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mul() {
	let s = "3a2c4";
	assert_eq!(parse_expr(String::from(s)), 20);
    }

    #[test]
    fn add_div() {
	let s = "32a2d2";
	assert_eq!(parse_expr(String::from(s)), 17);
    }

    #[test]
    fn add_sub_mul() {
	let s = "500a10b66c32";
	assert_eq!(parse_expr(String::from(s)), 14208);
    }

    #[test]
    fn sub_expr() {
	let s = "";
	assert_eq!(parse_expr(String::from(s)), 235);
    }

    #[test]
    fn nested_sub_expr() {
	let s = "";
	assert_eq!(parse_expr(String::from(s)), 990);
    }
}
