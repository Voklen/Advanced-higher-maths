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
	let to_particular_term = |r: u64| -> String {
		let combinatoric = if r != 0 && r != power {
			combinatoric_result(power, r).to_string()
		} else {
			"".to_string()
		};
		let x = if r != power {
			let x_power = power - r;
			format!("{{x}}^{{{x_power}}}")
		} else {
			"".to_string()
		};
		let y = if r != 0 {
			format!("{{y}}^{{{r}}}")
		} else {
			"".to_string()
		};
		format!("{combinatoric}{x}{y}")
	};

	fn collect_terms(total: String, term: String) -> String {
		format!("{total} + {term}")
	}

	(0..=power)
		.into_iter()
		.map(to_particular_term)
		.reduce(collect_terms)
		.unwrap()
}

fn factorial(n: u64) -> u64 {
	let mut result = 1;
	for i in 2..=n {
		result *= i;
	}
	result
}
