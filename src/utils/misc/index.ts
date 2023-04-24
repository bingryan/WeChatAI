export function parseDomain(url: string) {
	const { hostname } = new URL(url);
	const domain = hostname.split('.').slice(-2).join('.');
	const tld = hostname.split('.').slice(-1).join('');
	return {
		hostname,
		domain,
		tld,
	};
}

export function fileSaver(data: string, filename: string) {
	const blob = new Blob([data], { type: 'octet/stream;charset=utf-8' });
	const url = URL.createObjectURL(blob);
	const link = document.createElement('a');
	link.setAttribute('href', url);
	link.setAttribute('download', filename);
	document.body.appendChild(link);
	link.click();
	document.body.removeChild(link);
	URL.revokeObjectURL(url);
}
