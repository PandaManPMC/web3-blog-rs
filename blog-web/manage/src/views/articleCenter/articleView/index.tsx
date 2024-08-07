import React, { useRef } from "react";
import { useLocation } from "react-router-dom";
import ReactMarkdown from "react-markdown";
// @ts-ignore
import MarkNav from "markdown-navbar";
import gfm from "remark-gfm";
// @ts-ignore
import { Prism as SyntaxHighlighter } from "react-syntax-highlighter";
// @ts-ignore
import { coldarkCold } from "react-syntax-highlighter/dist/esm/styles/prism";
import "markdown-navbar/dist/navbar.css";
import "./index.less";

const ArticleView = () => {
	const location = useLocation();
	const contentRef = useRef(null);
	// @ts-ignore
	const { content, title } = location.state || {};

	const handleScrollIntoView = (e: React.MouseEvent<HTMLDivElement>) => {
		e.preventDefault();
		// 获取data-id
		setTimeout(() => {
			const dataId = window.location.hash.substring(1);
			if (dataId) {
				const contentElement = contentRef.current;
				if (contentElement) {
					// @ts-ignore
					const targetElement = contentElement.querySelector(`[data-id="${dataId}"]`);
					if (targetElement) {
						// 滚动到目标元素
						targetElement.scrollIntoView({ behavior: "smooth", block: "start" });
					}
				}
			}
		}, 10);
	};
	return (
		<div className="content-box">
			<div className="wrap">
				<div className="content" ref={contentRef}>
					<h1 className="title">{title}</h1>
					<ReactMarkdown
						remarkPlugins={[gfm]}
						components={{
							code: ({ children = [], className, ...props }) => {
								const match = /language-(\w+)/.exec(className || "");
								return (
									<SyntaxHighlighter
										language={match?.[1]}
										showLineNumbers={false}
										style={coldarkCold as any}
										PreTag="div"
										className="syntax-hight-wrapper"
										{...props}
									>
										{children as string[]}
									</SyntaxHighlighter>
								);
							}
						}}
					>
						{content}
					</ReactMarkdown>
				</div>
				<div className="nav" onClick={handleScrollIntoView}>
					<MarkNav className="article-menu" source={content} />
				</div>
			</div>
		</div>
	);
};

export default ArticleView;
