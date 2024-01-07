let devMode = true;
const pass = () => null;
const handleError = (error) => console.log(`Error: ${error}`);

chrome.downloads.onCreated.addListener(function (downloadItem) {
    const filename = downloadItem.filename
    console.log("Download filename:", filename);
    const downloadId = downloadItem.id;
    // 获取下载信息
    let downloadData = {
        title: filename,
        status: 'downloading',
        downloadId: downloadId,
        size: downloadItem.totalBytes,
        webpageUrl: downloadItem.url,
        downloadUrl: downloadItem.finalUrl,
        resumeState: downloadItem.canResume,
    };
    if (devMode) {
        console.log(downloadData);
    }
    // 发送数据到本地端口
    removeFromHistory(downloadId);
    sendDataToServer(downloadData);
});

async function removeFromHistory(downloadId) {
    await chrome.downloads.removeFile(downloadId).then(pass).catch(pass);
    await chrome.downloads.cancel(downloadId).then(pass).catch(handleError);
}

function sendDataToServer(data) {
    // 发送数据到本地端口
    fetch('http://localhost:63319', {
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
                console.log('Data sent:', data);
            }
    })
        .catch(error => {
            if (devMode) { 
                console.error('There was a problem sending the data:', error);
            }
    });
}