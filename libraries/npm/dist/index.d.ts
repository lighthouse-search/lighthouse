import { getPlatformApiURLWithoutPathname } from "./routing.js";
import device from "./device.js";
import fetch_wrapper from "./fetcher.js";
import general from "./general.js";
declare function getCreds(): {
    deviceid: string | null;
    privatekey: string;
    additional_data: any;
    type: string | null;
    fetch_properties: any;
};
declare function OnlyGetAdditionalData(): Promise<{}>;
declare function Lighthouse(credsObject: any): {
    getCreds: typeof getCreds;
    device: typeof device;
    fetch_wrapper: typeof fetch_wrapper;
    general: typeof general;
    query: {
        list: (query: string[] | undefined, filter: any) => Promise<any>;
    };
    account: {
        me: () => Promise<any>;
        list: () => Promise<any>;
        update: (data: object) => Promise<any>;
    };
    admin: {
        index: {
            job: {
                list: () => Promise<any>;
                update: (data: object) => Promise<any>;
            };
        };
    };
    metadata: {
        urls: () => Promise<any>;
    };
    getPlatformApiURLWithoutPathname: typeof getPlatformApiURLWithoutPathname;
};
export { Lighthouse, getCreds, OnlyGetAdditionalData };
