(window["webpackJsonp"] = window["webpackJsonp"] || []).push([[0],{

/***/ "../pkg/unidoc_playground.js":
/*!***********************************!*\
  !*** ../pkg/unidoc_playground.js ***!
  \***********************************/
/*! exports provided: compile, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./unidoc_playground_bg.wasm */ \"../pkg/unidoc_playground_bg.wasm\");\n/* harmony import */ var _unidoc_playground_bg_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./unidoc_playground_bg.js */ \"../pkg/unidoc_playground_bg.js\");\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"compile\", function() { return _unidoc_playground_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"compile\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return _unidoc_playground_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_new_59cb74e423758ede\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return _unidoc_playground_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_stack_558ba5917b466edd\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return _unidoc_playground_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbg_error_4bb6c2a97407129a\"]; });\n\n/* harmony reexport (safe) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return _unidoc_playground_bg_js__WEBPACK_IMPORTED_MODULE_1__[\"__wbindgen_object_drop_ref\"]; });\n\n\n\n\n//# sourceURL=webpack:///../pkg/unidoc_playground.js?");

/***/ }),

/***/ "../pkg/unidoc_playground_bg.js":
/*!**************************************!*\
  !*** ../pkg/unidoc_playground_bg.js ***!
  \**************************************/
/*! exports provided: compile, __wbg_new_59cb74e423758ede, __wbg_stack_558ba5917b466edd, __wbg_error_4bb6c2a97407129a, __wbindgen_object_drop_ref */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* WEBPACK VAR INJECTION */(function(module) {/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"compile\", function() { return compile; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_new_59cb74e423758ede\", function() { return __wbg_new_59cb74e423758ede; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_stack_558ba5917b466edd\", function() { return __wbg_stack_558ba5917b466edd; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbg_error_4bb6c2a97407129a\", function() { return __wbg_error_4bb6c2a97407129a; });\n/* harmony export (binding) */ __webpack_require__.d(__webpack_exports__, \"__wbindgen_object_drop_ref\", function() { return __wbindgen_object_drop_ref; });\n/* harmony import */ var _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./unidoc_playground_bg.wasm */ \"../pkg/unidoc_playground_bg.wasm\");\n\n\nconst heap = new Array(32).fill(undefined);\n\nheap.push(undefined, null, true, false);\n\nfunction getObject(idx) { return heap[idx]; }\n\nlet heap_next = heap.length;\n\nfunction dropObject(idx) {\n    if (idx < 36) return;\n    heap[idx] = heap_next;\n    heap_next = idx;\n}\n\nfunction takeObject(idx) {\n    const ret = getObject(idx);\n    dropObject(idx);\n    return ret;\n}\n\nlet WASM_VECTOR_LEN = 0;\n\nlet cachegetUint8Memory0 = null;\nfunction getUint8Memory0() {\n    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetUint8Memory0 = new Uint8Array(_unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetUint8Memory0;\n}\n\nconst lTextEncoder = typeof TextEncoder === 'undefined' ? (0, module.require)('util').TextEncoder : TextEncoder;\n\nlet cachedTextEncoder = new lTextEncoder('utf-8');\n\nconst encodeString = (typeof cachedTextEncoder.encodeInto === 'function'\n    ? function (arg, view) {\n    return cachedTextEncoder.encodeInto(arg, view);\n}\n    : function (arg, view) {\n    const buf = cachedTextEncoder.encode(arg);\n    view.set(buf);\n    return {\n        read: arg.length,\n        written: buf.length\n    };\n});\n\nfunction passStringToWasm0(arg, malloc, realloc) {\n\n    if (realloc === undefined) {\n        const buf = cachedTextEncoder.encode(arg);\n        const ptr = malloc(buf.length);\n        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);\n        WASM_VECTOR_LEN = buf.length;\n        return ptr;\n    }\n\n    let len = arg.length;\n    let ptr = malloc(len);\n\n    const mem = getUint8Memory0();\n\n    let offset = 0;\n\n    for (; offset < len; offset++) {\n        const code = arg.charCodeAt(offset);\n        if (code > 0x7F) break;\n        mem[ptr + offset] = code;\n    }\n\n    if (offset !== len) {\n        if (offset !== 0) {\n            arg = arg.slice(offset);\n        }\n        ptr = realloc(ptr, len, len = offset + arg.length * 3);\n        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);\n        const ret = encodeString(arg, view);\n\n        offset += ret.written;\n    }\n\n    WASM_VECTOR_LEN = offset;\n    return ptr;\n}\n\nlet cachegetInt32Memory0 = null;\nfunction getInt32Memory0() {\n    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer) {\n        cachegetInt32Memory0 = new Int32Array(_unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"memory\"].buffer);\n    }\n    return cachegetInt32Memory0;\n}\n\nconst lTextDecoder = typeof TextDecoder === 'undefined' ? (0, module.require)('util').TextDecoder : TextDecoder;\n\nlet cachedTextDecoder = new lTextDecoder('utf-8', { ignoreBOM: true, fatal: true });\n\ncachedTextDecoder.decode();\n\nfunction getStringFromWasm0(ptr, len) {\n    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));\n}\n/**\n* @param {string} input\n* @returns {string}\n*/\nfunction compile(input) {\n    try {\n        const retptr = _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](-16);\n        var ptr0 = passStringToWasm0(input, _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n        var len0 = WASM_VECTOR_LEN;\n        _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"compile\"](retptr, ptr0, len0);\n        var r0 = getInt32Memory0()[retptr / 4 + 0];\n        var r1 = getInt32Memory0()[retptr / 4 + 1];\n        return getStringFromWasm0(r0, r1);\n    } finally {\n        _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_add_to_stack_pointer\"](16);\n        _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](r0, r1);\n    }\n}\n\nfunction addHeapObject(obj) {\n    if (heap_next === heap.length) heap.push(heap.length + 1);\n    const idx = heap_next;\n    heap_next = heap[idx];\n\n    heap[idx] = obj;\n    return idx;\n}\n\nfunction __wbg_new_59cb74e423758ede() {\n    var ret = new Error();\n    return addHeapObject(ret);\n};\n\nfunction __wbg_stack_558ba5917b466edd(arg0, arg1) {\n    var ret = getObject(arg1).stack;\n    var ptr0 = passStringToWasm0(ret, _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_malloc\"], _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_realloc\"]);\n    var len0 = WASM_VECTOR_LEN;\n    getInt32Memory0()[arg0 / 4 + 1] = len0;\n    getInt32Memory0()[arg0 / 4 + 0] = ptr0;\n};\n\nfunction __wbg_error_4bb6c2a97407129a(arg0, arg1) {\n    try {\n        console.error(getStringFromWasm0(arg0, arg1));\n    } finally {\n        _unidoc_playground_bg_wasm__WEBPACK_IMPORTED_MODULE_0__[\"__wbindgen_free\"](arg0, arg1);\n    }\n};\n\nfunction __wbindgen_object_drop_ref(arg0) {\n    takeObject(arg0);\n};\n\n\n/* WEBPACK VAR INJECTION */}.call(this, __webpack_require__(/*! ./../www/node_modules/webpack/buildin/harmony-module.js */ \"./node_modules/webpack/buildin/harmony-module.js\")(module)))\n\n//# sourceURL=webpack:///../pkg/unidoc_playground_bg.js?");

/***/ }),

