import MarkdownIt from 'markdown-it';
import MarkdownItHighlightjs from 'markdown-it-highlightjs';
import mdKatex from '@traptitech/markdown-it-katex';
import mdTaskList from 'markdown-it-task-lists';

function highlightBlockWrap(str: string) {
	const temp = document.createElement('div');
	temp.innerHTML = str;
	const preList = temp.querySelectorAll('pre');
	preList.forEach((pre) => {
		const code = pre.querySelector('code') as HTMLElement;

		const clipboard = Object.assign(document.createElement('span'), {
			className: 'clipboard',
		});

		const icon = Object.assign(document.createElement('i'), {
			className: 'fa fa-clipboard',
		});

		clipboard.appendChild(icon);

		const header = document.createElement('div');
		header.classList.add('code-block-header');
		header.appendChild(clipboard);
		const wrapper = document.createElement('pre');
		wrapper.classList.add('code-block-wrapper');
		wrapper.appendChild(header);
		wrapper.appendChild(code);
		pre.replaceWith(wrapper);
	});
	return temp.innerHTML;
}
export default function markdownToHtml(
	content: any,
	wrap = highlightBlockWrap
) {
	const md = new MarkdownIt({
		html: true,
	})
		.use(MarkdownItHighlightjs)
		.use(mdTaskList)
		.use(mdKatex);

	return wrap(md.render(content));
}
