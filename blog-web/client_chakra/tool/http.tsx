import {useErrToast} from "@/tool/ui";

export const get = async (uri: string, { headers = {}, params = {}, timeout = 60000 } = {})=> {
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

async function postData(url = '', {data = {}, headers = {}, options = {}} = {}) {
    const baseURI = process.env.NEXT_PUBLIC_API_URL;
    try {
        const response = await fetch(baseURI + url, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
                ...headers,
            },
            body: JSON.stringify(data),
            ...options,
        });

        if (!response.ok) {
            throw new Error(`HTTP error! Status: ${response.status}`);
        }

        const responseData = await response.json();
        return responseData;
    } catch (error) {
        console.error('Error:', error);
        throw error;
    }
}

export const usePostWrap = ()=> {
    const errToast = useErrToast();
    return async (uri : string, {dataIn = {}, headers = {}, options = {}} = {}) => {
        let data, error;
        try {
            data = await postData(uri, {data: dataIn, headers: headers, options: options});
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
