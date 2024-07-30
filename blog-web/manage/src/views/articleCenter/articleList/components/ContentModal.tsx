import { useState, useImperativeHandle } from "react";
import { Modal, Typography } from "antd";
// @ts-ignore
import MarkdownIt from "markdown-it";
const ContentModal = (props: any) => {
	const mdParser = new MarkdownIt();
	const htmlContent = mdParser.render(props.setContent);

	const [isModalVisible, setIsModalVisible] = useState(false);
	useImperativeHandle(props.innerRef, () => ({
		showModal
	}));
	const showModal = (params: any) => {
		if (params.isModalVisible) {
			setIsModalVisible(true);
		} else {
			setIsModalVisible(false);
		}
	};
	const handleCancel = () => {
		props.onCancel();
		setIsModalVisible(false);
	};
	return (
		<>
			<Modal
				title="查看文章内容"
				style={{ top: "0", paddingBottom: "0" }}
				bodyStyle={{ height: "calc(100vh - 55px - 53px)", overflowY: "auto" }}
				visible={isModalVisible}
				onOk={handleCancel}
				onCancel={handleCancel}
				keyboard={false}
				maskClosable={false}
				width={"100%"}
			>
				<div dangerouslySetInnerHTML={{ __html: htmlContent }} />
			</Modal>
		</>
	);
};
export default ContentModal;
