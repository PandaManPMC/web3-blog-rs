import React, { useState } from 'react';
import { Box, Heading, Wrap, WrapItem } from '@chakra-ui/react';
import TextList from "@/app/common/TextList";

const TagList = ({ title, tags, onTagClick, renderItem }: {title: any, tags: any, onTagClick: any, renderItem: any}) => {
    const [selectedTag, setSelectedTag] = useState(null);

    const handleClick = (tag: any, index: any) => {
        if (index == selectedTag) {
            setSelectedTag(null);
        } else {
            setSelectedTag(index);
        }
        onTagClick(tag);
    };

    return (
        <Box p={5}>
            <Heading as="h3" size="lg" mb={4}>
                {title}
            </Heading>
            <Wrap spacing={2}>
                {tags.map((tag: any, index: any) => (
                    <WrapItem
                        key={index}
                        onClick={() => handleClick(tag, index)}
                        cursor="pointer"
                        bg={selectedTag === index ? 'blue.100' : 'white'}
                        _hover={{ bg: 'blue.50' }}
                        p={2}
                        borderRadius="md"
                        fontSize="xs"
                        border="1px"
                        borderColor="gray.200"
                    >
                        {renderItem ? renderItem(tag) : tag}
                    </WrapItem>
                ))}
            </Wrap>
        </Box>
    );
};

export default TagList;
