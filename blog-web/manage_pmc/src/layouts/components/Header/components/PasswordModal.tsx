import React, { useState, useImperativeHandle, Ref } from "react";
import { Modal, message, Input } from "antd";
import { LockOutlined } from "@ant-design/icons";
import { changePwd } from "@/api/modules/login";
import { setToken } from "@/redux/modules/global/action";
import { setUserName } from "@/redux/modules/user/action";
import { connect } from "react-redux";
import { useNavigate } from "react-router-dom";
const mapDispatchToProps = { setToken, setUserName };
const PasswordModal = (props: any) => {
	const { setToken, setUserName } = props;
	const navigate = useNavigate();
	const [isModalVisible, setIsModalVisible] = useState(false);
	const [pwd, setPwd] = useState({
		userPwd: ""
	});

	useImperativeHandle(props.innerRef, () => ({
		showModal
	}));

	const showModal = () => {
		setIsModalVisible(true);
	};

	const handleOk = async () => {
		const { code, tip } = await changePwd({ ...pwd });
		// @ts-ignore
		if (code === 2000) {
			message.success(tip);
			handleCancel();
			setToken("");
			setUserName("");
			message.success("退出登录成功！");
			navigate("/login");
		} else {
			message.success(tip);
		}
	};

	const handleCancel = () => {
		setPwd({
			userPwd: ""
		});
		setIsModalVisible(false);
	};
	return (
		<Modal
			title="修改密码"
			visible={isModalVisible}
			onOk={handleOk}
			onCancel={handleCancel}
			destroyOnClose={true}
			keyboard={false}
			maskClosable={false}
		>
			<Input.Password
				value={pwd.userPwd}
				autoComplete="new-password"
				placeholder="请输入新密码"
				prefix={<LockOutlined />}
				onChange={e => {
					setPwd({ userPwd: e.target.value.trim() });
				}}
			/>
		</Modal>
	);
};
export default connect(null, mapDispatchToProps)(PasswordModal);
