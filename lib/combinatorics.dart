import 'dart:math';

class Combinatoric {
  bool hasAnswer = true;
  late String question;
  late int answer;

  Combinatoric() {
    final rng = Random();
    final n = rng.nextInt(9) + 2;
    final r = rng.nextInt(n - 1) + 1;
    question = '{}^{$n}C_{$r}';
    answer = _factorial(n) ~/ (_factorial(r) * _factorial(n - r));
  }

  _factorial(int n) {
    int total = 1;
    for (var i = n; i >= 1; i--) {
      total *= i;
    }
    return total;
  }
}
