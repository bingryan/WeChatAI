<p align="center">
 <img src="docs/image/logo.png?raw=true" alt="image" height="100px"/>
<h1 align="center">WeChatAI</h1>
<div align="center">
 <strong>
    Create All in One personal AI chat assistant
 </strong>
</div>
<br/>
<p align="center">
<a href="https://github.com/bingryan/WeChatAI/releases" target="_blank">
<img alt="macOS" src="https://img.shields.io/badge/-macOS-black?style=for-the-badge&logo=apple&logoColor=white" />
</a>
<a href="https://github.com/bingryan/WeChatAI/releases" target="_blank">
<img alt="Windows" src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=green" />
</a>
<a href="https://github.com/bingryan/WeChatAI/releases" target="_blank">
<img alt="Linux" src="https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black" />
</a>
</p>

<p align="center">
    English | <a href="./README-CN.md">中文介绍</a>
</p>

> This project is not for reward and profit, only used for personal

## Screenshot

![](docs/image/WeChatAI-1.png)

## Supported Service

- [OpenAI](platform.openai.com)
- [Azure OpenAI](azure.com)

## For User

### Install

Please visit [Windows, Mac(M1, intel), Linux](https://github.com/bingryan/WeChatAI/releases) Download

#### Mac User

step1: Running Unsigned Applications on Mac

```
sudo spctl --master-disable
```

step2: `xattr -cr /Applications/WeChatAI.app`


### Features and Setting

The left setting block serves as a global setting:

- [x] Language: Chinese and English
- [x] Theme   : light and dark
- [x] Custom avatar: Any image format (including gif)
- [x] User chat background: Customize chat background color
- [x] Assistant chat background: Customize chat background color
- [x] User content format: supports markdown and raw (default), chat supports markdown format questioning
- [x] Assistant content format: supports markdown (default) and raw, recommended to use markdown for easy reading
- [x] Global Chat configuration: `New` chat configuration will be read from the global configuration as the default configuration -> [generate Open AI Key](https://platform.openai.com/account/api-keys)
  - [x] ContextSize: Context chat information, default to 2, both of which as the output basis for chatgpt
- [x] System layer settings
  - [x] Shortcut: `CommandOrControl+Shift+K`
  - [x] autolaunch: default not to start, requires manual activation
- [x] Page Operations
  - [x] Drag the avatar to adjust its position
  - [x] Click on the avatar to edit and delete
  - [x] Click "+" to create a new chat window, and the default configuration information will be read from the global configuration
- [ ] integrated model
  - [x] chatgpt
  - [ ] ...
- [ ] more...

## For Developers

Any form of PR is welcome (documentation, UI, code)

## Technology Stack and Credits

- [Tauri](https://github.com/tauri-apps/tauri): Build smaller, faster, and more secure desktop applications with a web frontend.
- [Vue.js](https://vuejs.org/): An approachable, performant and versatile framework for building web user interfaces.
- [tailwindcss](https://github.com/tailwindlabs/tailwindcss): A utility-first CSS framework for rapid UI development.
- [arco UI ](https://arco.design/): arco UI
- [highlight.js](https://github.com/highlightjs/highlight.js/): Code highlight
- [markdown-it](https://github.com/markdown-it/markdown-it): markdown
- [Axum](https://github.com/tokio-rs/axum): rust web framework for building local servers
- [chatgpt-web](https://github.com/Chanzhaoyu/chatgpt-web): A front-end implementation of chatgpt, which greatly inspires the display of chat content
- [chatgpt_rs](https://github.com/Maxuss/chatgpt_rs): A chatgpt client implemented with rust, because there is a BUG in my development, and modify the code [crates/chatgpt_rs](https://github.com/bingryan/chat-ai-model/tree/main/crates/chatgpt_rs)
