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
