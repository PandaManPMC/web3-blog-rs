import { useEffect, useState } from "react";
import { Table } from "antd";
import { getArticleLabelLst } from "@/api/modules/article";
import "./index.less";
import { formatTime } from "@/utils/time";
const ArticleList = () => {
	const [dataSource, setDataSource] = useState<Array<any>>([]);
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
			title: "类型名称",
			dataIndex: "classesName",
			key: "classesName",
			align: "center"
		},
		{
			title: "类型状态",
			dataIndex: "state",
			key: "state",
			align: "center",
			render: (state: number) => {
				if (state === 1) {
					return <span>正常</span>;
				} else if (state === 2) {
					return <span>不可见</span>;
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
		}
	];
	const getList = async () => {
		const { data } = await getArticleLabelLst({
			pageIndex: 1,
			pageSize: 20000
		});
		if (data) {
			// @ts-ignore
			setDataSource(data);
		}
	};
	return (
		<div className="card content-box">
			<Table
				bordered={true}
				dataSource={dataSource}
				columns={columns}
				rowKey={record => String(record.id)}
				// pagination={pagination}
				// onChange={handleTableChange}
			></Table>
		</div>
	);
};

export default ArticleList;
