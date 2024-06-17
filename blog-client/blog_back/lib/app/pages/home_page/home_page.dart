import 'dart:math';
import 'dart:ui';

import 'package:flutter/material.dart';

import 'component/home_screen_component.dart';
import 'component/home_set_component.dart';

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage>
    with SingleTickerProviderStateMixin {
  late AnimationController animCtr;
  late Animation<double> slideAnim;
  late Animation<double> scaleAnim;

  @override
  void initState() {
    super.initState();
    animCtr = AnimationController(
        duration: const Duration(milliseconds: 350), vsync: this);
    slideAnim = Tween(begin: 0.0, end: 288.0)
        .animate(CurvedAnimation(parent: animCtr, curve: Curves.fastOutSlowIn));
    scaleAnim = Tween(begin: 1.0, end: 0.8)
        .animate(CurvedAnimation(parent: animCtr, curve: Curves.fastOutSlowIn));
  }

  @override
  void dispose() {
    super.dispose();
    animCtr.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return PopScope(
      canPop: false,
      child: Scaffold(
        floatingActionButton: ElevatedButton(
          child: const Icon(Icons.settings),
          onPressed: () {
            if (animCtr.isCompleted) {
              animCtr.reverse();
            } else {
              animCtr.forward();
            }
          },
        ),
        body: Stack(children: [
          Image.asset(
            "images/scaffold.png",
            width: MediaQuery.of(context).size.width,
            height: MediaQuery.of(context).size.height,
            fit: BoxFit.cover,
          ),
          BackdropFilter(
            filter: ImageFilter.blur(sigmaX: 10, sigmaY: 10),
            child: Container(),
          ),
          AnimatedBuilder(
            animation: animCtr,
            builder: (_, child) {
              return Transform.translate(
                offset: Offset(slideAnim.value - 288, 0),
                child: const HomeSetComponent(),
              );
            },
            child: const HomeSetComponent(),
          ),
          AnimatedBuilder(
            animation: animCtr,
            builder: (_, child) {
              return Transform(
                alignment: Alignment.center,
                transform: Matrix4.identity()
                  ..translate(slideAnim.value, 0)
                  ..scale(scaleAnim.value, scaleAnim.value)
                  ..setEntry(3, 2, 0.001)
                  ..rotateY(animCtr.value - 30 * animCtr.value * pi / 180),
                child: child,
              );
            },
            child: const HomeScreenComponent(),
          ),
        ]),
      ),
    );
  }
}
