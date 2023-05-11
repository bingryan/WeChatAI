import * as handlebars from 'handlebars';
// import helpers from 'handlebars-helpers';

export const render = (template: string, data: any) => {
	// handlebars.registerHelper(helpers(handlebars));
	return handlebars.compile(template)(data);
};

export const getHandlebarsVars = (template: string) => {
	const reg = /{{{\s*(\w+)\s*}}}/g;
	const matches = template.match(reg);
	const keys: { [key: string]: string } = { p: 'prompt' };

	if (matches) {
		for (let i = 0; i < matches.length; i += 1) {
			const matchStr = matches[i].split(/{{{|}}}/)[1].trim();
			keys[`${matchStr.toLowerCase().charAt(0)}`] = matchStr;
		}
	}
	return keys;
};
