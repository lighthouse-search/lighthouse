import { getCreds } from "./index.js";

function getPlatformApiURLWithoutPathname(): string {
    const creds = getCreds();
    if (typeof localStorage != "undefined" && localStorage.getItem("custom_api") != null) {
        let custom_api_str = localStorage.getItem("custom_api");
        if (custom_api_str === null) {
            throw new Error("custom_api is null");
        }
        let custom_api = new URL(custom_api_str);
        return remove_trailing_slash(custom_api.href);
    } else if (typeof window != "undefined") {
        let api_url = new URL(`${window.location.protocol}//${window.location.host}/api/native-v1`);
        return remove_trailing_slash(api_url.href);
    } else if (creds && creds.additional_data.endpoint) {
        return remove_trailing_slash(creds.additional_data.endpoint);
    } else {
        throw "Could not get API url (is the window api available?)";
    }
}

function remove_trailing_slash(href: string): string {
    if (href.endsWith("/")) {
        const index = href.length-1;
        return href.slice(0, index) + href.slice(index + 1);
    } else {
        return href;
    }
}

export { getPlatformApiURLWithoutPathname };