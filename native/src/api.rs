use rand::Rng;

pub fn square(n: u32) -> u32 {
	n * n
}

pub struct Question {
	pub prompt: String,
	pub answer: String,
}

pub fn combinatoric() -> Question {
	let n = rand::thread_rng().gen_range(2..10);
	let r = rand::thread_rng().gen_range(1..n);
	let prompt = format!("{{}}^{{{n}}}C_{{{r}}}", n = n, r = r);
	let answer = factorial(n) / (factorial(r) * factorial(n - r));
	return Question {
		prompt,
		answer: answer.to_string(),
	};
}

fn factorial(n: u64) -> u64 {
	let mut result = 1;
	for i in 2..=n {
		result *= i;
	}
	result
}
