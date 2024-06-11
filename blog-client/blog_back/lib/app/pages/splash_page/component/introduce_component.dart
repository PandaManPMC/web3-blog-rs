import 'package:blog_back/app/pages/splash_page/component/introduce_button_next.dart';
import 'package:blog_back/app/pages/splash_page/component/introduce_text_view.dart';
import 'package:flutter/material.dart';

import 'introduce_indicator.dart';

class IntroduceComponent extends StatelessWidget {
  const IntroduceComponent({super.key});

  @override
  Widget build(BuildContext context) {
    const borderRadius = BorderRadius.only(
      topLeft: Radius.circular(28),
      topRight: Radius.circular(28),
    );

    var boxShadow = BoxShadow(
      color: const Color(0xFF0D253C).withOpacity(0.02),
      offset: const Offset(0, -10),
      blurRadius: 20,
    );

    return Container(
      height: 324,
      width: double.infinity,
      padding: const EdgeInsets.fromLTRB(0, 30, 0, 20),
      decoration: BoxDecoration(
        color: const Color(0xFFFFFFFF),
        borderRadius: borderRadius,
        boxShadow: [boxShadow],
      ),
      child: const Column(children: [
        Expanded(child: IntroduceTextView()),
        SizedBox(height: 20),
        Row(children: [
          SizedBox(width: 20),
          IntroduceIndicator(),
          Spacer(),
          IntroduceButtonNext(),
          SizedBox(width: 20),
        ])
      ]),
    );
  }
}
