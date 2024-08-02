import { useState, useImperativeHandle, useEffect } from "react";
import { Modal, message, Select, Checkbox, Row, Col } from "antd";
import { changeArticleLabel } from "@/api/modules/article";
import { getLabelLst } from "@/api/modules/label";
const ArticleLabelModal = (props: any) => {
	const CheckboxGroup = Checkbox.Group;
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [articleLabelId, setArticleLabelId] = useState([]); //当前文章的label列表
	const [articleLabelList, setArticleLabelList] = useState([]);
	useImperativeHandle(props.innerRef, () => ({
		showModal
	}));
	useEffect(() => {
		console.log(props.setRowData);
		if (props.setRowData) {
			// @ts-ignore
			setArticleLabelId([...props.setRowData]);
		}
		let lst = props.labelList;
		for (let j = 0; j < props.setRowData.length; j++) {
			for (let i = 0; i < lst.length; i++) {
				if (lst[i].value == props.setRowData[j]) {
					lst[i].checked = true;
					break;
				}
			}
		}

		setArticleLabelList(lst);
		console.log(lst);
	}, [props.setRowData]);
	const showModal = (params: any) => {
		if (params.isModalVisible) {
			setIsModalVisible(true);
		} else {
			setIsModalVisible(false);
		}
	};

	const handleOk = async () => {
		console.log(articleLabelId);
		// const { code, tip } = await changeArticleLabel({ id: props.setId, id_label: articleLabelId });
		// @ts-ignore
		// if (code === 2000) {
		setIsModalVisible(false);
		// message.success(tip);
		// setArticleLabelList([]);
		// props.onPublish(true);
		// } else {
		// message.error(tip);
		// }
	};

	const handleCancel = () => {
		props.onCancel();
		setIsModalVisible(false);
	};
	const handleChange = async (value: any) => {
		console.log(value);
		console.log(articleLabelList);
		let orCheckArr = [];
		for (let i = 0; i < articleLabelList.length; i++) {
			// @ts-ignore
			if (articleLabelList[i].checked) {
				// @ts-ignore
				orCheckArr.push(articleLabelList[i].value);
			}
			// @ts-ignore
			articleLabelList[i].checked = false;
		}
		console.log(orCheckArr);
		// @ts-ignore
		for (let j = 0; j < value.length; j++) {
			for (let i = 0; i < articleLabelList.length; i++) {
				// @ts-ignore
				if (value[j] == articleLabelList[i].value) {
					// @ts-ignore
					articleLabelList[i].checked = true;
				}
			}
		}

		let val = 0;
		console.log(orCheckArr.length);
		console.log(value.length);
		if (orCheckArr.length > value.length) {
			// remove
			for (let i = 0; i < orCheckArr.length; i++) {
				let isC = true;
				for (let j = 0; j < value.length; j++) {
					if (orCheckArr[i] == value[j]) {
						isC = false;
						break;
					}
				}
				if (isC) {
					val = orCheckArr[i];
					break;
				}
			}
		}
		if (orCheckArr.length < value.length) {
			// add
			for (let i = 0; i < value.length; i++) {
				let isC = true;
				for (let j = 0; j < orCheckArr.length; j++) {
					if (orCheckArr[j] == value[i]) {
						isC = false;
						break;
					}
				}
				if (isC) {
					val = value[i];
					break;
				}
			}
		}
		console.log("val=" + val);
		const { code, tip } = await changeArticleLabel({ id: props.setId, id_label: val });
		console.log(code + "===" + tip);
		setArticleLabelId(value);
		setArticleLabelList(articleLabelList);
		console.log(articleLabelId);
	};
	// @ts-ignore
	return (
		<>
			<Modal
				title={"文章标签"}
				visible={isModalVisible}
				onOk={handleOk}
				onCancel={handleCancel}
				keyboard={false}
				maskClosable={false}
			>
				<CheckboxGroup options={articleLabelList} value={articleLabelId} onChange={handleChange} />
			</Modal>
		</>
	);
};
export default ArticleLabelModal;
