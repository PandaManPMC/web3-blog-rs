import React, { useState } from "react";
import { Button, Form, Input, message } from "antd";
import { useNavigate } from "react-router-dom";
import { Login } from "@/api/interface";
import { loginApi } from "@/api/modules/login";
import { HOME_URL } from "@/config/config";
import { connect } from "react-redux";
import { setToken } from "@/redux/modules/global/action";
import { setTabsList } from "@/redux/modules/tabs/action";
import { setUserName } from "@/redux/modules/user/action";
const mapDispatchToProps = { setToken, setTabsList, setUserName };
import { UserOutlined, LockOutlined, CloseCircleOutlined } from "@ant-design/icons";
// @ts-ignore
import ReCAPTCHA from "react-google-recaptcha";
import { getVerifyReCaptchaToken } from "@/api/modules/common";
const LoginForm = (props: any) => {
	const { setToken, setTabsList, setUserName } = props;
	const navigate = useNavigate();
	const [form] = Form.useForm();
	const [loading, setLoading] = useState<boolean>(false);
	// 登录
	const onFinish = async (loginForm: Login.ReqLoginForm) => {
		try {
			setLoading(true);
			const { data } = await loginApi(loginForm);
			if (data) {
				// @ts-ignore
				setUserName(data["penName"]);
				// @ts-ignore
				setToken(data["userToken"]);
				setTabsList([]);
				message.success("登录成功！");
				navigate(HOME_URL);
			}
		} finally {
			setLoading(false);
		}
	};

	const onFinishFailed = (errorInfo: any) => {
		console.log("Failed:", errorInfo);
	};
	const getReCAPTCHA = async (value: string) => {
		// @ts-ignore
		const { data } = await getVerifyReCaptchaToken({
			captchaToken: value
		});
		form.setFieldsValue({ captchaToken: data });
	};

	return (
		<Form
			form={form}
			name="basic"
			labelCol={{ span: 5 }}
			initialValues={{
				userName: "laoniqiu",
				userPwd: "12345678",
				captchaToken: ""
			}}
			onFinish={onFinish}
			onFinishFailed={onFinishFailed}
			size="large"
			autoComplete="off"
		>
			<Form.Item name="userName" rules={[{ required: true, message: "请输入用户名" }]}>
				<Input placeholder="请输入用户名" prefix={<UserOutlined />} />
			</Form.Item>
			<Form.Item name="userPwd" rules={[{ required: true, message: "请输入密码" }]}>
				<Input.Password autoComplete="new-password" placeholder="请输入密码" prefix={<LockOutlined />} />
			</Form.Item>
			<Form.Item name="captchaToken" style={{ display: "none" }}>
				<Input placeholder="请输入验证码" prefix={<UserOutlined />} />
			</Form.Item>
			<Form.Item>
				<ReCAPTCHA sitekey="6Lcn0fYpAAAAALGpkke4_lQtZEg8XqvjYlhSsb74" onChange={getReCAPTCHA} />
			</Form.Item>

			<Form.Item className="login-btn">
				<Button
					onClick={() => {
						form.resetFields();
					}}
					icon={<CloseCircleOutlined />}
				>
					重置
				</Button>
				<Button type="primary" htmlType="submit" loading={loading} icon={<UserOutlined />}>
					登录
				</Button>
			</Form.Item>
		</Form>
	);
};

export default connect(null, mapDispatchToProps)(LoginForm);
