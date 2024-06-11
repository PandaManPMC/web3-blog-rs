import 'package:blog_back/app/pages/splash_page/splash_provider.dart';
import 'package:flutter/material.dart';
import 'package:provider/provider.dart';

class IntroduceTextView extends StatelessWidget {
  const IntroduceTextView({super.key});

  @override
  Widget build(BuildContext context) {
    SplashProvider provider = context.read<SplashProvider>();
    return PageView(
      controller: provider.controller,
      onPageChanged: provider.onPageChanged,
      children: List.generate(
        provider.introduceTexts.length,
        (i) => TextView(introduce: provider.introduceTexts[i]),
      ),
    );
  }
}

class TextView extends StatelessWidget {
  const TextView({super.key, required this.introduce});

  final Introduce introduce;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 20),
      child: Column(crossAxisAlignment: CrossAxisAlignment.start, children: [
        Text(
          introduce.title,
          style: const TextStyle(
            color: Color(0xFF0D253C),
            fontWeight: FontWeight.w600,
            fontSize: 24,
          ),
        ),
        const SizedBox(height: 10),
        Expanded(
          child: Text(
            introduce.content,
            style: const TextStyle(
              color: Color(0xFF0D253C),
              fontSize: 14,
            ),
          ),
        ),
      ]),
    );
  }
}
