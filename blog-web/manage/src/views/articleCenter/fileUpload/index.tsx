import { Upload, Button, message } from "antd";
import { UploadOutlined } from "@ant-design/icons";
import { store } from "@/redux";
import { useState } from "react";
const FileUpload = () => {
	const token: string = store.getState().global.token;
	const [urlLst, setUrlLst] = useState("");
	const props = {
		name: "file",
		action: "api/common/fileUpload",
		headers: {
			"x-user-token": token
		},
		onChange(info: { file: { status: string; name: any }; fileList: any }) {
			console.log("onChange");
			if (info.file.status !== "uploading") {
				console.log(info.file, info.fileList);
			}
			if (info.file.status === "done") {
				console.log(info.file.response);
				const { code, tip, data } = info.file.response;
				if (code != 2000) {
					message.error(tip);
					return;
				}
				message.success(data.fileUrl);
				setUrlLst(data.fileUrl);
			} else if (info.file.status === "error") {
				message.error(`${info.file.name} file upload failed.`);
			}
		}
	};
	return (
		<div style={{ margin: "20px" }}>
			<Upload {...props}>
				<Button icon={<UploadOutlined />}>Click to Upload</Button>
			</Upload>
			<div>{urlLst}</div>
		</div>
	);
};

export default FileUpload;
