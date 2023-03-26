import 'package:flutter/material.dart';

import 'package:advanced_higher_maths/bridge_generated.dart';
import 'package:advanced_higher_maths/ffi.dart';
import 'combinatorics.dart';

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
  Combinatoric combinatoric = Combinatoric();
  String text = '';

  void _reloadQuestion() async {
    combinatoric = Combinatoric();
    final answer = combinatoric.answer;
    final result = await api.square(n: answer);

    setState(() {
      text = result.toString();
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            Math.tex(
              combinatoric.question,
              textScaleFactor: 4,
            ),
            Text(combinatoric.answer.toString()),
            Text('The square is $text'),
          ],
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
