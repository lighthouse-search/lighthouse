declare const admin: {
    index: {
        job: {
            list: () => Promise<any>;
            update: (data: object) => Promise<any>;
        };
    };
};
export default admin;
