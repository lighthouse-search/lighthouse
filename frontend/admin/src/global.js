function credentials_object(router, fetch_properties) {
    if (typeof localStorage === "undefined") {
        console.log("LocalStorage is not available.");
        return null;
    }
    const auth_data = JSON.parse(localStorage.getItem("auth"));
  
    if (!auth_data) {
        console.log("No auth data found.");
        return null;
    }
  
    return { deviceid: auth_data.device_id, privatekey: auth_data.private_key, additional_data: { org: router.query.org, namespace: router.query.namespace, project_id: router.query.project }, fetch_properties };
}

function array_string_every_item(arr, string) {
    if (arr.length == 1) {
        return arr;
    }
    return arr.flatMap(item => [item, string]);
}

export { credentials_object, array_string_every_item };