"use strict";

module.exports = function isNegativeZero(value) {
	return Object.is(value, -0);
};
