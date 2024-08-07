import React, { useState, useImperativeHandle, useEffect } from "react";
// @ts-ignore
import MarkdownIt from "markdown-it";
import MdEditor from "react-markdown-editor-lite";
import "react-markdown-editor-lite/lib/index.css";
import { Input, Select, message, Typography, Row, Col, Button } from "antd";
import { articPublish, changeArticle, getArticleSequence, getClassesLst } from "@/api/modules/article";
import markdownItAnchor from "markdown-it-anchor";
import { figure } from "@mdit/plugin-figure";
// @ts-ignore
import markdownItTOC from "markdown-it-toc-done-right";
import { useLocation } from "react-router-dom";
import { fileUpload } from "@/api/modules/common";

const ArticleEdit = () => {
	const location = useLocation();
	// @ts-ignore
	const { data } = location.state || {};
	console.log(data);
	const mdParser = new MarkdownIt({
		html: true,
		linkify: true,
		typographer: true
	});
	mdParser.use(markdownItAnchor);
	mdParser.use(markdownItTOC, {
		containerClass: "toc",
		containerId: "toc",
		listType: "ul",
		listClass: "cataloglistClass",
		linkClass: "cataloglinkClass"
	});
	mdParser.use(figure);
	const [editState, setEditState] = useState(false);
	const [articleClassesLst, setArticleClassesLst] = useState([]);
	const [articleSequence, setArticleSequence] = useState({
		sequenceMin: 0,
		sequenceMax: 0
	});
	const [publish, setPublish] = useState<any>({
		idBlogClasses: null,
		titleArticle: null,
		statePublish: null,
		statePrivate: null,
		content: "",
		sequence: null
	});
	useEffect(() => {
		if (data) {
			setEditState(true);
			setPublish({ ...data });
		} else {
			setEditState(false);
		}
		getClassesLstSelect();
		getArticleSequenceData();
	}, []);

	const handleOk = async () => {
		if (editState) {
			// @ts-ignore
			const { code, tip } = await changeArticle({ ...publish });
			// @ts-ignore
			if (code === 2000) {
				message.success(tip);
				setPublish({
					idBlogClasses: null,
					titleArticle: null,
					statePublish: null,
					statePrivate: null,
					content: "",
					sequence: null
				});
				setEditState(false);
			} else {
				message.error(tip);
			}
		} else {
			// @ts-ignore
			const { code, tip } = await articPublish({ ...publish });
			// @ts-ignore
			if (code === 2000) {
				message.success(tip);
				setPublish({
					idBlogClasses: null,
					titleArticle: null,
					statePublish: null,
					statePrivate: null,
					content: "",
					sequence: null
				});
				setEditState(false);
			} else {
				message.error(tip);
			}
		}
	};
	const getClassesLstSelect = async () => {
		const { code, data, tip } = await getClassesLst({
			pageIndex: 1,
			pageSize: 20000,
			state: 1
		});
		// @ts-ignore
		if (code === 2000) {
			// @ts-ignore
			let lst: { label: any; value: any }[] = [];
			// @ts-ignore
			data.map((item: any) => {
				lst.push({ label: item.classesName, value: item.id });
			});
			// @ts-ignore
			setArticleClassesLst(lst);
		} else {
			message.error(tip);
		}
	};
	const getArticleSequenceData = async () => {
		const { code, data, tip } = await getArticleSequence({});
		// @ts-ignore
		if (code === 2000) {
			// @ts-ignore
			setArticleSequence(data);
		} else {
			message.error(tip);
		}
	};
	// Finish!
	// @ts-ignore
	function handleEditorChange({ text }) {
		setPublish({ ...publish, content: text });
	}
	const handleImageUpload = async (file: File, callback: (arg0: any) => void) => {
		const imageUrl: any = await fileUpload(file);
		if (imageUrl) {
			// @ts-ignore
			callback(imageUrl.data.fileUrl);
		} else {
			message.error(imageUrl.tip);
		}
	};

	return (
		<>
			<Typography.Title level={5} style={{ marginTop: "10px" }}>
				文章标题
			</Typography.Title>
			<Input
				size="large"
				placeholder="请输入文章标题"
				value={publish.titleArticle}
				onChange={e => {
					setPublish({ ...publish, titleArticle: String(e.target.value.trim()) });
				}}
			/>
			<Row justify="space-between" style={{ marginBottom: "16px" }}>
				<Col span={5}>
					<Typography.Title level={5} style={{ marginTop: "10px" }}>
						文章类型
					</Typography.Title>
					<Select
						size="large"
						value={publish.idBlogClasses}
						style={{ width: "100%" }}
						options={[...articleClassesLst]}
						placeholder="请选择文章类型"
						onChange={e => {
							setPublish({ ...publish, idBlogClasses: e });
						}}
					/>
				</Col>
				<Col span={5}>
					<Typography.Title level={5} style={{ marginTop: "10px" }}>
						文章可见性
					</Typography.Title>
					<Select
						size="large"
						value={publish.statePrivate}
						style={{ width: "100%" }}
						options={[
							{ value: 1, label: "私有" },
							{ value: 2, label: "公开" }
						]}
						placeholder="请选择文章可见性"
						onChange={e => {
							setPublish({ ...publish, statePrivate: e });
						}}
					/>
				</Col>
				<Col span={5}>
					<Typography.Title level={5} style={{ marginTop: "10px" }}>
						发布状态
					</Typography.Title>
					<Select
						size="large"
						value={publish.statePublish}
						style={{ width: "100%" }}
						options={[
							{ value: 1, label: "未发布" },
							{ value: 2, label: "已发布" }
						]}
						placeholder="请选择发布状态"
						onChange={e => {
							setPublish({ ...publish, statePublish: e });
						}}
					/>
				</Col>
				<Col span={5}>
					<Typography.Title level={5} style={{ marginTop: "10px" }}>
						文章顺序：{articleSequence.sequenceMin} - {articleSequence.sequenceMax}
					</Typography.Title>
					<Input
						size="large"
						placeholder="请输入文章顺序"
						type="number"
						value={publish.sequence}
						onChange={e => {
							setPublish({ ...publish, sequence: Number(e.target.value.trim()) });
						}}
					/>
				</Col>
			</Row>
			<Typography.Title level={5} style={{ marginTop: "10px" }}>
				文章内容
			</Typography.Title>
			<MdEditor
				value={publish.content}
				style={{ height: "500px" }}
				renderHTML={text => mdParser.render(text)}
				onChange={handleEditorChange}
				onImageUpload={handleImageUpload}
			/>
			<Button key="submit" type="primary" onClick={handleOk}>
				发布|保存
			</Button>
		</>
	);
};
export default ArticleEdit;
