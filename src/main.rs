fn a_add(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(rhs)?;
    Ok(lhs + v)
}

fn b_sub(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(rhs)?;
    Ok(lhs - v)
}

fn c_mul(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(rhs)?;
    Ok(lhs * v)
}

fn d_div(lhs: i32, rhs: &str) -> Result<i32, String> {
    let v = parse_expr(rhs)?;
    Ok(lhs / v)
}

fn ef_subexpr(lhs: &str, rhs: &str) -> Result<i32, String> {
    let e_offset = lhs.find('e');
    match e_offset {
	Some(i) => {
	    let pre_expr = &lhs[0..i];
	    let inner_expr = parse_expr(&lhs[(i+1)..])?;
	    parse_expr(&format!("{}{}{}", pre_expr, inner_expr.to_string(), rhs))
	},
	None => Err(String::from("Unopened e/f expression")),
    }
}

fn match_operator(c: char) -> bool {
    match c {
	'a' | 'b' | 'c' | 'd' | 'e' | 'f' => true,
	_ => false,
    }
}

fn parse_expr(sub_expr: &str) -> Result<i32, String> {
    let operator_offset = sub_expr.rfind(match_operator);
    match operator_offset {
	Some(i) => {
	    let lhs = &sub_expr[0..i];
	    let operator = &sub_expr[i..(i+1)];
	    let rhs = &sub_expr[(i+1)..];
	    match operator {
		"a" => a_add(parse_expr(lhs)?, rhs),
		"b" => b_sub(parse_expr(lhs)?, rhs),
		"c" => c_mul(parse_expr(lhs)?, rhs),
		"d" => d_div(parse_expr(lhs)?, rhs),
		"e" => Err(String::from("Unclosed e/f expression")),
		"f" => ef_subexpr(lhs, rhs),
		_ => Err(String::from("Unimplemented operator in expression")),
	    }
	},
	None => {
	    match sub_expr.parse::<i32>() {
		Ok(v) => Ok(v),
		Err(_) => Err(String::from("Integer expected in expression that contains no operators")),
	    }
	},
    }
}

fn main() {
    println!("{}", parse_expr("1a1").unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mul() {
	let s = "3a2c4";
	assert_eq!(parse_expr(s).unwrap(), 20);
    }

    #[test]
    fn add_div() {
	let s = "32a2d2";
	assert_eq!(parse_expr(s).unwrap(), 17);
    }

    #[test]
    fn add_sub_mul() {
	let s = "500a10b66c32";
	assert_eq!(parse_expr(s).unwrap(), 14208);
    }

    #[test]
    fn sub_expr() {
	let s = "3ae4c66fb32";
	assert_eq!(parse_expr(s).unwrap(), 235);
    }

    #[test]
    fn nested_sub_expr() {
	let s = "3c4d2aee2a4c41fc4f";
	assert_eq!(parse_expr(s).unwrap(), 990);
    }
}
