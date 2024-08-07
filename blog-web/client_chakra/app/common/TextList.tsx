import React, { useState } from 'react';
import { Box, Heading, List, ListItem } from '@chakra-ui/react';

const TextList = ({ title, items, onItemClick, renderItem }: {title: string, items: any, onItemClick: any, renderItem: any}) => {
    const [selectedItem, setSelectedItem] = useState(null);
    const handleClick = (item: any, index: React.SetStateAction<null>) => {
        if (index == selectedItem) {
            setSelectedItem(null);
        }else{
            setSelectedItem(index);
        }
        onItemClick(item);
    };
    return (
        <Box p={5}>
            <Heading as="h3" size="lg" mb={4}>
                {title}
            </Heading>
            <List spacing={3}>
                {items.map((item: any, index: any) => (
                    <ListItem
                        cursor="pointer"
                        bg={selectedItem === index ? 'blue.100' : 'white'}
                        _hover={{ bg: 'blue.50' }}
                        p={2}
                        borderRadius="md"
                        fontSize="xs"
                        key={index}
                        onClick={() => handleClick(item, index)}>{renderItem ? renderItem(item) : item}</ListItem>
                ))}
            </List>
        </Box>
    );
};

export default TextList;
