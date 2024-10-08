import { Upload, Button, message } from "antd";
import { UploadOutlined } from "@ant-design/icons";
import { store } from "@/redux";
import React, { useState } from "react";
import * as url from "node:url";
const FileUpload = () => {
	const token: string = store.getState().global.token;
	const [urlLst, setUrlLst] = useState("");
	const props = {
		name: "file",
		action: "a770x/common/fileUpload",
		headers: {
			"x-user-token": token
		},
		onChange(info: {
			file: {
				response: { code: Number; tip: ""; data: { fileUrl: "" } };
				status: string;
				name: any;
			};
			fileList: any;
		}) {
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
			{/*@ts-ignore*/}
			<Upload {...props}>
				<Button icon={<UploadOutlined />}>Click to Upload</Button>
			</Upload>
			<div>{urlLst}</div>
			<div>
				<img src={urlLst} alt="" />
			</div>
		</div>
	);
};

export default FileUpload;
