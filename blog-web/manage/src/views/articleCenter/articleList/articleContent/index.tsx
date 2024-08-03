import { useLocation } from "react-router-dom";
// @ts-ignore
import MarkdownIt from "markdown-it";
import markdownItAnchor from "markdown-it-anchor";
// @ts-ignore
import markdownItTOC from "markdown-it-toc-done-right";
const ArticleContent = () => {
	const location = useLocation();
	// @ts-ignore
	const { data } = location.state || {};
	console.log(data);
	const mdParser = new MarkdownIt({
		html: false,
		xhtmlOut: true,
		typographer: true
	});
	mdParser.use(markdownItAnchor, {
		symbol: "#",
		space: false,
		placement: "before",
		permalinkBefore: true,
		permalink: true
	});
	mdParser.use(markdownItTOC, {
		listType: "ul",
		level: [1, 2, 3, 4, 5, 6]
	});
	const htmlContent = mdParser.render(data.content);
	return (
		<>
			<div dangerouslySetInnerHTML={{ __html: htmlContent }} />
		</>
	);
};
export default ArticleContent;
