
// window.ethereum.request 的 error 判断
// if (error.info.error.code == 4001){
//     // 主动取消
// }

export const eth_chainId = async () => {
    if (!window.ethereum) {
        throw new Error("not found window.ethereum");
    }
    try {
        return await window.ethereum.request({method: 'eth_chainId'});
    }catch (error) {
        throw error
    }
}

