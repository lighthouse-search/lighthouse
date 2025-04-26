declare function general(): {
    objectToParams: (object: Record<string, any>) => string;
    formdataToJson: (formdata: object) => {
        formdata: object;
    };
    signJWT: (data: any, privateKeyV: string, options: object) => Promise<string>;
    sortedObject: (unsortedData: any) => Promise<any>;
    JSONorForm: (variable: any) => Promise<"FormData" | "JSON" | null>;
    filter_nonsense: (variable: any) => any;
};
export default general;
