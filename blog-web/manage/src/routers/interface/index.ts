export interface MetaProps {
	keepAlive?: boolean;
	requiresAuth?: boolean;
	title: string;
	key?: string;
	icon?: string;
	hidden?: boolean;
}

export interface RouteObject {
	caseSensitive?: boolean;
	sort?: number;
	children?: RouteObject[];
	element?: React.ReactNode;
	index?: boolean;
	path?: string;
	meta?: MetaProps;
	isLink?: string;
}
