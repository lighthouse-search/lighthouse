import { SignJWT, importPKCS8 } from "jose";

function general() {
    return {
        objectToParams: function(object: Record<string, any>) {
            let formData = new URLSearchParams(object);
            return formData.toString();
        },
        formdataToJson: function(formdata: object) {
            return {
                formdata: formdata
            }
        },
        signJWT: async function(data: any, privateKeyV: string, options: object) {
            const privateKeyPem = '-----BEGIN PRIVATE KEY-----\n' +
`${privateKeyV.replace("-----BEGIN PRIVATE KEY-----", "").replace("-----END PRIVATE KEY-----", "")}\n` +
'-----END PRIVATE KEY-----\n';

            const privateKey = await importPKCS8(privateKeyPem, "ES512");

            const jwt = await new SignJWT(data) // ECDSA with P-521 curve
            .setProtectedHeader({ alg: 'ES512' }) // Optional if you want to specify headers
            .sign(privateKey);
            
            return jwt;
        },
        sortedObject: async function(unsortedData: any) {
            let sortedData: any = {};
            
            await Object.keys(unsortedData).sort().forEach((key) => {
                sortedData[key] = unsortedData[key];
            });
    
            return sortedData;
        },
        JSONorForm: async function (variable: any) {
            if (variable instanceof FormData) {
                return 'FormData';
            }

            try {
                JSON.parse(JSON.stringify(variable));
                return 'JSON';
            } catch (error) {
            }
            
            return null;
        },
        filter_nonsense: function(variable: any) {
            if (variable == undefined) {
                return null;
            } else {
                return variable;
            }
        }
    }
}

export default general;