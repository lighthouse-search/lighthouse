declare function list(id?: string[]): Promise<any>;
declare const org: {
    list: typeof list;
};
export default org;
