import {useErrToast} from "@/tool/ui";

export const get = async (uri: string, { headers = {}, params = {}, timeout = 30000 } = {})=> {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeout);
    const baseURI = process.env.NEXT_PUBLIC_API_URL;

    const queryString = new URLSearchParams(params).toString();
    const url = `${baseURI}${uri}${queryString ? `?${queryString}` : ''}`;

    try {
        const res = await fetch(url, {
            method: 'get',
            headers: {
                'Content-Type': 'application/json',
                ...headers,
            },
            signal: controller.signal,
        });

        clearTimeout(timeoutId);

        if (!res.ok) {
            console.log(JSON.stringify(res));
            throw new Error(`Failed to fetch data: ${res.statusText}`);
        }

        return await res.json();
    } catch (error) {
        // @ts-ignore
        if (error.name === 'AbortError') {
            console.error('Fetch request timed out');
        } else {
            console.error('Error fetching data:', error);
        }
        throw error;
    }
}

export const useGetWrap = ()=> {
    const errToast = useErrToast();
    return async (uri: string, {headers = {}, params = {}, timeout = 30000} = {}) => {
        let data, error;
        try {
            data = await get(uri, {headers: headers, params: params, timeout: timeout});
        } catch (err) {
            error = err;
        }
        if (error) {
            console.log(error);
            errToast(`http error`, JSON.stringify(error));
            throw error;
        }
        if (2000 != data.code) {
            errToast(`http ${data.code}`, data.tip);
            return data;
        }
        return data;
    }
}
