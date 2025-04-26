import general from "./general.js";
import { Lighthouse, getCreds } from "./index.js";
import { getPlatformApiURLWithoutPathname } from "./routing.js";

async function list(query: string[] = [], filter: any): Promise<any> {
    const response = await Lighthouse(getCreds()).fetch_wrapper(`${getPlatformApiURLWithoutPathname()}/query/list?${general().objectToParams({ query, filter: filter ? JSON.stringify(filter) : null })}`, {
        method: 'GET', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'default', // *default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'error', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
    })
    
    const json = response.json();
    return json;
}

const query = { list };
export default query;