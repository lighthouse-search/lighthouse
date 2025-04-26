import { Coastguard } from "@oracularhades/lighthouse";

// Simulated function that throws an error
function generateError(): void {
    //
    //
    throw new Error("This is a deliberately thrown error!");
}

// Function to send error details to a server using fetch
async function sendErrorDetails(error: Error): Promise<void> {
    const errorPayload = {
        message: error.message,
        stack: error.stack || "No stack trace available",
        timestamp: new Date().toISOString()
    };

    // deviceIDG = credsObject.deviceid;
    // privateKeyG = credsObject.privatekey;
    // additional_data = credsObject.additional_data;
    // typeG = credsObject.type;
    // fetch_properties = credsObject.fetch_properties;

    try {
        const stackTrace = errorPayload.stack.split("\n").slice(1).join("\n").trimStart();
        const response = await Coastguard({ deviceid: "RNYAXIUXGDGISHVFFVSM1721686105254", privatekey: "MIHuAgEAMBAGByqGSM49AgEGBSuBBAAjBIHWMIHTAgEBBEIBI+7VtelV7Cqd7x/7SUq/XbM7c+STDKzqjUZKLaQUM5F3dX7oixlF+Xa0IzchqkHhSdxTHcZVAneCRf9+Id20b12hgYkDgYYABAFaUITIvT36tlyY7D2FQqfBvz1JSk6SCeRAkh5goKJA1fU3DVPVmOm0lcB2sIVuqlh971nB5H0ngjjuwcUvT3J8+AGkJFVxv8NkQuBu5IPyOs/nmonU0VtfJoizoVpby/g7cYkx57/GC7BRbMSOWzBV9fWjDLN/jRCLoG+eu5AMqORP1Q==", additional_data: { project_id: "d3103f56-d4a8-11ef-a084-0242ac110002", endpoint: "http://127.0.0.1:4455/api/native-v1" } })
        .event.create({ actions: [{ type: "error", nonce: stackTrace, alias: errorPayload.message, content: JSON.stringify(errorPayload), metadata: {
                user: {
                    email: "hi@example.com",
                    cool_embed_object: {
                        cool: "coolest"
                    }
                }
            }} 
        ]})

        if (!response.ok) {
            throw new Error(`Failed to send error details: ${response.statusText}`);
        }

        console.log("Error details sent successfully.");
    } catch (networkError) {
        console.error("Failed to send error details:", networkError);
    }
}

// Main function to execute
async function main() {
    const response = await Coastguard({ deviceid: "RNYAXIUXGDGISHVFFVSM1721686105254", privatekey: "MIHuAgEAMBAGByqGSM49AgEGBSuBBAAjBIHWMIHTAgEBBEIBI+7VtelV7Cqd7x/7SUq/XbM7c+STDKzqjUZKLaQUM5F3dX7oixlF+Xa0IzchqkHhSdxTHcZVAneCRf9+Id20b12hgYkDgYYABAFaUITIvT36tlyY7D2FQqfBvz1JSk6SCeRAkh5goKJA1fU3DVPVmOm0lcB2sIVuqlh971nB5H0ngjjuwcUvT3J8+AGkJFVxv8NkQuBu5IPyOs/nmonU0VtfJoizoVpby/g7cYkx57/GC7BRbMSOWzBV9fWjDLN/jRCLoG+eu5AMqORP1Q==", additional_data: { project_id: "d3103f56-d4a8-11ef-a084-0242ac110002", endpoint: "http://127.0.0.1:4455/api/native-v1" } })
    .event.create({ actions: [{ type: "request", nonce: "Startup", alias: "Startup", content: `Startup at ${new Date().toLocaleDateString()}`, metadata: {
            user: {
                email: "hi@example.com",
                cool_embed_object: {
                    cool: "coolest"
                }
            }
        }} 
    ]}).catch((error) => { console.log(error); });

    try {
        generateError();
    } catch (error) {
        if (error instanceof Error) {
            console.error("Caught an error:", error.message);
            await sendErrorDetails(error);
        } else {
            console.error("Unknown error type caught.");
        }
    }
}

main();
