import fetch from 'node-fetch';

/** SSR 不能使用原始 fetch ，该使用 node-fetch **/

export const nf_get = async (uri: string, { headers = {}, params = {}, timeout = 60000 } = {}) => {
    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), timeout);
    const baseURI = process.env.NEXT_PUBLIC_API_URL;

    const queryString = new URLSearchParams(params).toString();
    const url = `${baseURI}${uri}${queryString ? `?${queryString}` : ''}`;

    if (!headers.hasOwnProperty('Content-Type')){
        headers = { ...headers, 'Content-Type': 'application/json; charset=utf-8'};
    }

    try {
        const res = await fetch(url, {
            method: 'get',
            headers: {
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
        console.error('Error fetching data:', error);
        throw error;
    }
}
