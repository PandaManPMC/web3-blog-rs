import 'package:flutter/material.dart';

ThemeData appTheme = ThemeData.light().copyWith(
  useMaterial3: true,
  scaffoldBackgroundColor: const Color(0xFFE6EAF1),
  inputDecorationTheme: const InputDecorationTheme(
    floatingLabelBehavior: FloatingLabelBehavior.always,
    labelStyle: TextStyle(color: Color(0xFF2D4379), fontSize: 16),
    floatingLabelStyle: TextStyle(color: Color(0xFF2D4379), fontSize: 16),
    border: UnderlineInputBorder(
      borderSide: BorderSide(color: Color(0xFFD9DFEB)),
    ),
    enabledBorder: UnderlineInputBorder(
      borderSide: BorderSide(color: Color(0xFFD9DFEB)),
    ),
    focusedBorder: UnderlineInputBorder(
      borderSide: BorderSide(color: Color(0xFF376AED)),
    ),
  ),
);
