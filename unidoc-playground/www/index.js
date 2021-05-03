import * as wasm from "unidoc-playground"

function start() {
    let input = document.getElementById('input')
    let preview = document.getElementById('preview')

    let last_render = 0
    let last_value = null

    function render() {
        let value = input.value
        let now = performance.now()
        if (value === last_value) return
        if (now - last_render < 100) {
            setTimeout(() => render(), 120)
            return
        }

        last_value = value
        last_render = now
        preview.innerHTML = wasm.compile(value)
    }
    render()

    input.addEventListener('keypress', () => render())
    input.addEventListener('input', () => render())
}

start()
