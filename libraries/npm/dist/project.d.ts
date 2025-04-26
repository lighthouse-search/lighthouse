declare function list(id?: string[]): Promise<any>;
declare const project: {
    list: typeof list;
};
export default project;
