declare function list(id?: string[]): Promise<any>;
declare const namespace: {
    list: typeof list;
};
export default namespace;
