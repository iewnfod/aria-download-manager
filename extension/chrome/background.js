let devMode = true;
let shouldSendToServer = true;
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

// 扩展启动时从存储中读取数值
chrome.storage.local.get(['shouldSendToServer'], function (result) {
    shouldSendToServer = result.shouldSendToServer !== undefined ? result.shouldSendToServer : true;
    updateIcon();
});

// 当状态发生变化时，将其保存到存储器中
chrome.storage.local.set({ shouldSendToServer });

// 设置初始图标
updateIcon();

chrome.runtime.onInstalled.addListener(function () {
    // 在插件安装时添加点击事件监听器
    chrome.action.onClicked.addListener(function (tab) {
        shouldSendToServer = !shouldSendToServer;
        console.log("Sending to server: ", shouldSendToServer);

        // Save the state to storage when it changes
        chrome.storage.local.set({ shouldSendToServer });

        // 根据状态切换图标
        updateIcon();
    });
});

chrome.downloads.onChanged.addListener(function (downloadDelta) {
    const downloadId = downloadDelta.id;

    if (!shouldSendToServer || !downloadDelta.state || downloadDelta.state.current !== 'complete') {
        console.log("Download information will not be sent to the server.");
        return;
    }

    fetchADMState().then(admIsOpen => {
        if (!admIsOpen) {
            console.log("ADM is not open. Download information will not be sent to the server.");
            return;
        }
        if (processedDownloads.has(downloadId)) {
            console.log("Download information already processed.");
            return;
        }

        processedDownloads.add(downloadId);

        // 获取下载信息
        chrome.downloads.search({ id: downloadId }, function (results) {
            const downloadItem = results[0];

            let downloadData = {
                download_id: downloadId,
                size: downloadItem.totalBytes,
                webpage_url: downloadItem.url,
                download_url: downloadItem.finalUrl,
                resume_state: downloadItem.canResume,
            };
            if (devMode) {
                console.log(downloadData);
                console.log(JSON.stringify(downloadData));
            }

            // 发送数据到本地端口
            removeFromHistory(downloadId);
            sendDataToServer(downloadData);
        });
    });
});

// Initialize processedDownloads set
const processedDownloads = new Set();

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
        return false;
    }
}

function updateIcon() {
    const iconPath = shouldSendToServer ? defaultIconPath : pauseIconPath;
    chrome.action.setIcon({ path: iconPath });
}
