"use strict";

module.exports = function leftpad(text, length, padding) {
	String(text).padStart(length, padding || "0");
};
