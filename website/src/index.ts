import { Playground, addBigPlayground, convertToHtml } from "./playground"

class NavState {
    private openButton: HTMLElement
    private rand = Math.floor(+new Date() / 60_000)

    constructor(
        private content: HTMLElement,
        private contentLoading: string,
    ) { }

    get isTabOpened(): boolean {
        return this.openButton != null
    }

    openTab(button: HTMLElement) {
        if (this.openButton != null) {
            this.openButton.classList.remove('open')
        }
        this.openButton = button
        this.openButton.classList.add('open')

        const title = `Unidok - ${button.innerText}`
        document.title = title

        const search = '?' + button.getAttribute('data-cls')
        if (search !== window.location.search) {
            history.replaceState({}, title, search)
        }

        let finishedLoading = false
        setTimeout(() => {
            if (!finishedLoading) {
                this.content.innerHTML = this.contentLoading
            }
        }, 1000)

        const fileName = button.getAttribute('data-file')
        fetch(`./sections/${fileName}?${this.rand}`)
            .then(response => {
                if (response.status === 200) {
                    return response.text()
                } else {
                    finishedLoading = true
                    throw new Error(`${response.status} ${response.statusText}`)
                }
            })
            .then(text => {
                this.content.className = button.getAttribute('data-cls')
                convertToHtml(text, this.content, true, false)
                finishedLoading = true

                const elems = document.getElementsByClassName('playground');
                for (const pre of Array.from(elems)) {
                    if (pre instanceof HTMLElement) {
                        new Playground(pre).render()
                    }
                }
            })
            .catch(e => {
                finishedLoading = true
                this.content.innerHTML = `<div style="text-align: center; color: #ff7777; margin: 1.5em 0">
                Error loading the content: ${e.message}
            </div>`
                console.error(e)
            })
    }
}

export function main() {
    document.getElementById('open-playground')
        .addEventListener('click', addBigPlayground)

    const content = document.getElementById('content')
    const contentLoading = content.innerHTML

    const nav = document.getElementById('main-nav')
    const buttons = []
    const navState = new NavState(content, contentLoading)

    for (const btn of Array.from(nav.children)) {
        if (btn.tagName === 'BUTTON' && btn instanceof HTMLElement) {
            buttons.push(btn)
            btn.addEventListener('click', () => {
                navState.openTab(btn)
            })
            if ('?' + btn.getAttribute('data-cls') === window.location.search) {
                navState.openTab(btn)
            }
        }
    }
    if (!navState.isTabOpened) {
        navState.openTab(buttons[0])
    }
}
