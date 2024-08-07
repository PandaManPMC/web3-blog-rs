import {useToast} from '@chakra-ui/react';

export const useErrToast = () => {
    const toast = useToast();

    return (title: string, msg: string) => {
        toast({
            title: title,
            description: msg,
            status: 'error',
            duration: 5000,
            isClosable: true,
        });
    };
};

export const useInfoToast = () => {
    const toast = useToast();
    return (title: string) => {
        toast({
            title: title,
            status: 'info',
            duration: 3000,
            isClosable: true,
        });
    };
};

