import { useEffect, useRef, useState } from "react";
import { Table, Button, Input, Row, Col, message, Popconfirm } from "antd";
import { getLabelLst, delLabel } from "@/api/modules/label";
import "./index.less";
import { formatTime } from "@/utils/time";
import LabelModal from "./components/LabelModal";
import { PlusOutlined } from "@ant-design/icons";
const ArticleList = () => {
	const [dataSource, setDataSource] = useState<Array<any>>([]);
	const labelRef = useRef(null);
	// const [pagination, setPagination] = useState({
	// 	current: 1,
	// 	pageSize: 20
	// });
	// const handleTableChange = (pagination: SetStateAction<{ current: number; pageSize: number }>) => {
	// 	setPagination(pagination);
	// 	setQuery({
	// 		// @ts-ignore
	// 		pageIndex: pagination.current,
	// 		// @ts-ignore
	// 		pageSize: pagination.pageSize
	// 	});
	// 	getList();
	// };
	useEffect(() => {
		getList();
	}, []);
	const columns: any[] = [
		// {
		// 	title: "ID",
		// 	dataIndex: "id",
		// 	key: "id",
		// 	align: "center"
		// },
		{
			title: "标签名称",
			dataIndex: "labelName",
			key: "labelName",
			align: "center"
		},
		{
			title: "标签状态",
			dataIndex: "state",
			key: "state",
			align: "center",
			render: (state: number) => {
				if (state === 1) {
					return <span>正常</span>;
				} else if (state === 2) {
					return <span>已删除</span>;
				}
			}
		},
		{
			title: "创建时间",
			dataIndex: "createdAt",
			key: "createdAt",
			align: "center",
			render: (time: number) => {
				return formatTime(time);
			}
		},
		{
			title: "更新时间",
			dataIndex: "updatedAt",
			key: "updatedAt",
			align: "center",
			render: (time: number) => {
				return formatTime(time);
			}
		},
		{
			title: "顺序",
			dataIndex: "sequence",
			key: "sequence",
			align: "center"
		},
		{
			title: "操作",
			key: "edit",
			align: "center",
			render: (record: any) => {
				return (
					<Popconfirm
						placement="top"
						title={"确认删除该条标签吗？"}
						okText="确定"
						cancelText="取消"
						onConfirm={() => handleOpen("del", record.id)}
					>
						<Button type={"link"} danger>
							删除
						</Button>
					</Popconfirm>
				);
			}
		}
	];
	const getList = async () => {
		const { data } = await getLabelLst({
			pageIndex: 1,
			pageSize: 20000
		});
		if (data) {
			// @ts-ignore
			setDataSource(data);
		}
	};
	const LabelDone = (state: boolean) => {
		state && getList();
	};
	const handleOpen = async (type: string, id?: null) => {
		if (type === "del") {
			const { code, tip } = await delLabel({
				id: id
			});
			// @ts-ignore
			if (code === 2000) {
				getList();
				message.success(tip);
			} else {
				message.error(tip);
			}
			return false;
		}
		// @ts-ignore
		labelRef.current!.showModal({ isModalVisible: true });
	};
	return (
		<div className="card content-box">
			<div className="date">
				<Input.Group size="large">
					<Row justify="end">
						<Col>
							<Button style={{ marginLeft: "15px" }} type="primary" icon={<PlusOutlined />} onClick={() => handleOpen("add")}>
								新增标签
							</Button>
						</Col>
					</Row>
				</Input.Group>
			</div>
			<Table
				bordered={true}
				dataSource={dataSource}
				columns={columns}
				rowKey={record => String(record.id)}
				// pagination={pagination}
				// onChange={handleTableChange}
			></Table>
			<LabelModal innerRef={labelRef} onLable={LabelDone}></LabelModal>
		</div>
	);
};

export default ArticleList;
