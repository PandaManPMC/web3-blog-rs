import React from 'react';
import { Box, Button } from '@chakra-ui/react';
import ArticleListPage from "@/app/articleList/ArticleListPage";
import {nf_get} from "@/tool/httpNode";

const getServerData = async (): Promise<any> => {
    let data;
    try {
        let param = {pageIndex: 1, pageSize: 5};
        data = await nf_get('/article/list', {headers: {}, params: param, timeout: 30000});
    } catch (err) {
        return;
    }
    if (2000 != data.code) {
        return;
    }
    return data.data;
}

const HomePage = async () => {
    const serverData = await getServerData();

    const envName = process.env.NEXT_PUBLIC_ENV_NAME;
    console.log(envName);

    return (
        <Box minHeight="100vh" maxWidth="100%" display="flex" flexDirection="column">
            <ArticleListPage firstData={serverData}></ArticleListPage>
        </Box>
    );
}

export default HomePage;

