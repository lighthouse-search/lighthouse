#!/bin/bash

function coastguard_report() {
    coastguard test status -id f117bf7c-d4a7-11ef-a084-0242ac110002 -status event
}
# Test if we can reach example.com
if curl -s --fail http://example.com | jq -e '.success == true' > /dev/null
then
    echo "example.com is reachable."
    # Add commands to run on success here
else
    echo "example.com is not reachable."
    # Add commands to run on failure here
fi