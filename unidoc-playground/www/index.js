import * as wasm from "unidoc-playground"

const elems = document.getElementsByClassName('playground');
for (const elem of elems) {
    initializePlayground(elem)
}

document.getElementById('open-playground')
    .addEventListener('click', addBigPlayground)

/**
 * @param {HTMLElement} elem
 */
function initializePlayground(elem) {
    const content = elem.textContent
        .replace(/^\n/, '')
        .replace(/\n[ \t]*$/, '')

    const input = document.createElement('textarea')
    input.className = 'input'
    input.value = content
    input.setAttribute('placeholder', 'Type here...')

    const preview = document.createElement('div')
    preview.className = 'preview'

    const htmlButton = document.createElement('button')
    htmlButton.innerHTML = 'Show HTML'
    htmlButton.className = 'show-html'

    const newElem = document.createElement('div')
    newElem.className = 'playground initialized'

    const style = elem.getAttribute('style')
    if (style != null) {
        newElem.setAttribute('style', style)
    }
    const id = elem.id
    if (id != null) {
        newElem.id = id
    }
    newElem.append(input, preview, htmlButton)
    elem.replaceWith(newElem)

    let last_render = 0
    let last_value = null
    let is_html = false

    function render() {
        const value = input.value
        const now = performance.now()
        if (value === last_value) return
        if (now - last_render < 150) {
            setTimeout(() => render(), 170)
            return
        }

        last_value = value
        last_render = now

        try {
            // don't block during keypress
            setTimeout(() => {
                if (is_html) {
                    preview.innerText = wasm.compile(value)
                } else {
                    preview.innerHTML = wasm.compile(value)

                    const mathElems = preview.getElementsByTagName('math')
                    /** @type {HTMLElement[]} */
                    const mathElemsCopy = []
                    for (const elem of mathElems) {
                        mathElemsCopy.push(elem)
                    }
                    for (const elem of mathElemsCopy) {
                        const converted = MathJax.mathml2chtml(elem.outerHTML)
                        elem.replaceWith(converted)

                        MathJax.startup.document.clear()
                        MathJax.startup.document.updateDocument()
                    }
                }
            }, 20)
        } catch (e) {
            console.warn('Input:')
            console.log(value)
            console.error(e.message)
            preview.innerHTML = '<p style="color:#ff4444"><strong>Fatal error</strong></p>'
        }
    }
    render()

    input.addEventListener('keypress', () => render())
    input.addEventListener('input', () => render())
    input.addEventListener('focus', () => render())

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

    initializePlayground(elem)

    const closeBtn = document.createElement('button')
    closeBtn.innerHTML = 'Close playground'
    closeBtn.id = 'close-big-playground'
    document.body.append(closeBtn)

    setTimeout(() => {
        const newElem = document.getElementById('big-playground')
        const ta = newElem.children[0]

        const oldValue = localStorage.getItem('big-playground-text')
        if (oldValue != null) {
            ta.value = oldValue
        }

        ta.focus();

        newElem.addEventListener('keydown', e => {
            if (e.code === 'Escape') close()
        })
        ta.addEventListener('input', () => {
            localStorage.setItem('big-playground-text', ta.value)
        })
        closeBtn.addEventListener('click', close)

        function close() {
            newElem.remove()
            closeBtn.remove()
            document.body.style.overflow = ''
        }
    })
}
