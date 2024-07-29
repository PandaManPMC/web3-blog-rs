import { useState, useImperativeHandle, useEffect } from "react";
import { Modal, message, Select } from "antd";
import { changeArticleLabel } from "@/api/modules/article";
import { getLabelLst } from "@/api/modules/label";
const ArticleLabelModal = (props: any) => {
	console.log(props);
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [articleLabelId, setArticleLabelId] = useState([]); //当前文章的label列表
	const [articleLabelList, setArticleLabelList] = useState([]);
	useImperativeHandle(props.innerRef, () => ({
		showModal
	}));
	useEffect(() => {
		if (props.setRowData) {
			let labels = [];
			props.setRowData.map((item: any) => {
				labels.push(item.id);
			});
			// @ts-ignore
			setArticleLabelId([...props.setRowData]);
		}
		setArticleLabelList(props.labelList);
	}, [props.setRowData]);
	const showModal = (params: any) => {
		if (params.isModalVisible) {
			setIsModalVisible(true);
		} else {
			setIsModalVisible(false);
		}
	};

	const handleOk = async () => {
		const { code, tip } = await changeArticleLabel({ id: props.setId, id_label: articleLabelId });
		// @ts-ignore
		if (code === 2000) {
			setIsModalVisible(false);
			message.success(tip);
			setArticleLabelList([]);
			props.onPublish(true);
		} else {
			message.error(tip);
		}
	};

	const handleCancel = () => {
		props.onCancel();
		setIsModalVisible(false);
	};
	const handelChange = (value: any) => {
		setArticleLabelId(value);
	};
	return (
		<>
			<Modal title={"文章标签"} visible={isModalVisible} onOk={handleOk} onCancel={handleCancel}>
				{/*<Select*/}
				{/*	mode="multiple"*/}
				{/*	value={articleLabelId}*/}
				{/*	onChange={handelChange}*/}
				{/*	style={{ width: "100%" }}*/}
				{/*	placeholder="请选择文章标签"*/}
				{/*	options={[...articleLabelList]}*/}
				{/*/>*/}
				<Select
					size="large"
					value={articleLabelId}
					style={{ width: "100%" }}
					options={[...articleLabelList]}
					placeholder="请选择文章标签"
					onChange={handelChange}
				/>
			</Modal>
		</>
	);
};
export default ArticleLabelModal;
