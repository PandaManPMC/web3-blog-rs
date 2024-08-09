"use client"

import React from 'react';
import { Box, Heading, Text, Tag, HStack, VStack, Avatar, Flex, Icon } from '@chakra-ui/react';
import { FiBookmark } from 'react-icons/fi';
import ReactMarkdown from 'react-markdown';
import remarkGfm from 'remark-gfm';
import {convertTimestampToYYYYMMDD} from "@/tool/util";
const ArticleCard = ({data, onClick}: {data: any, onClick: any}) => {

    return (
        <Box
            borderWidth="1px"
            borderRadius="md"
            p={4}
            mb={4}
            boxShadow="sm"
            maxHeight="300px"
            onClick={onClick}>
            <Heading as="h2" size="md" isTruncated>
                {data.titleArticle}
            </Heading>
            <Box
                mt={2}
                overflow="hidden"
                display="-webkit-box"
                maxHeight="150px"
                css={{
                    WebkitLineClamp: '3',
                    WebkitBoxOrient: 'vertical',
                }}
            >
                <ReactMarkdown remarkPlugins={[remarkGfm]}>
                    {data.content.replace(/\[TOC\]/g, '')}
                </ReactMarkdown>
            </Box>

            <HStack spacing={2} mt={4} wrap="wrap">
                {data.labels.map((tag: string, index: number) => (
                    <Tag key={index} borderRadius="full" variant="solid" colorScheme="teal" size="sm">
                        {tag}
                    </Tag>
                ))}
            </HStack>

            <Flex mt={4} justify="space-between" align="center" fontSize="sm" color="gray.500">
                <HStack spacing={2}>
                    <Avatar src={data.avatar} size="sm" />
                    <VStack align="start" spacing={0}>
                        <Text>{data.pemName}</Text>
                        <Text>{convertTimestampToYYYYMMDD(data.timePublish)}</Text>
                    </VStack>
                </HStack>
                <HStack spacing={4}>
                    <Text>阅读: {data.watchCount}</Text>
                    <Text>评论: {data.viewCount}</Text>
                    <Icon as={FiBookmark} w={5} h={5} />
                </HStack>
            </Flex>
        </Box>
    );
};

export default ArticleCard;
