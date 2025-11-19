function credentials_object(router, fetch_properties) {
    let auth_data = null;
    if (typeof window != "undefined") {
        auth_data = JSON.parse(localStorage.getItem("auth"));
    }
  
    if (!auth_data) {
        console.log("No auth data found.");
        return null;
    }
  
    return { deviceid: auth_data.device_id, privatekey: auth_data.private_key, additional_data: { org: router.query.org, namespace: router.query.namespace, project_id: router.query.project }, fetch_properties };
}

export { credentials_object };