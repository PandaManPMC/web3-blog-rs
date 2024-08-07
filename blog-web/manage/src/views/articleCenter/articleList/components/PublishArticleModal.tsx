import React, { useState, useImperativeHandle, useEffect } from "react";
import MDEditor, { commands } from "@uiw/react-md-editor";
import { Modal, Input, Select, message, Typography, Row } from "antd";
import { articPublish, changeArticle } from "@/api/modules/article";
import { fileUpload } from "@/api/modules/common";
import rehypeToc from "rehype-toc";
const PublishArticleModal = (props: any) => {
	const [editState, setEditState] = useState(false);
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [classesList, setClassesList] = useState([]);
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
		setClassesList(props.setClasses);
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
	const customImageCommand = {
		name: "image",
		keyCommand: "image",
		buttonProps: { "aria-label": "Insert image" },
		icon: (
			<svg width="13" height="13" viewBox="0 0 20 20">
				<path
					fill="currentColor"
					d="M15 9c1.1 0 2-.9 2-2s-.9-2-2-2-2 .9-2 2 .9 2 2 2zm4-7H1c-.55 0-1 .45-1 1v14c0 .55.45 1 1 1h18c.55 0 1-.45 1-1V3c0-.55-.45-1-1-1zm-1 13l-6-5-2 2-4-5-4 8V4h16v11z"
				></path>
			</svg>
		),
		execute: async (state: any, api: { replaceSelection: (arg0: string) => void }) => {
			const input = document.createElement("input");
			input.type = "file";
			input.accept = "image/*";
			input.onchange = async () => {
				// @ts-ignore
				const file = input.files[0];
				if (file) {
					const { data, code } = await fileUpload(file);
					// @ts-ignore
					if (code === 2000) {
						// @ts-ignore
						const imageMarkdown = `![${file.name}](${data.fileUrl})`;
						api.replaceSelection(imageMarkdown);
					}
				}
			};
			input.click();
		}
	};
	const customCommands = commands.getCommands().map(cmd => (cmd.name === "image" ? customImageCommand : cmd));
	const handleScrollIntoView = (event: React.MouseEvent<HTMLDivElement>) => {
		if (event.target instanceof HTMLAnchorElement && event.target.hash) {
			event.preventDefault();
			const targetId = decodeURIComponent(event.target.hash.substring(1)); // Decode the URI component

			// Find the target element within the .w-md-editor-md-preview container
			const previewContainer = document.querySelector(".wmde-markdown");
			if (previewContainer) {
				// Escape special characters for querySelector
				const escapedTargetId = targetId.replace(/(:|\.|\[|\]|\(|\)|\+|\^|\$|\*|\||\\)/g, "\\$1");
				const targetElement = previewContainer.querySelector(`#${escapedTargetId}`);

				if (targetElement) {
					targetElement.scrollIntoView({ behavior: "smooth" });
				}
			}
		}
	};
	// @ts-ignore
	return (
		<>
			<Modal
				title={editState ? "编辑文章" : "新增文章"}
				visible={isModalVisible}
				onOk={handleOk}
				onCancel={handleCancel}
				width="100%"
				style={{ top: 0 }}
				bodyStyle={{ height: "95%", overflowY: "hidden" }}
				keyboard={false}
				maskClosable={false}
			>
				<Row justify="space-between">
					<div style={{ width: "50%" }}>
						<Typography.Title level={5}>文章标题</Typography.Title>
						<Input
							size="large"
							placeholder="请输入文章标题"
							value={publish.titleArticle}
							style={{ width: "90%" }}
							onChange={e => {
								setPublish({ ...publish, titleArticle: String(e.target.value.trim()) });
							}}
						/>
					</div>
					<div style={{ width: "50%" }}>
						<Typography.Title level={5}>文章类型</Typography.Title>
						<Select
							size="large"
							style={{ width: "90%" }}
							value={publish.idBlogClasses}
							options={classesList}
							placeholder="请选择文章类型"
							onChange={e => {
								setPublish({ ...publish, idBlogClasses: e });
							}}
						/>
					</div>
					<div style={{ width: "50%" }}>
						<Typography.Title level={5} style={{ marginTop: "10px" }}>
							文章可见性
						</Typography.Title>
						<Select
							size="large"
							value={publish.statePrivate}
							style={{ width: "90%" }}
							options={[
								{ value: 1, label: "私有" },
								{ value: 2, label: "公开" }
							]}
							placeholder="请选择文章可见性"
							onChange={e => {
								setPublish({ ...publish, statePrivate: e });
							}}
						/>
					</div>
					<div style={{ width: "50%" }}>
						<Typography.Title level={5} style={{ marginTop: "10px" }}>
							发布状态
						</Typography.Title>
						<Select
							size="large"
							value={publish.statePublish}
							style={{ width: "90%" }}
							options={[
								{ value: 1, label: "未发布" },
								{ value: 2, label: "已发布" }
							]}
							placeholder="请选择发布状态"
							onChange={e => {
								setPublish({ ...publish, statePublish: e });
							}}
						/>
					</div>
					<div style={{ width: "50%" }}>
						<Typography.Title level={5} style={{ marginTop: "10px" }}>
							文章顺序
						</Typography.Title>
						<Input
							style={{ width: "90%" }}
							size="large"
							placeholder="请输入文章顺序"
							value={publish.sequence}
							onChange={e => {
								setPublish({ ...publish, sequence: Number(e.target.value.trim()) });
							}}
						/>
					</div>
				</Row>

				<Typography.Title level={5} style={{ marginTop: "10px" }}>
					文章内容
				</Typography.Title>
				<div className="container" onClick={handleScrollIntoView}>
					<MDEditor
						height={450}
						style={{ width: "100%" }}
						value={publish.content}
						onChange={val => setPublish({ ...publish, content: val })}
						commands={customCommands}
						previewOptions={{
							rehypePlugins: [[rehypeToc, { headings: ["h2", "h3"], cssClasses: { toc: "wmde-markdown" } }]]
						}}
					/>
				</div>
			</Modal>
		</>
	);
};
export default PublishArticleModal;
