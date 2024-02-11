/**
 * @name chat.js
 * @version 0.1.4
 * @url https://github.com/lencx/ChatGPT/tree/main/scripts/chat.js
 */

function chatInit() {
  const ICONS = {
    copy: `<svg class="chatappico copy" stroke="currentColor" fill="none" stroke-width="2" viewBox="0 0 24 24" stroke-linecap="round" stroke-linejoin="round" class="h-4 w-4" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"></path><rect x="8" y="2" width="8" height="4" rx="1" ry="1"></rect></svg>`,
    cpok: `<svg class="chatappico cpok" viewBox="0 0 24 24"><g fill="none" stroke="#10a37f" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"><rect width="8" height="4" x="8" y="2" rx="1" ry="1"/><path d="M8 4H6a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-2M16 4h2a2 2 0 0 1 2 2v4m1 4H11"/><path d="m15 10l-4 4l4 4"/></g></svg>`,
  };

  let currentUtterance = null;
  let currentIndex = -1;
  let chatConf = {};

  async function init() {
    chatConf = (await invoke('get_app_conf')) || {};

    new MutationObserver(observeMutations).observe(document.body, {
      childList: true,
      subtree: true,
    });

    document.addEventListener('visibilitychange', focusOnInput);
    // autoContinue();
  }

  function observeMutations(mutationsList) {
    for (const mutation of mutationsList) {
      if (mutation.target.closest('form')) {
        addChatButtons();
      }
    }
  }

  function focusOnInput() {
    const textArea = document.getElementsByTagName('textarea')[0];
    if (textArea) {
      textArea.focus();
    }
  }

  function addChatButtons() {
    const list = Array.from(document.querySelectorAll('main >div>div>div>div>div'));

    list.forEach((item, idx) => {
      if (shouldSkip(item)) {
        return;
      }

      const saybtn = item.querySelector('button.rounded-md').cloneNode(true);
      saybtn.classList.add('chat-item-voice');
      saybtn.title = 'Say';
      saybtn.innerHTML = ICONS.voice;

      item.querySelector('.self-end').appendChild(saybtn);

      saybtn.onclick = () => handleClick(item, idx, saybtn);
    });
  }

  function shouldSkip(item) {
    return !item.querySelector('button.rounded-md') || !item.querySelector('.self-end');
  }

  init();
}

document.addEventListener('DOMContentLoaded', chatInit);
