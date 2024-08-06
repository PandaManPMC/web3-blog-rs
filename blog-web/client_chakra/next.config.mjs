/** @type {import('next').NextConfig} */
import fs from 'fs';
import path from 'path';
import dotenv from 'dotenv';

// 根据环境变量加载对应的 .env 文件
const envFilePath = path.resolve(process.cwd(), `.env.${process.env.NODE_ENV}`);
const envConfig = dotenv.parse(fs.readFileSync(envFilePath));

for (const k in envConfig) {
    process.env[k] = envConfig[k];
}

console.log("配置文件：" + envFilePath);

/** @type {import('next').NextConfig} */
const nextConfig = {
    // 其他 Next.js 配置项
    output: 'export',
    distDir: 'out', // 这是导出静态文件的目录
    exportTrailingSlash: true // 在每个路径后面添加斜杠，以便于兼容静态站点
};

export default nextConfig;
