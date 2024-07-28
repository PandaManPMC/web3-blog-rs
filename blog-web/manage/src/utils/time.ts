import * as moment from "moment-timezone";
export function formatTime(timestamp: number, type = "ymdHMS"): string {
	if (String(timestamp).length === 10) {
		timestamp = timestamp * 1000;
	}
	// 创建 Moment 对象，将其设置为北京时区
	const beijingTime = moment.tz(timestamp, "Asia/Shanghai");
	if (type === "ymdHMS") {
		return beijingTime.format("YYYY-MM-DD HH:mm:ss");
	} else {
		return beijingTime.format("YYYY-MM-DD");
	}
}
