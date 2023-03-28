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

fn factorial(n: u64) -> u64 {
	let mut result = 1;
	for i in 2..=n {
		result *= i;
	}
	result
}
