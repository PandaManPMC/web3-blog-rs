import 'package:flutter/material.dart';

class SetMenuTitle extends StatelessWidget {
  const SetMenuTitle({
    super.key,
    required this.text,
    required this.icon,
    required this.active,
    required this.onTap,
  });

  final String text;

  final IconData icon;

  final bool active;

  final Function() onTap;

  @override
  Widget build(BuildContext context) {
    return Column(children: [
      const Padding(
        padding: EdgeInsets.only(left: 24),
        child: Divider(color: Colors.white24, height: 1),
      ),
      Stack(children: [
        AnimatedPositioned(
          duration: const Duration(milliseconds: 350),
          height: 56,
          left: 0,
          width: active ? 288 : 0,
          child: DecoratedBox(
            decoration: BoxDecoration(
              color: const Color(0xFF376AED),
              borderRadius: BorderRadius.circular(10),
            ),
          ),
        ),
        ListTile(
          leading: Icon(icon, color: Colors.white),
          title: Text(text),
          textColor: Colors.white,
          onTap: onTap,
        ),
      ]),
    ]);
  }
}
