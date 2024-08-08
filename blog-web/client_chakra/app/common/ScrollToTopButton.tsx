import React from 'react';
import { IconButton, useColorModeValue } from '@chakra-ui/react';
import { ChevronUpIcon } from '@chakra-ui/icons';

const ScrollToTopButton = () => {
    const handleScrollToTop = () => {
        window.scrollTo({
            top: 0,
            behavior: 'smooth',
        });
    };

    const bgColor = useColorModeValue('rgba(255, 255, 255, 0.6)', 'rgba(0, 0, 0, 0.6)');

    return (
        <IconButton
            aria-label="Scroll to top"
            icon={<ChevronUpIcon />}
            position="fixed"
            bottom="100px"
            right="100px"
            onClick={handleScrollToTop}
            backgroundColor={bgColor}
            _hover={{ backgroundColor: useColorModeValue('rgba(255, 255, 255, 0.8)', 'rgba(0, 0, 0, 0.8)') }}
            borderRadius="full"
            boxShadow="md"
            zIndex="1000"
        />
    );
};

export default ScrollToTopButton;
