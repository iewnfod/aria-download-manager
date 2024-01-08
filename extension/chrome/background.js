let devMode = true;
const pass = () => null;
const handleError = (error) => console.log(`Error: ${error}`);

chrome.downloads.onCreated.addListener(function (downloadItem) {
    const downloadId = downloadItem.id;
    // 获取下载信息
    let downloadData = {
        download_id: downloadId,
        size: downloadItem.totalBytes,
        webpage_url: downloadItem.url,
        download_url: downloadItem.finalUrl,
        resume_state: downloadItem.canResume,
    };
    if (devMode) {
        console.log(downloadData);
        console.log(JSON.stringify(downloadData))
    }
    // 发送数据到本地端口
    removeFromHistory(downloadId);
    sendDataToServer(downloadData);
});

async function removeFromHistory(downloadId) {
    await chrome.downloads.removeFile(downloadId).then(pass).catch(pass);
    await chrome.downloads.cancel(downloadId).then(pass).catch(handleError);
}

async function fetchADMState() {
    try {
        const response = await fetch('http://127.0.0.1:63319/state', {
            method: 'GET'
        });

        if (!response.ok) {
            return false;
        }

        const data = await response.json();

        if (data && data.status === 0) {
            return true;
        }
        return false;
    } catch (error) {
        console.error('Error fetching ADM state:', error);
        return false;
    }
}

async function getADMState() {
    try {
        const result = await fetchADMState();
        console.log("ADM On State: ", result);
    } catch (error) {
        console.error('Error:', error);
    }
}

function callADM() { 

}

function sendDataToServer(data) {
    // 发送数据到本地端口
    fetch('http://127.0.0.1:63319/api', {
        method: 'POST',
        headers: {
        'Content-Type': 'application/json'
        },
        body: JSON.stringify(data)
    })
    .then(response => {
        if (!response.ok) {
            if (devMode) { 
                throw new Error('Network response was not ok');
            }
        }
        return response.json();
    })
        .then(data => {
            if (devMode) { 
                console.log('Server Responsed:', data);
            }
    })
        .catch(error => {
            if (devMode) { 
                console.error('There was a problem sending the data:', error);
            }
    });
}