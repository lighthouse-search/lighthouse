declare function list(query: string[] | undefined, filter: any): Promise<any>;
declare const query: {
    list: typeof list;
};
export default query;
