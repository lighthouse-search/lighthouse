// Helper + shared id for the developer settings dialog. Kept in its own module
// (separate from the component) so the component file only exports a React
// component — otherwise Next.js Fast Refresh bails to a full page reload.

export const DIALOG_ID = "developer_settings_dialog";

// Open the dialog from anywhere (it's mounted once, globally, in Base).
export function open_developer_settings() {
    if (typeof document == "undefined") { return; }
    const dialog = document.getElementById(DIALOG_ID);
    if (dialog && typeof dialog.showModal == "function") {
        dialog.showModal();
    }
}
