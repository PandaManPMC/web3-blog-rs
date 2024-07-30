import { useState, useImperativeHandle, useEffect } from "react";
// @ts-ignore
import MarkdownIt from "markdown-it";
import MdEditor from "react-markdown-editor-lite";
import "react-markdown-editor-lite/lib/index.css";
import { Modal, Input, Select, message } from "antd";
import { articPublish, changeArticle } from "@/api/modules/article";

const PublishArticleModal = (props: any) => {
	const mdParser = new MarkdownIt(/* Markdown-it options */);
	const [editState, setEditState] = useState(false);
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [publish, setPublish] = useState<any>({
		idBlogClasses: null,
		titleArticle: null,
		statePublish: null,
		statePrivate: null,
		content: "",
		sequence: null
	});
	useImperativeHandle(props.innerRef, () => ({
		showModal
	}));
	useEffect(() => {
		if (JSON.stringify(props.setRowData) !== "{}") {
			setEditState(true);
			setPublish({ ...props.setRowData });
		} else {
			setEditState(false);
		}
	}, []);
	const showModal = (params: any) => {
		if (params.isModalVisible) {
			setIsModalVisible(true);
		} else {
			setIsModalVisible(false);
		}
	};
	const handleOk = async () => {
		if (editState) {
			// @ts-ignore
			const { code, tip } = await changeArticle({ ...publish });
			// @ts-ignore
			if (code === 2000) {
				setIsModalVisible(false);
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
				props.onPublish(true);
			} else {
				message.error(tip);
			}
		} else {
			// @ts-ignore
			const { code, tip } = await articPublish({ ...publish });
			// @ts-ignore
			if (code === 2000) {
				setIsModalVisible(false);
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
				props.onPublish(true);
			} else {
				message.error(tip);
			}
		}
	};
	const handleCancel = () => {
		setEditState(false);
		props.onCancel();
		setIsModalVisible(false);
		setPublish({
			idBlogClasses: null,
			titleArticle: null,
			statePublish: null,
			statePrivate: null,
			content: "",
			sequence: null
		});
	};
	// Finish!
	// @ts-ignore
	function handleEditorChange({ text }) {
		setPublish({ ...publish, content: text });
	}
	return (
		<>
			<Modal
				title={editState ? "编辑文章" : "新增文章"}
				visible={isModalVisible}
				onOk={handleOk}
				onCancel={handleCancel}
				style={{ width: "100vw", top: "0", paddingBottom: "0" }}
				bodyStyle={{ height: "calc(100vh - 55px - 53px)", overflowY: "auto" }}
				keyboard={false}
				maskClosable={false}
				width={"100%"}
			>
				<Input
					size="large"
					placeholder="请输入文章类型"
					value={publish.idBlogClasses}
					onChange={e => {
						setPublish({ ...publish, idBlogClasses: Number(e.target.value.trim()) });
					}}
				/>
				<Input
					size="large"
					placeholder="请输入文章标题"
					style={{ marginTop: "10px" }}
					value={publish.titleArticle}
					onChange={e => {
						setPublish({ ...publish, titleArticle: String(e.target.value.trim()) });
					}}
				/>
				<Select
					size="large"
					value={publish.statePrivate}
					style={{ width: "100%", marginTop: "10px" }}
					options={[
						{ value: 1, label: "私有" },
						{ value: 2, label: "公开" }
					]}
					placeholder="请选择文章可见性"
					onChange={e => {
						setPublish({ ...publish, statePrivate: e });
					}}
				/>
				<Select
					size="large"
					value={publish.statePublish}
					style={{ width: "100%", marginTop: "10px" }}
					options={[
						{ value: 1, label: "未发布" },
						{ value: 2, label: "已发布" }
					]}
					placeholder="请选择发布状态"
					onChange={e => {
						setPublish({ ...publish, statePublish: e });
					}}
				/>
				<Input
					size="large"
					placeholder="请输入文章顺序"
					style={{ marginTop: "10px" }}
					value={publish.sequence}
					onChange={e => {
						setPublish({ ...publish, sequence: Number(e.target.value.trim()) });
					}}
				/>
				<MdEditor
					value={publish.content}
					style={{ height: "500px", marginTop: "10px" }}
					renderHTML={text => mdParser.render(text)}
					onChange={handleEditorChange}
				/>
			</Modal>
		</>
	);
};
export default PublishArticleModal;
