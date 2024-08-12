"use client";

import {Box, Image, Flex, Text, Button, Container, Heading, Textarea, IconButton, Avatar, Badge} from '@chakra-ui/react';
import {useGetWrap, usePostWrap} from "@/tool/http";
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
import rehypeRaw from 'rehype-raw';
import { useSDK } from "@metamask/sdk-react";

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

    const postWrap = usePostWrap();
    const getWrap = useGetWrap();
    const [article, setArticle] = useState({
        id: 0,
        pemName: "",
        titleArticle: "",
        timePublish: 0,
        watchCount: 0,
        content: "",
        mkFooter: "",
        viewCount: 0,

    });
    const [articleLoading, setArticleLoading] = useState(true);

    const [views, setViews] = useState([]);

    const searchParams = useSearchParams();
    const id = searchParams.get('id');

    useEffect(() => {
        getArticle(id);
        getViews(id);
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

    const getViews = async (id: any) => {
        let data;
        try {
            data = await getWrap('/article/views', {params: {idBlog: id}});
        } catch (err) {
        } finally {
            setArticleLoading(false);
        }
        if (2000 != data.code) {
            return;
        }
        setViews(data.data);
    }

    const [comment, setComment] = useState("");
    const [isSubmitLoading, setSubmitLoading] = useState(false);
    const maxLength = 200;
    const handleCommentChange = (event: any) => {
        const value = event.target.value;
        if (value.length <= maxLength) {
            setComment(value);
        }
    };

    const submitView =  async () => {
        console.log(comment);
        console.log(account);
        if ("" == account){
            return;
        }

        await createView();
    }

    const createView = async () => {
        setSubmitLoading(true);
        let data;
        try {
            data = await postWrap('/article/createView', {dataIn: {
                    idBlog: article.id,
                    viewContent: comment,
                    coinSymbol: "MATIC",
                    ticket: "adasdeeasx232",
                }});
            console.log(data)
        } catch (err) {
        } finally {
            setSubmitLoading(false);
        }
        if (2000 != data.code){
            return;
        }

        // @ts-ignore
        setViews([data.data, ...views]);
        article.viewCount += 1;
        setArticle(article);
        setComment("");
    }


    const { account } = useSDK(); // 从 @metamask/sdk-react 中获取连接状态和账户信息
    const [isMetaMaskInstalled, setIsMetaMaskInstalled] = useState(false);

    useEffect(() => {
        if (typeof window.ethereum !== 'undefined' && window.ethereum.isMetaMask) {
            setIsMetaMaskInstalled(true);
        } else {
            setIsMetaMaskInstalled(false);
        }
    }, []);


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
                                    rehypePlugins={[rehypeRaw]}
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
                    <Box
                        width="100%"
                        margin="0 auto"
                        padding="4"
                        borderWidth="1px"
                        borderRadius="lg"
                        boxShadow="md"
                    >
                        <Box fontSize="xl" mb="4">
                            {article.viewCount} 条评论
                        </Box>
                        <Box position="relative">
                            <Textarea
                                placeholder="写下你的评论支持 markdown 语法"
                                resize="none"
                                mb="4"
                                width="100%"
                                rows={5}
                                value={comment}
                                onChange={handleCommentChange}
                            />
                            <Text
                                position="absolute"
                                bottom="20px"
                                right="30px"
                                fontSize="sm"
                                color="gray.500"
                            >
                                {comment.length}/{maxLength}
                            </Text>
                        </Box>
                        <Flex justify="flex-end">
                            <Button colorScheme="blue" onClick={submitView} isLoading={isSubmitLoading}>
                                提交评论
                            </Button>
                        </Flex>
                    </Box>
                    {views.map(({address, coinSymbol, createdAt, viewContent}, index) => (
                        <Box
                            key={index}
                            width="100%"
                            padding="4"
                            borderWidth="1px"
                            borderRadius="lg"
                            boxShadow="md"
                            // maxWidth="600px"
                            margin="0 auto"
                        >
                            <Flex align="center" mb="4">
                                <Avatar src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADgAAAA4CAMAAACfWMssAAAA4VBMVEX///92PRb2hRvkdhv3hhvMYBb2gADvfxv6xJ8AAADneBtsOBbVaBjtfR2/rJ1xOhbiaAB8QBZtKwDjbwD1eQD+ihsSFBbYwrSvnpHFs6bmgzxoHwC1XhkkHBb/7OBmJgD0xKrqmWXVyMJyNQDx7eqXcmC6pZuefGxzOAnBZhj6vpcAKkv98uu9bCzcbxrelmiMg33NXADmpoHn4NrMnoSeQgCBQA721sSeUhjPaxqulImOSheATjKKX0nadRqPQgD2jTJOPUDBXhxyTDz4mE3Mf0fspHl4bGZYUErpkVWdjYLIJt+5AAADDklEQVRIie2V21biMBSGKQlE05YWAjIUBkFQKGJVPBXxjOf3f6DZORWS4nJu5mLW8r/oSpP9NX+SvZtC4Uf/Tnvn3b8J657vWT2j3Yuo9x3Wiy52R1bfrhNcsrB/9TV11Q/ZZeDULKc1x3GCmDE222i5O4Oh2IWgmul1NIA+x00pZWHUm5rUtBeFjNKUc87A9Np2hIJqQhGw/a01gUXoTKqBjDG8CqdCbuwjhGh5TRQ6/I6rIwyvK1DYBRIXlTB/Q2nGmeCwnfWD3TnEFjMBN9c2udrDda8DZ01uhyKiOYLoyqbYHWNz9scGuci8gtOFwY33DfDAmJLb1Ys0bPIJD/IJYEwaCxLHrjVgJUChbQ4H1TsJ3lkTOm2TuzZnDBbIl6CPFiZZuzbA4Y3hM6Vqd2BvaGq4vTFOw1yj2+HZI0EzazascaS3NXDdmCfZCkQ0dl3td2DX4+1YUk46TwSHdN5wMpmnjmTHtxa41w4Ct9q5KzYIQjaIEGngu04V2Lb96ygc3Md+o4GxDjVBVMS40fDj+5bNFbp1jFUxbALFJmNcz/8fJmWxGcRLpDSoXj0iNqs8yYEPMjlxUlLikZjot0QNP+SdqmogHo+DhygsIprwICrp69b/qLBV1hWPvJKXPj55EvSeHlPoQLpYylubnfIpUbI8PFwm0qpsr+ra9jqtZ78KDIdGjpZzCc6XR/CaTZjzmjnl9nx+ZoSXB/YJP1ufrEYtr8/6T8FFFC5A0RbdKuLZAEMiRbnkDvrZA6tuqdAAI5Ug4hQ9UcRyLlHM4kQSFRKZa2Sq2xMhtLgmKj7nqQBm5k431Empvl1sKBWVDz0eWtmqZ+RB/LvsZec3aOeFMfE1bRQxKwFeM9IrAXbcPNneAW2fNI8BLXm6aNirBV5lIDoFrNl8k+AbNI/ZaTbIcne2XCTcjWjWguDm+7bQO2+3ZojfkWKJNlf4gMuYsT6/jYcVQD9/CX0CVhnyW7nP+LX8kQN7YTTJbDQrEC7Ugma2nEkU9nLg1Ejes0qms6+jNqol1fw28Ef/tf4AVG5J7Csoz3AAAAAASUVORK5CYII=" mr="3" />
                                <Box>
                                    <Flex align="center">
                                        <Text fontWeight="bold">{coinSymbol}</Text>
                                        <Text ml="2" color="gray.500">{address}</Text>
                                    </Flex>
                                    <Text fontSize="sm" color="gray.500">{convertTimestampToYYYYMMDD(createdAt)}</Text>
                                </Box>
                            </Flex>
                            <Box mb="4">
                                <div className="markdown-body">
                                    <ReactMarkdown
                                        remarkPlugins={[remarkGfm]}
                                        rehypePlugins={[rehypeRaw]}
                                    >
                                        {viewContent}
                                    </ReactMarkdown>
                                </div>
                            </Box>
                        </Box>
                        ))}
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