/***/ "../pkg/unidoc_playground_bg.wasm":
/*!****************************************!*\
  !*** ../pkg/unidoc_playground_bg.wasm ***!
  \****************************************/
/*! exports provided: memory, compile, __wbindgen_add_to_stack_pointer, __wbindgen_malloc, __wbindgen_realloc, __wbindgen_free */
/***/ (function(module, exports, __webpack_require__) {

eval("\"use strict\";\n// Instantiate WebAssembly module\nvar wasmExports = __webpack_require__.w[module.i];\n__webpack_require__.r(exports);\n// export exports from WebAssembly module\nfor(var name in wasmExports) if(name != \"__webpack_init__\") exports[name] = wasmExports[name];\n// exec imports from WebAssembly module (for esm order)\n/* harmony import */ var m0 = __webpack_require__(/*! ./unidoc_playground_bg.js */ \"../pkg/unidoc_playground_bg.js\");\n\n\n// exec wasm module\nwasmExports[\"__webpack_init__\"]()\n\n//# sourceURL=webpack:///../pkg/unidoc_playground_bg.wasm?");

/***/ }),

/***/ "./index.js":
/*!******************!*\
  !*** ./index.js ***!
  \******************/
/*! no exports provided */
/***/ (function(module, __webpack_exports__, __webpack_require__) {

"use strict";
eval("__webpack_require__.r(__webpack_exports__);\n/* harmony import */ var unidoc_playground__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! unidoc-playground */ \"../pkg/unidoc_playground.js\");\n\n\ndocument.getElementById('open-playground')\n    .addEventListener('click', addBigPlayground)\n\nconst nav = document.getElementById('main-nav')\nconst buttons = []\nlet openButton = null\nfor (const btn of nav.children) {\n    if (btn.tagName === 'BUTTON') {\n        buttons.push(btn)\n        btn.addEventListener('click', () => {\n            openTab(btn)\n        })\n        if ('#' + btn.getAttribute('data-cls') === window.location.hash) {\n            openTab(btn)\n        }\n    }\n}\nif (openButton == null) openTab(buttons[0])\n\nfunction openTab(button) {\n    if (openButton != null) {\n        openButton.classList.remove('open')\n    }\n    openButton = button\n    openButton.classList.add('open')\n    window.location.hash = button.getAttribute('data-cls')\n\n    const fileName = button.getAttribute('data-file')\n    fetch(`./sections/${fileName}`)\n        .then(response => response.text())\n        .then(text => {\n            const content = document.getElementById('content')\n            content.className = button.getAttribute('data-cls')\n            convertToHtml(text, content)\n\n            const elems = document.getElementsByClassName('playground');\n            for (const elem of elems) {\n                initializePlayground(elem)\n            }\n        })\n        .catch(e => console.error(e))\n}\n\n/**\n * @param {HTMLElement} target\n */\nfunction convertToHtml(text, target) {\n    target.innerHTML = unidoc_playground__WEBPACK_IMPORTED_MODULE_0__[\"compile\"](text)\n\n    const mathElems = target.getElementsByTagName('math')\n    /** @type {HTMLElement[]} */\n    const mathElemsCopy = []\n    for (const elem of mathElems) {\n        mathElemsCopy.push(elem)\n    }\n    if (mathElemsCopy.length > 0) {\n        for (const elem of mathElemsCopy) {\n            const converted = MathJax.mathml2chtml(elem.outerHTML)\n            elem.replaceWith(converted)\n        }\n\n        MathJax.startup.document.clear()\n        MathJax.startup.document.updateDocument()\n    }\n}\n\n/**\n * @param {HTMLElement} elem\n */\nfunction initializePlayground(elem) {\n    const content = elem.textContent\n        .replace(/^\\n/, '')\n        .replace(/\\n[ \\t]*$/, '')\n\n    const input = document.createElement('textarea')\n    input.className = 'input'\n    input.value = content\n    input.setAttribute('placeholder', 'Type here...')\n\n    const preview = document.createElement('div')\n    preview.className = 'preview'\n\n    const htmlButton = document.createElement('button')\n    htmlButton.innerHTML = 'Show HTML'\n    htmlButton.className = 'show-html'\n\n    const newElem = document.createElement('div')\n    newElem.className = 'playground initialized'\n\n    const style = elem.getAttribute('style')\n    if (style != null) {\n        newElem.setAttribute('style', style)\n    }\n    const id = elem.id\n    if (id != null) {\n        newElem.id = id\n    }\n    newElem.append(input, preview, htmlButton)\n    elem.replaceWith(newElem)\n\n    let last_render = 0\n    let last_value = null\n    let is_html = false\n\n    function render() {\n        const value = input.value\n        const now = performance.now()\n        if (value === last_value) return\n        if (now - last_render < 150) {\n            setTimeout(() => render(), 170)\n            return\n        }\n\n        last_value = value\n        last_render = now\n\n        try {\n            // don't block during keypress\n            setTimeout(() => {\n                if (is_html) {\n                    preview.innerText = unidoc_playground__WEBPACK_IMPORTED_MODULE_0__[\"compile\"](value)\n                } else {\n                    convertToHtml(value, preview)\n                }\n            }, 20)\n        } catch (e) {\n            console.warn('Input:')\n            console.log(value)\n            console.error(e.message)\n            preview.innerHTML = '<p style=\"color:#ff4444\"><strong>Fatal error</strong></p>'\n        }\n    }\n    render()\n\n    input.addEventListener('keypress', () => render())\n    input.addEventListener('input', () => render())\n    input.addEventListener('focus', () => render())\n\n    htmlButton.addEventListener('click', () => {\n        is_html = !is_html\n        last_value = null\n        render()\n\n        if (is_html) {\n            htmlButton.innerHTML = 'Hide HTML'\n            preview.classList.add('html')\n        } else {\n            htmlButton.innerHTML = 'Show HTML'\n            preview.classList.remove('html')\n        }\n    })\n}\n\nfunction addBigPlayground() {\n    const elem = document.createElement('pre')\n    elem.className = 'playground'\n    elem.id = 'big-playground'\n    document.body.append(elem)\n    document.body.style.overflow = 'hidden'\n\n    initializePlayground(elem)\n\n    const closeBtn = document.createElement('button')\n    closeBtn.innerHTML = 'Close playground'\n    closeBtn.id = 'close-big-playground'\n    document.body.append(closeBtn)\n\n    setTimeout(() => {\n        const newElem = document.getElementById('big-playground')\n        const ta = newElem.children[0]\n\n        const oldValue = localStorage.getItem('big-playground-text')\n        if (oldValue != null) {\n            ta.value = oldValue\n        }\n\n        ta.focus();\n\n        newElem.addEventListener('keydown', e => {\n            if (e.code === 'Escape') close()\n        })\n        ta.addEventListener('input', () => {\n            localStorage.setItem('big-playground-text', ta.value)\n        })\n        closeBtn.addEventListener('click', close)\n\n        function close() {\n            newElem.remove()\n            closeBtn.remove()\n            document.body.style.overflow = ''\n        }\n    })\n}\n\n\n//# sourceURL=webpack:///./index.js?");

/***/ }),

/***/ "./node_modules/webpack/buildin/harmony-module.js":
/*!*******************************************!*\
  !*** (webpack)/buildin/harmony-module.js ***!
  \*******************************************/
/*! no static exports found */
/***/ (function(module, exports) {

eval("module.exports = function(originalModule) {\n\tif (!originalModule.webpackPolyfill) {\n\t\tvar module = Object.create(originalModule);\n\t\t// module.parent = undefined by default\n\t\tif (!module.children) module.children = [];\n\t\tObject.defineProperty(module, \"loaded\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.l;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"id\", {\n\t\t\tenumerable: true,\n\t\t\tget: function() {\n\t\t\t\treturn module.i;\n\t\t\t}\n\t\t});\n\t\tObject.defineProperty(module, \"exports\", {\n\t\t\tenumerable: true\n\t\t});\n\t\tmodule.webpackPolyfill = 1;\n\t}\n\treturn module;\n};\n\n\n//# sourceURL=webpack:///(webpack)/buildin/harmony-module.js?");

/***/ })

}]);