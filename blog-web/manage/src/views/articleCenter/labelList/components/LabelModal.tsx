import { useState, useImperativeHandle } from "react";
import { Modal, Input, message } from "antd";
import { createLabel } from "@/api/modules/label";

const LabelModal = (props: any) => {
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [label, setLabel] = useState<any>({
		labelName: null,
		sequence: null
	});
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

	const handleOk = async () => {
		// @ts-ignore
		const { code, tip } = await createLabel({ ...label });
		// @ts-ignore
		if (code === 2000) {
			setIsModalVisible(false);
			message.success(tip);
			setLabel({
				labelName: null,
				sequence: null
			});
			props.onLable(true);
		} else {
			message.success(tip);
		}
	};
	const handleCancel = () => {
		setIsModalVisible(false);
	};
	return (
		<>
			<Modal
				title="新增标签"
				visible={isModalVisible}
				onOk={handleOk}
				onCancel={handleCancel}
				keyboard={false}
				maskClosable={false}
			>
				<Input
					size="large"
					placeholder="请输入标签名称"
					value={label.labelName}
					onChange={e => {
						setLabel({ ...label, labelName: e.target.value.trim() });
					}}
				/>
				<Input
					size="large"
					placeholder="请输入标签顺序"
					style={{ marginTop: "10px" }}
					value={label.sequence}
					onChange={e => {
						setLabel({ ...label, sequence: Number(e.target.value.trim()) });
					}}
				/>
			</Modal>
		</>
	);
};
export default LabelModal;
