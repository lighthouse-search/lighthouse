declare function list(id: string[] | undefined, filter: any): Promise<any>;
declare function create(data: object): Promise<any>;
declare const event: {
    list: typeof list;
    create: typeof create;
};
export default event;
