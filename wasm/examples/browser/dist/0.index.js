(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "./node_modules/tstee/pure-evm.js":
/*!****************************************!*\
  !*** ./node_modules/tstee/pure-evm.js ***!
  \****************************************/
/*! exports provided: exec */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"exec\", function() { return exec; });\n/* harmony import */ var _pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./pure-evm_bg.wasm */ \"./node_modules/tstee/pure-evm_bg.wasm\");\n\n\nlet cachegetUint8Memory = null;\nfunction getUint8Memory() {\n    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== _pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory = new Uint8Array(_pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nfunction passArray8ToWasm(arg) {\n    console.log(_pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__)\n    const ptr = _pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"](arg.length * 1);\n    getUint8Memory().set(arg, ptr / 1);\n    WASM_VECTOR_LEN = arg.length;\n    return ptr;\n}\n\nlet cachegetInt32Memory = null;\nfunction getInt32Memory() {\n    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== _pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory = new Int32Array(_pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory;\n}\n\nfunction getArrayU8FromWasm(ptr, len) {\n    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);\n}\n/**\n* @param {Uint8Array} code\n* @param {Uint8Array} data\n* @returns {Uint8Array}\n*/\nfunction exec(code, data) {\n    const retptr = 8;\n    const ret = _pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"exec\"](retptr, passArray8ToWasm(code), WASM_VECTOR_LEN, passArray8ToWasm(data), WASM_VECTOR_LEN);\n    const memi32 = getInt32Memory();\n    const v0 = getArrayU8FromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();\n    _pure_evm_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);\n    return v0;\n}\n\n\n\n//# sourceURL=webpack:///./node_modules/tstee/pure-evm.js?");

/***/ }),

/***/ "./node_modules/tstee/pure-evm_bg.wasm":
/*!*********************************************!*\
  !*** ./node_modules/tstee/pure-evm_bg.wasm ***!
  \*********************************************/
/*! exports provided: memory, exec, __wbindgen_malloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///./node_modules/tstee/pure-evm_bg.wasm?");

/***/ })

}]);