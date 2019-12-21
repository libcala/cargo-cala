r#"<html><meta charset="utf-8"><meta name="viewport" content="width=device-width, initial-scale=1"><title>{}</title><link rel="icon" href="icon.svg"><head><script>fetch('app.wasm').then(response=>response.arrayBuffer()).then(function(bin){{var t;const module=new WebAssembly.Module(bin);const instance=new WebAssembly.Instance(module,{{env:{{
navigator_userAgent_Len:()=>l(navigator.userAgent),
navigator_userAgent_Ptr:(ptr)=>m(ptr),
console_warn:(ptr,len)=>console.warn(f(ptr,len)),
console_info:(ptr,len)=>console.info(f(ptr,len)),
console_debug:(ptr,len)=>console.debug(f(ptr,len)),
alert:(ptr,len)=>alert(f(ptr,len)),
}}}});var linear_memory = instance.exports.memory;function f(ptr,len){{var buffer=new Uint16Array(linear_memory.buffer,ptr,len);let str="";for(let i=0;i<buffer.length;i++){{str+=String.fromCharCode(buffer[i]);}}return str;}}function l(s){{t=new String(s);return t.length;}}function m(ptr){{var buffer=new Uint16Array(linear_memory.buffer,ptr,t.length);for(let i=0;i<t.length;i++){{buffer[i]=t.charCodeAt(i);}}}}instance.exports.main()}});</script></head><body></body></html>"#
