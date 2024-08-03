import { useLocation } from "react-router-dom";
// @ts-ignore
import MarkdownIt from "markdown-it";
import markdownItAnchor from "markdown-it-anchor";
import { figure } from "@mdit/plugin-figure";
// @ts-ignore
import markdownItTOC from "markdown-it-toc-done-right";
const ArticleContent = () => {
	const location = useLocation();
	// @ts-ignore
	const { data } = location.state || {};
	console.log(data);
	const mdParser = new MarkdownIt({
		html: false,
		typographer: true
	});
	mdParser.use(markdownItAnchor, { permalink: true });
	mdParser.use(markdownItTOC, {
		containerClass: "toc",
		containerId: "toc",
		listType: "ul",
		listClass: "cataloglistClass",
		linkClass: "cataloglinkClass"
	});
	mdParser.use(figure);
	const htmlContent = mdParser.render(data.content);
	return (
		<>
			<div dangerouslySetInnerHTML={{ __html: htmlContent }} />
		</>
	);
};
export default ArticleContent;
