const __converter = require("../pkg/wasm_ascii_maker.js")

const Converter = __converter.Converter

const new_conv = new Converter()

new_conv.new_reader()
new_conv.log_out()