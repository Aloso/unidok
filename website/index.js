import * as unidok from "unidok"
import { drawSelection, EditorView, highlightActiveLine, keymap } from "@codemirror/view"
import { EditorState } from "@codemirror/state"
import { defaultKeymap } from "@codemirror/commands"
import { highlightSelectionMatches } from "@codemirror/search"
// import { highlightTree } from "@codemirror/highlight"
// import { Tree, NodeType } from "lezer-tree"

const _formattings = [
    'InlineFormatting', 'Italic', 'Bold', 'Strikethrough', 'Superscript', 'Subscript', 'InlineCode',
    'Heading', 'AtxHeading', 'SetextHeading1', 'SetextHeading2', 'AtxHeadingMarker', 'SetextHeadingMarker',
    'Link', 'LinkText', 'LinkRef', 'LinkHref', 'LinkTitle', 'LinkRefDef',
    'Image', 'ImageAltText', 'ImageHref', 'ImageTitle',
    'Footnote',
    'Blockquote', 'BlockquoteMarker',
    'List', 'OrderedList', 'UnorderedList', 'ListMarker',
    'ThematicBreak',
    'CodeBlock', 'CodeFence', 'InfoString',
    'Table', 'TableCell', 'TableCellMeta',
    'Math', 'MathContent',
    'Limiter',
    'Comment',
    'HtmlTag', 'HtmlTagName', 'HtmlAttrName', 'HtmlAttrValue',
    'HtmlDoctype',
    'HtmlCdata',
    'HtmlComment',
    'HtmlEntity',
    'Macro', 'MacroName', 'MacroArg', 'MacroKey', 'MacroArgString', 'MacroArgList', 'CurlyBraces',
    'Escaped'
]


/**
 * @typedef {{
 *   openButton: HTMLElement,
 *   content: HTMLElement,
 *   contentLoading: string,
 *   rand: number,
 * }} NavState
 */

function main() {
    document.getElementById('open-playground')
        .addEventListener('click', addBigPlayground)

    const content = document.getElementById('content')
    const contentLoading = content.innerHTML

    const nav = document.getElementById('main-nav')
    const buttons = []
    const navState = {
        openButton: null,
        content,
        contentLoading,
        rand: Math.floor(new Date() / 10000)
    }

    for (const btn of nav.children) {
        if (btn.tagName === 'BUTTON') {
            buttons.push(btn)
            btn.addEventListener('click', () => {
                openTab(btn, navState)
            })
            if ('?' + btn.getAttribute('data-cls') === window.location.search) {
                openTab(btn, navState)
            }
        }
    }
    if (navState.openButton == null) openTab(buttons[0], navState)
}
main()

/**
 * @param {HTMLElement} button
 * @param {NavState} navState
 */
function openTab(button, navState) {
    if (navState.openButton != null) {
        navState.openButton.classList.remove('open')
    }
    navState.openButton = button
    navState.openButton.classList.add('open')

    const title = `Unidok - ${button.innerText}`
    document.title = title

    const search = '?' + button.getAttribute('data-cls')
    if (search !== window.location.search) {
        history.replaceState({}, title, search)
    }

    let finishedLoading = false
    setTimeout(() => {
        if (!finishedLoading) {
            navState.content.innerHTML = navState.contentLoading
        }
    }, 1000)

    const fileName = button.getAttribute('data-file')
    fetch(`./sections/${fileName}?${navState.rand}`)
        .then(response => {
            if (response.status === 200) {
                return response.text()
            } else {
                finishedLoading = true
                throw new Error(`${response.status} ${response.statusText}`)
            }
        })
        .then(text => {
            navState.content.className = button.getAttribute('data-cls')
            convertToHtml(text, navState.content, true)
            finishedLoading = true

            const elems = document.getElementsByClassName('playground');
            for (const elem of elems) {
                initializePlayground(elem, () => { })
            }
        })
        .catch(e => {
            finishedLoading = true
            navState.content.innerHTML = `<div style="text-align: center; color: #ff7777; margin: 1.5em 0">
                Error loading the content: ${e.message}
            </div>`
            console.error(e)
        })
}

/**
 * @param {string} text
 * @param {HTMLElement} target
 * @param {boolean} dont_wait
 * @param {boolean} retrieve_spans
 * @returns {unidok.SyntaxSpan[]?}
 */
