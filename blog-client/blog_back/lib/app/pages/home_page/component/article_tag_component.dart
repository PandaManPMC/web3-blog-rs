import 'package:flutter/material.dart';

class ArticleTagComponent extends StatefulWidget {
  const ArticleTagComponent({super.key});

  @override
  State<ArticleTagComponent> createState() => _ArticleTagComponentState();
}

class _ArticleTagComponentState extends State<ArticleTagComponent> {
  void onTapAdd() {}

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(minHeight: 80),
      child: Wrap(spacing: 5, runSpacing: 10, children: [
        ButtonArticleAdd(onTap: onTapAdd),
        const ArticleTag("Art"),
        const ArticleTag("Design"),
        const ArticleTag("Culture"),
        const ArticleTag("Production Production"),
      ]),
    );
  }
}

class ButtonArticleAdd extends StatelessWidget {
  const ButtonArticleAdd({super.key, required this.onTap});

  final Function() onTap;

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        height: 32,
        width: 70,
        color: Colors.transparent,
        alignment: Alignment.centerLeft,
        child: const Text(
          "Add Tags",
          style: TextStyle(
              color: Color(0xFF376AED),
              fontSize: 14,
              fontWeight: FontWeight.w600),
        ),
      ),
    );
  }
}

class ArticleTag extends StatelessWidget {
  const ArticleTag(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) {
    return Container(
      height: 32,
      padding: const EdgeInsets.symmetric(horizontal: 3),
      decoration: BoxDecoration(
        color: const Color(0xFF376AED).withOpacity(0.1),
        borderRadius: BorderRadius.circular(32),
        border: Border.all(color: const Color(0xFF376AED), width: 1),
      ),
      child: Row(mainAxisSize: MainAxisSize.min, children: [
        const SizedBox(width: 10),
        ConstrainedBox(
          constraints: const BoxConstraints(maxWidth: 100),
          child: Text(
            text,
            maxLines: 1,
            overflow: TextOverflow.ellipsis,
            style: const TextStyle(color: Color(0xFF376AED), fontSize: 12),
          ),
        ),
        const SizedBox(width: 10),
        Image.asset("images/cancel.png", width: 24, height: 24),
      ]),
    );
  }
}
