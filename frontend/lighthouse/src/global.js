function credentials_object(router, fetch_properties) {
    if (typeof localStorage == "undefined") { return null; }
    const auth_data = typeof localStorage != "undefined" ? JSON.parse(localStorage.getItem("auth")) : null;
  
    if (!auth_data) {
        console.log("No auth data found.");
        return null;
    }
  
    return { deviceid: auth_data.device_id, privatekey: auth_data.private_key, additional_data: { org: router.query.org, namespace: router.query.namespace, project_id: router.query.project }, fetch_properties };
}

export { credentials_object };