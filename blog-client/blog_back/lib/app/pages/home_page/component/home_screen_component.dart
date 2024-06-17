import 'package:blog_back/app/pages/home_page/component/article_tag_component.dart';
import 'package:flutter/material.dart';

class HomeScreenComponent extends StatelessWidget {
  const HomeScreenComponent({super.key});

  @override
  Widget build(BuildContext context) {
    return ColoredBox(
      color: const Color(0xFFFFFFFF),
      child: Column(children: [
        AppBar(
          automaticallyImplyLeading: false,
          title: const Text("New Article"),
          titleTextStyle: const TextStyle(
            color: Color(0xFF0D253C),
            fontSize: 24,
            fontWeight: FontWeight.w600,
          ),
          actions: [
            IconButton(onPressed: onTapUpload, icon: const Icon(Icons.upload)),
          ],
        ),
        Expanded(
          child: SingleChildScrollView(
            padding: const EdgeInsets.fromLTRB(20, 30, 20, 0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                const TextField(
                  style: TextStyle(
                    color: Color(0xFF0D253C),
                    fontSize: 22,
                    fontWeight: FontWeight.w600,
                  ),
                  decoration: InputDecoration(
                    hintText: "Please enter article title",
                  ),
                ),
                const SizedBox(height: 15),
                const Text(
                  "Add article tags",
                  style: TextStyle(
                    color: Color(0xFF0D253C),
                    fontSize: 14,
                  ),
                ),
                const SizedBox(height: 15),
                Divider(
                  indent: 0,
                  endIndent: 120,
                  color: const Color(0xFF7B8BB2).withOpacity(0.2),
                  height: 1,
                ),
                const SizedBox(height: 20),
                const ArticleTagComponent(),
                const SizedBox(height: 20),
                const Text(
                  "Article Content",
                  style: TextStyle(
                    color: Color(0xFF0D253C),
                    fontSize: 14,
                  ),
                ),
                const SizedBox(height: 15),
                Divider(
                  color: const Color(0xFF7B8BB2).withOpacity(0.2),
                  height: 1,
                ),
              ],
            ),
          ),
        ),
      ]),
    );
  }

  void onTapUpload() {}
}
