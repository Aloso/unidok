import * as unidok from "unidok"
import { drawSelection, EditorView, highlightActiveLine, keymap, ViewUpdate } from "@codemirror/view"
import { EditorState } from "@codemirror/state"
import { defaultKeymap, indentLess, indentMore, insertTab } from "@codemirror/commands"
import { highlightSelectionMatches } from "@codemirror/search"

export class Playground {
    private lastRenderTime = 0
    private lastRenderValue: string = null
    private isHtml = false
    private changeListeners: (() => void)[] = []
    private renderListeners: (() => void)[] = []

    private input: HTMLDivElement
    private preview: HTMLElement
    private toggleButton: HTMLElement

    private editorView: EditorView

    public value: string

    constructor(pre: HTMLElement) {
        this.value = pre.textContent
            .replace(/^\n/, '')
            .replace(/\n[ \t]*$/, '')

        this.input = document.createElement('div')
        this.input.className = 'input'

        this.initToggleButton()
        this.initInput()
        this.initPreview()

        const newElem = document.createElement('div')
        newElem.className = 'playground initialized'
        applyAttributes(pre, newElem)
        newElem.append(this.input, this.preview, this.toggleButton)
        pre.replaceWith(newElem)
    }

    initInput() {
        const updateListener = (update: ViewUpdate) => {
            if (update.docChanged) {
                this.value = this.getValue()
                this.render()
                this.changeListeners.forEach(listener => listener())
            }
        }
        const myKeymap = defaultKeymap.slice()
        myKeymap.push({
            key: 'Tab',
            preventDefault: true,
            run(target): boolean {
                if (target.state.selection.main.empty) {
                    return insertTab(target)
                } else {
                    return indentMore(target)
                }
            },
            shift(target): boolean {
                return indentLess(target)
            }
        })

        this.editorView = new EditorView({
            state: EditorState.create({
                doc: this.value,
                extensions: [
                    EditorView.lineWrapping,
                    EditorState.allowMultipleSelections.of(true),
                    drawSelection(),
                    highlightActiveLine(),
                    highlightSelectionMatches({ minSelectionLength: 3 }),
                    EditorState.tabSize.of(4),
                    EditorView.updateListener.of(updateListener),
                    keymap.of(myKeymap)
                ]
            }),
            parent: this.input,
        })
    }

    initPreview() {
        const preview = document.createElement('div')
        preview.className = 'preview'
        this.preview = preview
    }

    initToggleButton() {
        const toggleButton = document.createElement('button')
        this.toggleButton = toggleButton

        toggleButton.innerHTML = 'Show HTML'
        toggleButton.className = 'show-html'
        toggleButton.addEventListener('click', () => this.toggleHtml())
    }

    getValue(): string {
        return this.editorView.state.doc.sliceString(0)
    }

    render() {
        const value = this.value

        const now = performance.now()
        if (value === this.lastRenderValue) return
        if (now - this.lastRenderTime < 150) {
            setTimeout(() => this.render(), 170)
            return
        }

        this.lastRenderValue = value
        this.lastRenderTime = now

        let spans
        try {
            if (this.isHtml) {
                let result = unidok.compile(value)
                this.preview.innerText = result.text
                spans = result.spans
            } else {
                spans = convertToHtml(value, this.preview, false, true)
            }
        } catch (e) {
            console.warn('Input:')
            console.log(value)
            console.error(e.message)
            this.preview.innerHTML = '<p style="color:#ff4444"><strong>Fatal error</strong></p>'
        }

        this.renderListeners.forEach(listener => listener())
    }

    toggleHtml() {
        this.isHtml = !this.isHtml
        this.lastRenderValue = null
        this.render()

        if (this.isHtml) {
            this.toggleButton.innerHTML = 'Hide HTML'
            this.preview.classList.add('html')
        } else {
            this.toggleButton.innerHTML = 'Show HTML'
            this.preview.classList.remove('html')
        }
    }

    onChange(listener: () => void) {
        this.changeListeners.push(listener)
    }

    onRender(listener: () => void) {
        this.renderListeners.push(listener)
    }
}

function applyAttributes(from: HTMLElement, to: HTMLElement) {
    const style = from.getAttribute('style')
    if (style != null) {
        to.setAttribute('style', style)
    }
    const id = from.id
    if (id != null) {
        to.id = id
    }
}

export function addBigPlayground() {
    const pre = document.createElement('pre')
    pre.className = 'playground'
    pre.id = 'big-playground'
    document.body.append(pre)
    document.body.style.overflow = 'hidden'

    const oldValue = localStorage.getItem('big-playground-text')
    if (oldValue != null) {
        pre.textContent = oldValue
    }

    const playground = new Playground(pre)
    playground.onChange(() => {
        localStorage.setItem('big-playground-text', playground.value)
    })
    playground.render()

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

export function convertToHtml(
    text: string,
    target: HTMLElement,
    dont_wait?: boolean,
    retrieve_spans?: boolean
): unidok.SyntaxSpan[] {
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

function updateHtmlWithMath(target: HTMLElement) {
    const mathElems = target.getElementsByTagName('math')
    for (const elem of Array.from(mathElems)) {
        const converted = (window as any).MathJax.mathml2chtml(elem.outerHTML)
        elem.replaceWith(converted)
    }

    (window as any).MathJax.startup.document.clear();
    (window as any).MathJax.startup.document.updateDocument()
}

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
