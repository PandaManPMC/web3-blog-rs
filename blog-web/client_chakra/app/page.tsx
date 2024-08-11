"use client"

import React, {useEffect, useState} from 'react';
import ArticleCard from "@/app/common/ArticleCard";
import { Box, Button } from '@chakra-ui/react';
import {useGetWrap} from "@/tool/http";
import Link from 'next/link';
import {usePathname, useRouter} from 'next/navigation';
import {useSelector} from "react-redux";
import {RootState} from "@/storage/store";
let pageIndex = 1;
const pageSize = 3;

const HomePage = () => {
    const envName = process.env.ENV_NAME;
    console.log(envName);
    const getWrap = useGetWrap();
    const [articlesLst, setArticlesLst] = useState([]);
    const [articlesLstLoading, setArticlesLstLoading] = useState(true);
    const [noMore, setNoMore] = useState(false);
    const selectedQuery = useSelector((state: RootState) => state.selectedQuery);

    const isHomePage = usePathname() == "/";

    useEffect(() => {
        if (!isHomePage) {
            return;
        }
        pageIndex = 1;
        getArticlesLst(pageIndex);
    }, []);

    useEffect(() => {
        // console.log("useEffect Advertise.tsx selectedQuery=", selectedQuery)
        if (undefined == selectedQuery) {
            return;
        }
        if (0 > selectedQuery.idBlogLabel && 0 > selectedQuery.idBlogClasses) {
            return;
        }

        pageIndex = 1;
        setNoMore(false);
        setArticlesLstLoading(true);
        setArticlesLst([]);
        getArticlesLst(pageIndex);
    }, [selectedQuery]);

    const getArticlesLst = async (pageIndex_: number) => {
        // console.log(pageIndex_);
        let param = {pageIndex: pageIndex_, pageSize: pageSize};

        if (selectedQuery) {
            if (0 < selectedQuery.idBlogLabel) {
                // @ts-ignore
                param["idBlogLabel"] = selectedQuery.idBlogLabel;
            }
            if (0 < selectedQuery.idBlogClasses) {
                // @ts-ignore
                param["idBlogClasses"] = selectedQuery.idBlogClasses;
            }
        }

        let data;
        try {
            data = await getWrap('/article/list', {params: param});
        } catch (err) {
            return;
        }finally {
            setArticlesLstLoading(false);
        }
        // console.log(data);

        if (2000 != data.code) {
            return;
        }
        // console.log("data.len="+ data.data.length + ",articlesLst.len=" + articlesLst.length);

        if (0 == data.data.length){
            setNoMore(true);
            return;
        }

        if (1 == pageIndex_) {
            // @ts-ignore
            setArticlesLst([...data.data]);
        }else{
            // @ts-ignore
            setArticlesLst([...articlesLst, ...data.data]);
        }
    }

    const loadMoreArticles = () => {
        setArticlesLstLoading(true);
        pageIndex += 1;
        console.log(pageIndex);
        getArticlesLst(pageIndex);
    };

    return (
        <Box minHeight="100vh" maxWidth="100%" display="flex" flexDirection="column">
            <Box p={4}>
                {articlesLst.map((article, index) => (
                    // @ts-ignore
                    <Link key={index} href={{pathname: '/article', query: {id: article.id}}} passHref target="_blank" rel="noopener noreferrer">
                        <ArticleCard key={index} data={article} onClick={()=> {}}/>
                    </Link>
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

export default HomePage;

