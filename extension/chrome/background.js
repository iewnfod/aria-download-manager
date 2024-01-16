let devMode = true;
let shouldSendToServer = true;
const blockedMimeTypes = ['application/pdf', /* 其他MIME类型 */];
const pass = () => null;
const handleError = (error) => console.log(`Error: ${error}`);

// 初始图标路径
const defaultIconPath = {
    "16": "img/icon16.png",
    "48": "img/icon48.png",
    "128": "img/icon128.png"
};

// 暂停时的图标路径
const pauseIconPath = {
    "16": "img/pause_icon16.png",
    "48": "img/pause_icon48.png",
    "128": "img/pause_icon128.png"
};

// 设置初始图标
chrome.action.setIcon({ path: defaultIconPath });

chrome.runtime.onInstalled.addListener(function () {
    // 在插件安装时添加点击事件监听器
    chrome.action.onClicked.addListener(function (tab) {
        shouldSendToServer = !shouldSendToServer;
        console.log("Sending to server: ", shouldSendToServer);

        // 根据状态切换图标
        const newIconPath = shouldSendToServer ? defaultIconPath : pauseIconPath;
        chrome.action.setIcon({ path: newIconPath });
    });
});

chrome.downloads.onCreated.addListener(function (downloadItem) {
    if (!shouldSendToServer) {
        console.log("Download information will not be sent to the server.");
        return;
    }

    if (downloadItem.state !== 'in_progress') {
        return;
    }

    if (blockedMimeTypes.includes(downloadItem.mime)) {
        console.log("Download of " + downloadItem.mime + " files is not allowed.");
        return;
    }
    
    const downloadId = downloadItem.id;
    // 获取下载信息
    let downloadData = {
        downloadID: downloadId,
        size: downloadItem.totalBytes,
        webpageUrl: downloadItem.url,
        downloadUrl: downloadItem.finalUrl,
        resumeState: downloadItem.canResume,
    };

    chrome.cookies.getAll({ url: downloadItem.url }, function (cookies) {
        downloadData.cookies = cookies;
    });

    if (devMode) {
        console.log(downloadData);
        console.log(JSON.stringify(downloadData));
    }
    // 发送数据到本地端口
    if (getADMState()) {
        removeFromHistory(downloadId);
        sendDataToServer(downloadData);
    }
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
        // console.error('Error fetching ADM state:', error);
        pass();
        return false;
    }
}

async function getADMState() {
    try {
        const result = await fetchADMState();
        console.log("ADM On State: ", result);
        return result;
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