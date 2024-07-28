import LoginForm from "./components/LoginForm";
import loginLeft from "@/assets/images/login_left.png";
import logo from "@/assets/images/logo.png";
import "./index.less";
import classNames from "classnames";
const Login = () => {
	return (
		<div className="login-container">
			<div className="login-box">
				<div className="login-left">{/*<img src={loginLeft} alt="login" />*/}</div>
				<div className={classNames("login-form", { active: !!loginLeft })}>
					<div className="login-logo">
						<img className="login-icon" src={logo} alt="logo" />
						<span className="logo-text">Web3 Admin</span>
					</div>
					<LoginForm />
				</div>
			</div>
		</div>
	);
};

export default Login;
