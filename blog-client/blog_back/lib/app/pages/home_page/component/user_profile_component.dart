import 'package:flutter/material.dart';

class UserProfileComponent extends StatelessWidget {
  const UserProfileComponent({super.key});

  @override
  Widget build(BuildContext context) {
    return ListTile(
      title: const Text("Jasmine Levin"),
      subtitle: const Text("BC Designer"),
      textColor: const Color(0xFFFFFFFF),
      titleTextStyle: const TextStyle(
        fontSize: 18,
        fontWeight: FontWeight.w600,
      ),
      subtitleTextStyle: const TextStyle(fontSize: 14),
      leading: Image.asset("images/user.png", width: 50, height: 50),
    );
  }
}
