"use client";

import {Box, Image, Flex, Text, Button, Container, Heading} from '@chakra-ui/react';
import {useGetWrap} from "@/tool/http";
import { useSearchParams  } from 'next/navigation';
import {convertTimestampToYYYYMMDD} from "@/tool/util";
import 'github-markdown-css/github-markdown.css';
import React, {Suspense, useEffect, useState} from 'react';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import remarkParse from 'remark-parse';
import { unified } from 'unified';
import 'github-markdown-css/github-markdown.css';
import {useDispatch, useSelector} from "react-redux";
import {AppDispatch, RootState, setArticleTOCState} from "@/storage/store";
import SyntaxHighlighter from "react-syntax-highlighter";
import { dracula } from "react-syntax-highlighter/dist/esm/styles/hljs";

const ArticlePage = () => {
    const dispatch = useDispatch<AppDispatch>();
    const author = useSelector((state: RootState) => state.author);

    const SelectId = useSelector((state: RootState) => state.ArticleTOCStateSliceReducer).SelectId;
    useEffect(() => {
        if ("" == SelectId) {
            return;
        }
        scrollToAnchor(SelectId);
    }, [SelectId]);

    const scrollToAnchor = (slug: string) => {
        const element = document.getElementById(slug);
        console.log(slug);
        if (element) {
            const yOffset = -10;
            const y = element.getBoundingClientRect().top + window.pageYOffset + yOffset;
            window.scrollTo({ top: y, behavior: 'smooth' });
        }
    };


    const getWrap = useGetWrap();
    const [article, setArticle] = useState({
        pemName: "",
        titleArticle: "",
        timePublish: 0,
        watchCount: 0,
        content: "",
        mkFooter: ""
    });
    const [articleLoading, setArticleLoading] = useState(true);

    const searchParams = useSearchParams();
    const id = searchParams.get('id');

    useEffect(() => {
        getArticle(id);
    }, []);

    const getArticle = async (id: any) => {
        let data;
        try {
            data = await getWrap('/article/read', {params: {id: id}});
        } catch (err) {
        }finally {
            setArticleLoading(false);
        }
        if (2000 != data.code) {
            return;
        }
        setArticle(data.data);

        const art = data.data;
        const processor = unified().use(remarkParse);
        const tree = processor.parse(art.content + art.mkFooter);

        const headings: { level: any; text: any; slug: string; }[] = [];
        const walker = (node: any) => {
            if (node.type === 'heading') {
                const text = node.children.map((n: any) => n.value).join('');
                const slug = "TOC" + text;
                headings.push({ level: node.depth, text, slug });
            }
            if (node.children) {
                node.children.forEach(walker);
            }
        };
        tree.children.forEach(walker);

        // @ts-ignore
        dispatch(setArticleTOCState({TOC: headings, SelectId: ""}));
    }

    return (
        <Box minHeight="100vh" maxWidth="100%" display="flex" flexDirection="column">
            { articleLoading ? (
                <Box display="flex" justifyContent="center" mt={4}>
                    <Button isLoading={articleLoading} loadingText="加载中" colorScheme="gray" ></Button>
                </Box>
            ) : (
                <>
                    <Container maxW="container.md" py={10}>
                        <Heading as="h1" size="xl" mb={4}>
                            {article.titleArticle}
                        </Heading>
                        <Flex alignItems="center" marginBottom={1}>
                            <Box>
                                <Image
                                    src={"" + author.profile}
                                    alt="作者头像"
                                    boxSize="35px"
                                    borderRadius="full"
                                    objectFit="cover"
                                />
                            </Box>
                            <Text ml={3} fontSize="lg" color="gray.700">
                                {author.penName}
                            </Text>
                        </Flex>
                        <Text fontSize="sm" color="gray.500" mb={6}>
                             更新于 {convertTimestampToYYYYMMDD(article.timePublish)} 阅读数 {article.watchCount}
                        </Text>

                        <main style={{flex: '3'}}>
                            <div className="markdown-body">
                                <ReactMarkdown
                                    remarkPlugins={[remarkGfm]}
                                    components={{
                                        // @ts-ignore
                                        code({node, inline, className, children, ...props}) {
                                            const match = /language-(\w+)/.exec(className || '');
                                            return !inline && match ? (
                                                <SyntaxHighlighter language={match[1]} style={dracula} PreTag="div">
                                                    {String(children).replace(/\n$/, '')}
                                                </SyntaxHighlighter>
                                            ) : (
                                                <code className={className} {...props}>
                                                    {children}
                                                </code>
                                            );
                                        },
                                        h1: ({children}) => {
                                            // @ts-ignore
                                            const text = children.toString();
                                            const slug = "TOC" + text;
                                            return <h1 id={slug}>{children}</h1>;
                                        },
                                        h2: ({children}) => {
                                            // @ts-ignore
                                            const text = children.toString();
                                            const slug = "TOC" + text;
                                            return <h2 id={slug}>{children}</h2>;
                                        },
                                        h3: ({children}) => {
                                            // @ts-ignore
                                            const text = children.toString();
                                            const slug = "TOC" + text;
                                            return <h3 id={slug}>{children}</h3>;
                                        },
                                        h4: ({children}) => {
                                            // @ts-ignore
                                            const text = children.toString();
                                            const slug = "TOC" + text;
                                            return <h4 id={slug}>{children}</h4>;
                                        },
                                        h5: ({children}) => {
                                            // @ts-ignore
                                            const text = children.toString();
                                            const slug = "TOC" + text;
                                            return <h5 id={slug}>{children}</h5>;
                                        },
                                        h6: ({children}) => {
                                            // @ts-ignore
                                            const text = children.toString();
                                            const slug = "TOC" + text;
                                            return <h6 id={slug}>{children}</h6>;
                                        },
                                    }}
                                >
                                    {article.content + article.mkFooter}
                                </ReactMarkdown>
                            </div>
                        </main>

                    </Container>
                </>
            )}
        </Box>
    )
        ;
};

export default function Article() {
    return (
        <Suspense fallback={<div>Loading...</div>}>
            <ArticlePage />
        </Suspense>
    );
}
