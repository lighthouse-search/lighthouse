async function hydrate_users(dids) {
    if (Array.isArray(dids) == false) {
        throw "dids must be array.";
    }
  
    let params = new URLSearchParams();
    await dids.forEach((did) => {
        params.append("actors", did);
    });
    const request = await fetch(`https://public.api.bsky.app/xrpc/app.bsky.actor.getProfiles?${params.toString()}`, {
        "method": "GET",
        "mode": "cors"
    });
    const response = await request.json();
  
    if (request.status == 200) {
        return response.profiles;
    } else {
        throw `hydrate_users failed, status code ${request.status} - response:
${JSON.stringify(response)}`;
    }
}

export { hydrate_users }