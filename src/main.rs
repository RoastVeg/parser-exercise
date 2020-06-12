fn a_add(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(String::from(rhs))?;
    Ok(lhs + v)
}

fn b_sub(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(String::from(rhs))?;
    Ok(lhs - v)
}

fn c_mul(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(String::from(rhs))?;
    Ok(lhs * v)
}

fn d_div(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(String::from(rhs))?;
    Ok(lhs / v)
}

fn match_operator(c: char) -> bool {
    match c {
	'a' | 'b' | 'c' | 'd' => true,
	_ => false,
    }
}

fn parse_expr(sub_expr: String) -> Result<i32, String> {
    // Use find and rfind for e/f
    let operator_offset = sub_expr.rfind(match_operator);
    match operator_offset {
	Some(i) => {
	    let lhs = parse_expr(String::from(&sub_expr[0..i]))?;
	    let operator = &sub_expr[i..(i+1)];
	    let rhs = &sub_expr[(i+1)..];
	    match operator {
		"a" => a_add(lhs, rhs),
		"b" => b_sub(lhs, rhs),
		"c" => c_mul(lhs, rhs),
		"d" => d_div(lhs, rhs),
		_ => Err(String::from("Invalid operator in expression")),
	    }
	},
	None => {
	    match sub_expr.parse::<i32>() {
		Ok(v) => Ok(v),
		Err(e) => Err(format!("Integer expected in expression: {:?}", e)),
	    }
	},
    }
}

fn main() {
    println!("{}", parse_expr(String::from("1a1")).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mul() {
	let s = "3a2c4";
	assert_eq!(parse_expr(String::from(s)).unwrap(), 20);
    }

    #[test]
    fn add_div() {
	let s = "32a2d2";
	assert_eq!(parse_expr(String::from(s)).unwrap(), 17);
    }

    #[test]
    fn add_sub_mul() {
	let s = "500a10b66c32";
	assert_eq!(parse_expr(String::from(s)).unwrap(), 14208);
    }

    #[test]
    fn sub_expr() {
	let s = "3ae4c66fb32";
	assert_eq!(parse_expr(String::from(s)).unwrap(), 235);
    }

    #[test]
    fn nested_sub_expr() {
	let s = "3c4d2aee2a4c41fc4f";
	assert_eq!(parse_expr(String::from(s)).unwrap(), 990);
    }
}
