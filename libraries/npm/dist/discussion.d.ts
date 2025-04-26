declare function list(id: string[] | undefined, filter: any): Promise<any>;
declare function update(actions: object): Promise<any>;
declare const discussion: {
    list: typeof list;
    update: typeof update;
};
export default discussion;
