use rand::Rng;

pub struct Question {
	pub prompt: String,
	pub answer: String,
}

pub fn combinatoric() -> Question {
	let n = rand::thread_rng().gen_range(2..10);
	let r = rand::thread_rng().gen_range(1..n);
	let prompt = combinatoric_prompt(n, r);
	let answer = combinatoric_result(n, r);
	return Question {
		prompt,
		answer: answer.to_string(),
	};
}
fn combinatoric_prompt(n: u64, r: u64) -> String {
	if rand::random() {
		format!("{{}}^{{{n}}}C_{{{r}}}")
	} else {
		format!("{n} \\choose {r}")
	}
}
fn combinatoric_result(n: u64, r: u64) -> u64 {
	factorial(n) / (factorial(r) * factorial(n - r))
}

pub fn algebraic_expansion() -> Question {
	let power = rand::thread_rng().gen_range(3..8);
	let prompt = format!("(x+y)^{{{power}}}");
	let answer = algebraic_expansion_answer(power);
	return Question { prompt, answer };
}
fn algebraic_expansion_answer(power: u64) -> String {
	let to_particular_term = |r| algebraic_expansion_for_single_term(r, power);

	(0..=power)
		.into_iter()
		.map(to_particular_term)
		.reduce(collect_terms)
		.unwrap()
}
fn algebraic_expansion_for_single_term(r: u64, power: u64) -> String {
	let combinatoric = if r != 0 && r != power {
		combinatoric_result(power, r).to_string()
	} else {
		"".to_string()
	};
	let x = x_part(r, power);
	let y = y_part(r);
	format!("{combinatoric}{x}{y}")
}
fn x_part(r: u64, power: u64) -> String {
	if r == power {
		return "".to_string();
	}
	let x_power = power - r;
	if x_power == 1 {
		return "x".to_string();
	}
	format!("{{x}}^{{{x_power}}}")
}
fn y_part(r: u64) -> String {
	match r {
		0 => "".to_string(),
		1 => "y".to_string(),
		_ => format!("{{y}}^{{{r}}}"),
	}
}
fn collect_terms(total: String, term: String) -> String {
	format!("{total} + {term}")
}

struct Item {
	pub var_name: String,
	pub multiply: i64,
	pub expo: i64,
}

impl Item {
	fn standard(variable_name: &str) -> Self {
		let multiply = rand::thread_rng().gen_range(-5..=5);
		let power = rand::thread_rng().gen_range(-2..=3);
		Item {
			var_name: variable_name.to_string(),
			multiply,
			expo: power,
		}
	}

	fn as_LaTeX(&self) -> String {
		let var_name = &self.var_name;
		let multiply = match self.multiply {
			1 => "".to_string(),
			-1 => "-".to_string(),
			other => other.to_string(),
		};
		match self.expo {
			1 => format!("{multiply}{var_name}"),
			expo => format!("{multiply}{var_name}^{{{expo}}}"),
		}
	}
}

struct PoweredNum {
	pub number: i64,
	pub exponent: i64,
}

pub fn particular_term() -> Question {
	let term1 = Item::standard("x");
	let term2 = Item::standard("y");
	let power = rand::thread_rng().gen_range(3..=8);
	let expansion_as_latex = format!("({} + {})^{power}", term1.as_LaTeX(), term2.as_LaTeX());

	let var_answer = calculate_particular_term_var(term1, term2, power);
	let prompt = format!(
		"\\textbf{{Find the coefficient of }} {var_answer} \\textbf{{ in the expansion of}} {expansion_as_latex}"
	);
	Question {
		prompt,
		answer: "".into(),
	}
}
fn calculate_particular_term_var(term1: Item, term2: Item, expo: i64) -> String {
	let num1 = PoweredNum {
		number: term1.multiply,
		exponent: expo,
	};
	let num2 = PoweredNum {
		number: term2.multiply,
		exponent: expo,
	};
	// x^(expo1_multiply(expo1_sub - r))
	let expo1_sub = expo;
	let expo1_multiply = term1.expo;
	// x^(expo1_sub - r*expo1_multiply)
	let expo1_sub = expo1_sub * expo1_multiply;

	// x^(expo2_multiply * r)
	let expo2_multiply = term2.expo;

	if term1.var_name == term2.var_name {
		let var_name = term1.var_name;
		let expo_multiply = expo1_multiply * expo2_multiply;
		let expo_string = add_and_format(expo1_sub, expo_multiply, &var_name);
		format!("{var_name}^{{{expo_string}}}")
	} else {
		let var1_name = term1.var_name;
		let expo_string = add_and_format(expo1_sub, expo1_multiply, "r");
		let var1_string = format!("{var1_name}^{{{expo_string}}}");

		let var2_name = term2.var_name;
		let var2_string = format!("{var2_name}^{{{expo2_multiply}r}}");
		format!("{var1_string}{var2_string}")
	}
}

fn add_and_format(num: i64, multiplier: i64, variable_name: &str) -> String {
	let multiplier_string = if multiplier == 1 {
		"".into()
	} else {
		multiplier.to_string()
	};
	if num < 0 {
		return format!("{multiplier_string}{variable_name}{num}");
	} else if num == 0 {
		return format!("{multiplier_string}{variable_name}");
	}
	match multiplier {
		..=-1 => format!("{num}{multiplier_string}{variable_name}"),
		0 => format!("{num}"),
		1.. => format!("{multiplier_string}{variable_name} + {num}"),
	}
}

fn factorial(n: u64) -> u64 {
	let mut result = 1;
	for i in 2..=n {
		result *= i;
	}
	result
}
