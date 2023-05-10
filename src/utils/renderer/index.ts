import * as handlebars from 'handlebars';
// import helpers from 'handlebars-helpers';

export const render = (template: string, data: any) => {
	// handlebars.registerHelper(helpers(handlebars));
	return handlebars.compile(template)(data);
};

export const getHandlebarsVars = (template: string) => {
	const re = /(?<=\{\{\{)[^}]*(?=\}\}\})/g;
	const matches = template.match(re);
	const keys: { [key: string]: string } = { p: 'prompt' };
	if (matches) {
		for (let i = 0; i < matches.length; i += 1) {
			keys[`${matches[i].toLowerCase().charAt(0)}`] = matches[i];
		}
	}
	return keys;
};
