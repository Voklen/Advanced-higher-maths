import 'dart:math';

import 'package:flutter/material.dart';

import 'package:advanced_higher_maths/ffi.dart';

import 'package:flutter_math_fork/flutter_math.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  final List<Future<Question> Function()> allQuestions = [
    api.combinatoric,
    api.algebraicExpansion
  ];
  final random = Random();
  Future<Question> currentQuestion = api.combinatoric();

  void _reloadQuestion() async {
    final nextQuestion = allQuestions[random.nextInt(allQuestions.length)];
    setState(() {
      currentQuestion = nextQuestion();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: FutureBuilder(
          future: currentQuestion,
          builder: (context, snapshot) {
            final question = snapshot.data;
            if (question == null) {
              return Container();
            }
            return Column(
              mainAxisAlignment: MainAxisAlignment.center,
              children: [
                Math.tex(
                  question.prompt,
                  textScaleFactor: 4,
                ),
                Math.tex(
                  question.answer,
                ),
              ],
            );
          },
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _reloadQuestion,
        tooltip: 'Reload question',
        child: const Icon(Icons.restart_alt),
      ),
    );
  }
}
