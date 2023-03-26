use rand::Rng;

pub struct Question {
	pub prompt: String,
	pub answer: String,
}

pub fn combinatoric() -> Question {
	let n = rand::thread_rng().gen_range(2..10);
	let r = rand::thread_rng().gen_range(1..n);
	let prompt = combinatoric_prompt(n, r);
	let answer = factorial(n) / (factorial(r) * factorial(n - r));
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

fn factorial(n: u64) -> u64 {
	let mut result = 1;
	for i in 2..=n {
		result *= i;
	}
	result
}
