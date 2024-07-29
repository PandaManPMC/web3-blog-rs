import { useState, useImperativeHandle } from "react";
import { Modal, Input, message } from "antd";
import { createClasses } from "@/api/modules/article";

const ClassesModal = (props: any) => {
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [classes, setClasses] = useState<any>({
		classesName: null,
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
		const { code, tip } = await createClasses({ ...classes });
		// @ts-ignore
		if (code === 2000) {
			setIsModalVisible(false);
			message.success(tip);
			setClasses({
				classesName: null,
				sequence: null
			});
			props.onClasses(true);
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
				title="新增类型"
				visible={isModalVisible}
				onOk={handleOk}
				onCancel={handleCancel}
				keyboard={false}
				maskClosable={false}
			>
				<Input
					size="large"
					placeholder="请输入类型名称"
					value={classes.classesName}
					onChange={e => {
						setClasses({ ...classes, classesName: e.target.value.trim() });
					}}
				/>
				<Input
					size="large"
					placeholder="请输入类型顺序"
					style={{ marginTop: "10px" }}
					value={classes.sequence}
					onChange={e => {
						setClasses({ ...classes, sequence: Number(e.target.value.trim()) });
					}}
				/>
			</Modal>
		</>
	);
};
export default ClassesModal;
