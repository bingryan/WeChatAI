import * as handlebars from 'handlebars';
import helpers from 'handlebars-helpers';

export const render = (template: string, data: any) => {
	handlebars.registerHelper(helpers(handlebars));
	return handlebars.compile(template)(data);
};
