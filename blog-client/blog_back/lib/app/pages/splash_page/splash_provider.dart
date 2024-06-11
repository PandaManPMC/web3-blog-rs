import 'package:flutter/material.dart';

class SplashProvider with ChangeNotifier {
  final List<Introduce> introduceTexts = [
    Introduce("Read the article you want instantly",
        "You can read thousands of articles on Blog Club, save them in the application and share them with your loved ones."),
    Introduce("Read the article you want instantly",
        "You can read thousands of articles on Blog Club, save them in the application and share them with your loved ones."),
    Introduce("Read the article you want instantly",
        "You can read thousands of articles on Blog Club, save them in the application and share them with your loved ones."),
  ];

  int indicatorIndex = 0;
  final PageController controller = PageController();

  @override
  void dispose() {
    super.dispose();
    controller.dispose();
  }

  void onPageChanged(int page) {
    indicatorIndex = page;
    notifyListeners();
  }
}

class Introduce {
  final String title;

  final String content;

  Introduce(this.title, this.content);
}
