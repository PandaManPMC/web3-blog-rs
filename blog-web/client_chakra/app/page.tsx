"use client"

import React, {useEffect, useState} from 'react';
import Image from "next/image";
import Link from 'next/link';
import ArticleCard from "@/app/common/ArticleCard";
import { Box, Button } from '@chakra-ui/react';
import {useGetWrap} from "@/tool/http";

let pageIndex = 1;
const pageSize = 5;

const Home = () => {
    const envName = process.env.ENV_NAME;
    console.log(envName);
    const getWrap = useGetWrap();

    const [articlesLst, setArticlesLst] = useState([]);
    const [articlesLstLoading, setArticlesLstLoading] = useState(true);
    const [noMore, setNoMore] = useState(false);

    useEffect(() => {
        pageIndex = 1;
        getArticlesLst(pageIndex);
    }, []);

    const getArticlesLst = async (pageIndex_: number) => {
        console.log(pageIndex_);

        let data;
        try {
            data = await getWrap('/article/list', {params: {pageIndex: pageIndex_, pageSize: pageSize}});
        } catch (err) {
            return;
        }finally {
            setArticlesLstLoading(false);
        }
        // console.log(data);

        if (2000 != data.code) {
            return;
        }
        console.log("data.len="+ data.data.length + ",articlesLst.len=" + articlesLst.length);

        if (0 == data.data.length){
            setNoMore(true);
            return;
        }

        // @ts-ignore
        setArticlesLst([...articlesLst, ...data.data]);
    }

    const loadMoreArticles = () => {
        setArticlesLstLoading(true);
        pageIndex += 1;
        console.log(pageIndex);
        getArticlesLst(pageIndex);
    };

    const handleArticleClick = (article: any) => {
        console.log(1223);
        console.log(article);
    }

    return (
        <Box minHeight="100vh" maxWidth="100vh" display="flex" flexDirection="column">
            <Box p={4}>
                {articlesLst.map((article, index) => (
                    <ArticleCard key={index} data={article} onClick={() => handleArticleClick(article)}/>
                ))}
                {noMore ? (
                    <Box display="flex" justifyContent="center" mt={4}>
                        没有更多
                    </Box>
                ) : (
                    <Box display="flex" justifyContent="center" mt={4}>
                        <Button onClick={loadMoreArticles} isLoading={articlesLstLoading} loadingText="加载中" colorScheme="gray" >
                            加载更多
                        </Button>
                    </Box>
                )}
            </Box>
        </Box>
    );
}

export default Home;

