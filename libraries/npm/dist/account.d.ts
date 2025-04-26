declare function me(): Promise<any>;
declare function list(): Promise<any>;
declare function update(data: object): Promise<any>;
declare const account: {
    me: typeof me;
    list: typeof list;
    update: typeof update;
};
export default account;
