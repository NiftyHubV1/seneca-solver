const request = indexedDB.open('firebaseLocalStorageDb', 1);

request.onerror = (event) => {
    console.error(`Database error: ${event.target.errorCode}. Please try signing out and back in again. If that fails, report this issue at https://github.com/ArcaEge/seneca-solver/issues`);
};

request.onsuccess = (event) => {
    let db = event.target.result;
    const transaction = db.transaction('firebaseLocalStorage', 'readonly');
    const store = transaction.objectStore('firebaseLocalStorage');

    let query = store.getAll();

    query.onsuccess = (event) => {
        let value = event.target.result["0"].value;
        let apiKey = value.apiKey;
        let refreshToken = value.stsTokenManager.refreshToken;

        console.log("Successfully extracted keys. Downloading...");

        let json = {
            apiKey: apiKey,
            refreshToken: refreshToken
        };

        const blob = new Blob([JSON.stringify(json)], { type: 'application/json' });
        const link = document.createElement('a');
        link.href = URL.createObjectURL(blob);
        link.download = "seneca-solver-keys.json";
        document.body.appendChild(link);
        link.click();
        document.body.removeChild(link);

        console.log("Download complete. You can now close this tab.");
        console.log("Important: Treat this file as a password and do not share it with anyone, otherwise they can access your Seneca account.");
    };

    query.onerror = (event) => {
        console.error(`Error extracting keys: ${event.target.errorCode}. Please report this issue at https://github.com/ArcaEge/seneca-solver/issues`);
    }

    transaction.oncomplete = function () {
        db.close();
    };
};