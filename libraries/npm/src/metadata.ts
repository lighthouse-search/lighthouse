import general from "./general.js";
import { Lighthouse, getCreds } from "./index.js";
import { getPlatformApiURLWithoutPathname } from "./routing.js";

async function urls(): Promise<any> {
    const response = await Lighthouse(getCreds()).fetch_wrapper(`${getPlatformApiURLWithoutPathname()}/metadata/urls`, {
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
    
    const data = response.json();
    
    return data;
}

const metadata = { urls };
export default metadata;