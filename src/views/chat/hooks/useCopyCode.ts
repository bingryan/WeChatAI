import { onMounted, onUpdated } from 'vue';
import { copyText } from '@/utils/format';

export function useCopyCode() {
	function copyCodeBlock() {
		const codeBlockWrapper = document.querySelectorAll('.code-block-wrapper');
		codeBlockWrapper.forEach((wrapper) => {
			const copyBtn = wrapper.querySelector('.clipboard');
			const codeBlock = wrapper.querySelector('code');
			if (copyBtn && codeBlock) {
				copyBtn.addEventListener('click', () => {
					if (navigator.clipboard?.writeText)
						navigator.clipboard.writeText(codeBlock.textContent ?? '');
					else copyText({ text: codeBlock.textContent ?? '', origin: true });
				});
			}
		});
	}

	onMounted(() => copyCodeBlock());

	onUpdated(() => copyCodeBlock());
}
