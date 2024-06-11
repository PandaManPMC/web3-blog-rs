import 'package:flutter/material.dart';

class SplashPhoto extends StatelessWidget {
  const SplashPhoto({super.key, required this.photo});

  final String photo;

  @override
  Widget build(BuildContext context) {
    var boxShadow = BoxShadow(
      color: const Color(0xFF0D253C).withOpacity(0.2),
      offset: const Offset(5, 20),
      blurRadius: 10,
    );

    return Container(
      height: 60,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(28),
        boxShadow: [boxShadow],
        image: DecorationImage(image: AssetImage(photo), fit: BoxFit.fill),
      ),
    );
  }
}
