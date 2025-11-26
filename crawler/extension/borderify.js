async function get_page_urls(text) {
    const url_regex = /(?:(?:https?):\/\/)[\w/\-?=%.]+\.[\w/\-?=%.]+/g;
    const text_array = text.match(url_regex);
    console.log("URL SPLIT", text_array);
    if (text_array == null) {
        return [];
    }

    let urls = [];
    for (const element of text_array) {
        if (element.startsWith("https://") == false && element.startsWith("http://") == false) {
            continue;
        }
        
        try {
            const url = new URL(element);

            if (url.username || url.password) {
                // Skip URLs with authentication info
                continue;
            }

            // Didn't error - we found a url!
            urls.push(url.href);
        } catch (error) {
            // Error'd - most likely not a URL.
        }
    }

    return urls;
}

async function get_metatags() {
    try {
        // Get all meta tags
        const meta_tags = document.querySelectorAll('meta');

        // Convert meta tags to a JSON object
        const meta_json = Array.from(meta_tags).reduce((acc, meta) => {
            const name = meta.getAttribute('name') || meta.getAttribute('property');
            const content = meta.getAttribute('content');

            if (name && content) {
                acc[name] = content;
            }
            return acc;
        }, {});

        return meta_json;
    } catch (error) {
        console.log("error", error);
    }
}

async function get_linktags() {
    try {
        // Get all link tags
        const link_tags = document.querySelectorAll('link');

        // Convert link tags to a JSON object
        const link_json = Array.from(link_tags).reduce((acc, link) => {
            const rel = link.getAttribute('rel');
            const href = link.getAttribute('href');

            if (rel && href) {
                if (!acc[rel]) {
                    acc[rel] = [];
                }
                acc[rel].push(href);
            }
            return acc;
        }, {});

        return link_json;
    } catch (error) {
        console.log("error", error);
    }
}

window.onload = function() {
    // setTimeout(() => { console.log("HTML", document.documentElement.outerHTML); }, 5000);
    // setTimeout(() => { console.log("HTML", document.body.innerText); }, 5000);
    setTimeout(async () => {
        this.alert("LOADED!");
        console.log("test ", document.querySelectorAll('main'));

        const urls = await get_page_urls(document.body.textContent);
        const response = await fetch("http://127.0.0.1:4459/api/native-v1/crawler/index", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                actions: [
                    {
                        url: window.location.href,
                        content: {
                            title: document.title,
                            text: document.body.outerText,
                            // html: document.documentElement.outerHTML
                            urls: urls,
                            metatag: await get_metatags(),
                            linktag: await get_linktags()
                        }
                    }
                ]
            })
        });

        console.log("RESPONSE", response.status);
        console.log("URLS", urls);
    }, 5000);
};