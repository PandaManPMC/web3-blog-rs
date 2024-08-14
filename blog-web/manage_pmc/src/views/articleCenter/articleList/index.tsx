import React, { useEffect, useRef, useState } from "react";
import { Table, Input, Row, Col, Button, Select, message } from "antd";
import { SearchOutlined, PlusOutlined, CloseCircleOutlined } from "@ant-design/icons";
import { getArticleLst, getArticleLabelLst, getClassesLst } from "@/api/modules/article";
import "./index.less";
import { formatTime } from "@/utils/time";
import ArticleLabelModal from "./components/ArticleLabelModal";
import { getLabelLst } from "@/api/modules/label";
import { Link } from "react-router-dom";
const ArticleList = () => {
	// 按钮权限
	const [dataSource, setDataSource] = useState<Array<any>>([]);
	const [rowData, setRowData] = useState({});
	const [rowId, setRowId] = useState(null);
	const [rowLabel, setRowLabel] = useState([]);
	const [articleLabelList, setArticleLabelList] = useState([]);
	const [articleClassesLst, setArticleClassesLst] = useState([]);
	const [query, setQuery] = useState<any>({
		idBlogClasses: null,
		stateArticle: null,
		statePublish: null,
		statePrivate: null,
		pageIndex: 1,
		pageSize: 20000
	});
	const publishRef = useRef(null);
	const articleLabelRef = useRef(null);
	useEffect(() => {
		getList();
		getLabels();
		getClassesLstSelect();
	}, []);
	const columns: any[] = [
		{
			title: "编号",
			dataIndex: "id",
			key: "id",
			align: "center"
		},
		{
			title: "文章类型",
			dataIndex: "idBlogClasses",
			key: "idBlogClasses",
			align: "center",
			render: (idBlogClasses: number) => {
				for (const cls of articleClassesLst) {
					// @ts-ignore
					if (idBlogClasses == cls.value) {
						// @ts-ignore
						return <span>{cls.label}</span>;
					}
				}
			}
		},
		{
			title: "文章状态",
			dataIndex: "stateArticle",
			key: "stateArticle",
			align: "center",
			render: (stateArticle: number) => {
				if (stateArticle === 1) {
					return <span>正常</span>;
				} else if (stateArticle === 2) {
					return <span>已删除</span>;
				}
			}
		},
		{
			title: "发布状态",
			dataIndex: "statePublish",
			key: "statePublish",
			align: "center",
			render: (statePublish: number) => {
				if (statePublish === 1) {
					return <span>未发布</span>;
				} else if (statePublish === 2) {
					return <span>已发布</span>;
				}
			}
		},
		{
			title: "文章可见性",
			dataIndex: "statePrivate",
			key: "statePrivate",
			align: "center",
			render: (statePrivate: number) => {
				if (statePrivate === 1) {
					return <span>私有</span>;
				} else if (statePrivate === 2) {
					return <span>公开</span>;
				}
			}
		},
		{
			title: "文章标题",
			dataIndex: "titleArticle",
			key: "titleArticle",
			align: "center"
		},
		{
			title: "点赞数量",
			dataIndex: "likeCount",
			key: "likeCount",
			align: "center"
		},
		{
			title: "观看数量",
			dataIndex: "watchCount",
			key: "watchCount",
			align: "center"
		},
		{
			title: "评论数量",
			dataIndex: "viewCount",
			key: "viewCount",
			align: "center"
		},
		{
			title: "发布时间",
			dataIndex: "timePublish",
			align: "center",
			render: (time: number) => {
				return formatTime(time);
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
				// @ts-ignore
				return (
					<>
						<Link to={`/articleManage/articleContent`} state={{ data: record }}>
							详情
						</Link>
						<Button type={"link"} onClick={() => handleLabel(record)}>
							标签
						</Button>
						<Link to={`/articleManage/articleEdit`} state={{ data: record }}>
							编辑
						</Link>
					</>
				);
			}
		}
	];
	const getList = async () => {
		const { data } = await getArticleLst({ ...query });
		if (data) {
			// @ts-ignore
			setDataSource(data);
		}
	};

	const handleLabel = async (row: any) => {
		const { code, data } = await getArticleLabelLst({ id: row.id });
		// @ts-ignore
		if (code === 2000) {
			// @ts-ignore
			articleLabelRef.current!.showModal({ isModalVisible: true });
			// @ts-ignore
			setRowLabel(data);
			setRowId(row.id);
		}
	};
	const getLabels = async () => {
		const { code, data, tip } = await getLabelLst({
			pageIndex: 1,
			pageSize: 20000
		});
		// @ts-ignore
		if (code === 2000) {
			// @ts-ignore
			let labelList: { label: any; value: any }[] = [];
			// @ts-ignore
			data.map((item: any) => {
				labelList.push({ label: item.labelName, value: item.id });
			});
			// @ts-ignore
			setArticleLabelList(labelList);
		} else {
			message.error(tip);
		}
	};
	const getClassesLstSelect = async () => {
		const { code, data, tip } = await getClassesLst({
			pageIndex: 1,
			pageSize: 20000
		});
		// @ts-ignore
		if (code === 2000) {
			// @ts-ignore
			let lst: { label: any; value: any }[] = [];
			// @ts-ignore
			data.map((item: any) => {
				lst.push({ label: item.classesName, value: item.id });
			});
			// @ts-ignore
			setArticleClassesLst(lst);
		} else {
			message.error(tip);
		}
	};
	return (
		<div className="card content-box">
			<div className="date">
				<Input.Group size="large">
					<Row justify="space-between" style={{ marginBottom: "16px" }}>
						<Col span={5}>
							<Select
								size="large"
								value={query.idBlogClasses}
								style={{ width: "100%" }}
								options={[...articleClassesLst]}
								placeholder="请选择文章类型"
								onChange={e => {
									setQuery({ ...query, idBlogClasses: e });
								}}
							/>
						</Col>
						<Col span={5}>
							<Select
								size="large"
								value={query.stateArticle}
								style={{ width: "100%" }}
								options={[
									{ value: 1, label: "正常" },
									{ value: 2, label: "已删除" }
								]}
								placeholder="请选择文章状态"
								onChange={e => {
									setQuery({ ...query, stateArticle: e });
								}}
							/>
						</Col>
						<Col span={5}>
							<Select
								size="large"
								value={query.statePublish}
								style={{ width: "100%" }}
								options={[
									{ value: 1, label: "未发布" },
									{ value: 2, label: "已发布" }
								]}
								placeholder="请选择发布状态"
								onChange={e => {
									setQuery({ ...query, statePublish: e });
								}}
							/>
						</Col>
						<Col span={5}>
							<Select
								size="large"
								value={query.statePrivate}
								style={{ width: "100%" }}
								options={[
									{ value: 1, label: "私有" },
									{ value: 2, label: "公开" }
								]}
								placeholder="请选择文章可见性"
								onChange={e => {
									setQuery({ ...query, statePrivate: e });
								}}
							/>
						</Col>
					</Row>
					<Row justify="space-between">
						<Col>
							<Button type="primary" icon={<SearchOutlined />} onClick={getList}>
								搜索
							</Button>
							<Button
								style={{ marginLeft: "15px" }}
								icon={<CloseCircleOutlined />}
								onClick={() =>
									setQuery({
										idBlogClasses: null,
										stateArticle: null,
										statePublish: null,
										statePrivate: null,
										pageIndex: 1,
										pageSize: 200
									})
								}
							>
								重置
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
			<ArticleLabelModal
				innerRef={articleLabelRef}
				onPublish={getList}
				setRowData={rowLabel}
				setId={rowId}
				onCancel={() => setRowLabel([])}
				labelList={articleLabelList}
			></ArticleLabelModal>
		</div>
	);
};

export default ArticleList;
