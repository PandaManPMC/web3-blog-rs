import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

import 'component/introduce_component.dart';
import 'component/splash_photo_staggered.dart';
import 'splash_provider.dart';

class SplashPage extends StatefulWidget {
  const SplashPage({super.key});

  @override
  State<SplashPage> createState() => _SplashPageState();
}

class _SplashPageState extends State<SplashPage>
    with SingleTickerProviderStateMixin {
  late AnimationController animCtr;

  @override
  void initState() {
    super.initState();
    animCtr = AnimationController(
        duration: const Duration(milliseconds: 1000), vsync: this);
    playPageAnim();
  }

  @override
  void deactivate() {
    super.deactivate();
    animCtr.stop();
  }

  @override
  void dispose() {
    super.dispose();
    animCtr.dispose();
  }

  void playPageAnim() async {
    await Future.delayed(const Duration(milliseconds: 350));
    animCtr.forward();
  }

  @override
  Widget build(BuildContext context) {
    var top = MediaQuery.of(context).padding.top;

    return ChangeNotifierProvider(
      create: (_) => SplashProvider(),
      child: WillPopScope(
        onWillPop: () async => false,
        child: Scaffold(
          body: SingleChildScrollView(
            padding: EdgeInsets.fromLTRB(20, top + 60, 20, 0),
            child: SlideTransition(
              position: Tween<Offset>(
                      begin: const Offset(0, -2), end: const Offset(0, 0))
                  .animate(CurvedAnimation(
                      parent: animCtr, curve: Curves.bounceOut)),
              child: const SplashPhotoStaggered(),
            ),
          ),
          bottomSheet: SlideTransition(
            position: Tween<Offset>(
                    begin: const Offset(-1.5, 0), end: const Offset(0, 0))
                .animate(
                    CurvedAnimation(parent: animCtr, curve: Curves.bounceOut)),
            child: const IntroduceComponent(),
          ),
        ),
      ),
    );
  }
}
