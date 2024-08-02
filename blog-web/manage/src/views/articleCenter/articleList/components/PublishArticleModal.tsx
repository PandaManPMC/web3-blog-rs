import React, { useState, useImperativeHandle, useEffect } from "react";
// @ts-ignore
import MarkdownIt from "markdown-it";
import MdEditor from "react-markdown-editor-lite";
import "react-markdown-editor-lite/lib/index.css";
import { Modal, Input, Select, message, Typography, Row, Col } from "antd";
import { articPublish, changeArticle, getClassesLst } from "@/api/modules/article";

const PublishArticleModal = (props: any) => {
	console.log(props);
	const mdParser = new MarkdownIt(/* Markdown-it options */);
	const [editState, setEditState] = useState(false);
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [articleClassesLst, setArticleClassesLst] = useState([]);
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
		getClassesLstSelect();
	}, [props.setRowData]);
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
						<Typography.Title level={5}>文章类型</Typography.Title>
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
							文章顺序
						</Typography.Title>
						<Input
							size="large"
							placeholder="请输入文章顺序"
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
				/>
			</Modal>
		</>
	);
};
export default PublishArticleModal;
