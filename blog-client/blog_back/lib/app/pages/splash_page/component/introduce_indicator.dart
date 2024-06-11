import 'package:blog_back/app/pages/splash_page/splash_provider.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class IntroduceIndicator extends StatelessWidget {
  const IntroduceIndicator({super.key});

  @override
  Widget build(BuildContext context) {
    int indicatorIndex = context.watch<SplashProvider>().indicatorIndex;
    return Row(children: [
      IndicatorDot(active: indicatorIndex == 0),
      const SizedBox(width: 8),
      IndicatorDot(active: indicatorIndex == 1),
      const SizedBox(width: 8),
      IndicatorDot(active: indicatorIndex == 2),
    ]);
  }
}

class IndicatorDot extends StatelessWidget {
  const IndicatorDot({super.key, required this.active});

  final bool active;

  @override
  Widget build(BuildContext context) {
    return AnimatedContainer(
      duration: const Duration(milliseconds: 350),
      height: 8,
      width: active ? 23 : 8,
      decoration: BoxDecoration(
        color: active ? const Color(0xFF376AED) : const Color(0xFFDEE7FF),
        borderRadius: BorderRadius.circular(8),
      ),
    );
  }
}
