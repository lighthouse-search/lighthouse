import general from "./general.js";
import { Lighthouse, getCreds } from "./index.js";
import { getPlatformApiURLWithoutPathname } from "./routing.js";

async function list(id: string[] = [], filter: any): Promise<any> {
    id = general().filter_nonsense(id);
    const response = await Lighthouse(getCreds()).fetch_wrapper(`${getPlatformApiURLWithoutPathname()}/discussion/list?${general().objectToParams({ id, filter: filter ? JSON.stringify(filter) : null })}`, {
        method: 'GET', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'default', // *default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
        },
        redirect: 'error', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
    });
    
    const data = await response.json();
    
    return data;
}

async function update(actions: object): Promise<any> {
    const response = await Lighthouse(getCreds()).fetch_wrapper(`${getPlatformApiURLWithoutPathname()}/discussion/update`, {
        method: 'POST', // *GET, POST, PUT, DELETE, etc.
        mode: 'cors', // no-cors, *cors, same-origin
        cache: 'default', // *default, no-cache, reload, force-cache, only-if-cached
        credentials: 'same-origin', // include, *same-origin, omit
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ actions }),
        redirect: 'error', // manual, *follow, error
        referrerPolicy: 'no-referrer', // no-referrer, *no-referrer-when-downgrade, origin, origin-when-cross-origin, same-origin, strict-origin, strict-origin-when-cross-origin, unsafe-url
    })
    
    const data = await response.json();
    
    return data;
}

const discussion = { list, update };
export default discussion;