function convertToHtml(text, target, dont_wait, retrieve_spans) {
    const result = unidok.compile(text, retrieve_spans)

    if (result.contains_math) {
        if (dont_wait) {
            target.innerHTML = result.text
            updateHtmlWithMath(target)
        } else {
            setTimeout(() => {
                target.innerHTML = result.text
                updateHtmlWithMath(target)
            }, 20)
        }
    } else {
        target.innerHTML = result.text
    }

    return result.spans
}

/**
 * @param {HTMLElement} target
 */
function updateHtmlWithMath(target) {
    const mathElems = target.getElementsByTagName('math')
    /** @type {HTMLElement[]} */
    const mathElemsCopy = []
    for (const elem of mathElems) {
        mathElemsCopy.push(elem)
    }
    for (const elem of mathElemsCopy) {
        const converted = MathJax.mathml2chtml(elem.outerHTML)
        elem.replaceWith(converted)
    }

    MathJax.startup.document.clear()
    MathJax.startup.document.updateDocument()
}

/**
 * @param {HTMLElement} elem
 * @param {(string) => void} onChange
 */
function initializePlayground(elem, onChange) {
    const content = elem.textContent
        .replace(/^\n/, '')
        .replace(/\n[ \t]*$/, '')

    const newElem = document.createElement('div')
    newElem.className = 'playground initialized'

    const inputOuter = document.createElement('div')
    inputOuter.className = 'input'

    const input = new EditorView({
        state: EditorState.create({
            doc: content,
            extensions: [
                EditorView.lineWrapping,
                EditorState.allowMultipleSelections.of(true),
                drawSelection(),
                highlightActiveLine(),
                highlightSelectionMatches({
                    minSelectionLength: 3,
                }),
                EditorState.tabSize.of(4),
                EditorView.updateListener.of((update) => {
                    if (update.docChanged) {
                        render(onChange)
                    }
                }),
                keymap.of(defaultKeymap)
            ]
        }),
        parent: inputOuter,
    })

    const preview = document.createElement('div')
    preview.className = 'preview'

    const htmlButton = document.createElement('button')
    htmlButton.innerHTML = 'Show HTML'
    htmlButton.className = 'show-html'

    const style = elem.getAttribute('style')
    if (style != null) {
        newElem.setAttribute('style', style)
    }
    const id = elem.id
    if (id != null) {
        newElem.id = id
    }
    newElem.append(inputOuter, preview, htmlButton)
    elem.replaceWith(newElem)

    let last_render = 0
    let last_value = null
    let is_html = false

    /**
     * @param {(string) => void} onChange
     */
    function render(onChange) {
        const values = input.state.doc
        let value = ''
        for (const v of values) {
            value += v
        }

        const now = performance.now()
        if (value === last_value) return
        if (now - last_render < 150) {
            setTimeout(() => render(onChange), 170)
            return
        }

        last_value = value
        last_render = now

        let spans
        try {
            if (is_html) {
                let result = unidok.compile(value)
                preview.innerText = result.text
                spans = result.spans
            } else {
                spans = convertToHtml(value, preview, false, true)
            }
        } catch (e) {
            console.warn('Input:')
            console.log(value)
            console.error(e.message)
            preview.innerHTML = '<p style="color:#ff4444"><strong>Fatal error</strong></p>'
        }

        if (onChange != null) onChange(value)
    }
    render(onChange)

    htmlButton.addEventListener('click', () => {
        is_html = !is_html
        last_value = null
        render()

        if (is_html) {
            htmlButton.innerHTML = 'Hide HTML'
            preview.classList.add('html')
        } else {
            htmlButton.innerHTML = 'Show HTML'
            preview.classList.remove('html')
        }
    })
}

function addBigPlayground() {
    const elem = document.createElement('pre')
    elem.className = 'playground'
    elem.id = 'big-playground'
    document.body.append(elem)
    document.body.style.overflow = 'hidden'

    const oldValue = localStorage.getItem('big-playground-text')
    if (oldValue != null) {
        elem.textContent = oldValue
    }

    initializePlayground(elem, (value) => {
        localStorage.setItem('big-playground-text', value)
    })

    const closeBtn = document.createElement('button')
    closeBtn.innerHTML = 'Close playground'
    closeBtn.id = 'close-big-playground'
    document.body.append(closeBtn)

    setTimeout(() => {
        const newElem = document.getElementById('big-playground')

        newElem.addEventListener('keydown', e => {
            if (e.code === 'Escape') close()
        })
        closeBtn.addEventListener('click', close)

        function close() {
            newElem.remove()
            closeBtn.remove()
            document.body.style.overflow = ''
        }
    })
}
