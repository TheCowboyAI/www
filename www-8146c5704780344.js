let Q=0,L=null,N=`undefined`,T=`boolean`,U=`string`,M=1,W=`Object`,O=`utf-8`,S=`number`,X=4,R=`function`,J=Array,V=Array.isArray,P=Error,K=undefined;var D=(async(a,b)=>{if(typeof Response===R&&a instanceof Response){if(typeof WebAssembly.instantiateStreaming===R){try{return await WebAssembly.instantiateStreaming(a,b)}catch(b){if(a.headers.get(`Content-Type`)!=`application/wasm`){console.warn(`\`WebAssembly.instantiateStreaming\` failed because your server does not serve wasm with \`application/wasm\` MIME type. Falling back to \`WebAssembly.instantiate\` which is slower. Original error:\\n`,b)}else{throw b}}};const c=await a.arrayBuffer();return await WebAssembly.instantiate(c,b)}else{const c=await WebAssembly.instantiate(a,b);if(c instanceof WebAssembly.Instance){return {instance:c,module:a}}else{return c}}});var u=(a=>{const b=typeof a;if(b==S||b==T||a==L){return `${a}`};if(b==U){return `"${a}"`};if(b==`symbol`){const b=a.description;if(b==L){return `Symbol`}else{return `Symbol(${b})`}};if(b==R){const b=a.name;if(typeof b==U&&b.length>Q){return `Function(${b})`}else{return `Function`}};if(V(a)){const b=a.length;let c=`[`;if(b>Q){c+=u(a[Q])};for(let d=M;d<b;d++){c+=`, `+ u(a[d])};c+=`]`;return c};const c=/\[object ([^\]]+)\]/.exec(toString.call(a));let d;if(c.length>M){d=c[M]}else{return toString.call(a)};if(d==W){try{return `Object(`+ JSON.stringify(a)+ `)`}catch(a){return W}};if(a instanceof P){return `${a.name}: ${a.message}\n${a.stack}`};return d});var F=((a,b)=>{});var x=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h63c06145b6c59cc8(b,c)});var g=(a=>{const b=c(a);f(a);return b});var e=(a=>{if(d===b.length)b.push(b.length+ M);const c=d;d=b[c];b[c]=a;return c});var z=((b,c)=>{a.wasm_bindgen__convert__closures__invoke0_mut__h7751d172c49e9d22(b,c)});function B(b,c){try{return b.apply(this,c)}catch(b){a.__wbindgen_exn_store(e(b))}}var C=((a,b)=>{if(a===Q){return c(b)}else{return k(a,b)}});var I=(async(b)=>{if(a!==K)return a;if(typeof b===N){b=new URL(`www-8146c5704780344_bg.wasm`,import.meta.url)};const c=E();if(typeof b===U||typeof Request===R&&b instanceof Request||typeof URL===R&&b instanceof URL){b=fetch(b)};F(c);const {instance:d,module:e}=await D(await b,c);return G(d,e)});var r=(()=>{if(q===L||q.byteLength===Q){q=new Int32Array(a.memory.buffer)};return q});var p=(a=>a===K||a===L);var c=(a=>b[a]);var H=(b=>{if(a!==K)return a;const c=E();F(c);if(!(b instanceof WebAssembly.Module)){b=new WebAssembly.Module(b)};const d=new WebAssembly.Instance(b,c);return G(d,b)});var E=(()=>{const b={};b.wbg={};b.wbg.__wbindgen_object_clone_ref=(a=>{const b=c(a);return e(b)});b.wbg.__wbindgen_object_drop_ref=(a=>{g(a)});b.wbg.__wbindgen_string_new=((a,b)=>{const c=k(a,b);return e(c)});b.wbg.__wbindgen_cb_drop=(a=>{const b=g(a).original;if(b.cnt--==M){b.a=Q;return !0};const c=!1;return c});b.wbg.__wbindgen_jsval_eq=((a,b)=>{const d=c(a)===c(b);return d});b.wbg.__wbindgen_string_get=((b,d)=>{const e=c(d);const f=typeof e===U?e:K;var g=p(f)?Q:o(f,a.__wbindgen_malloc,a.__wbindgen_realloc);var h=l;r()[b/X+ M]=h;r()[b/X+ Q]=g});b.wbg.__wbindgen_is_string=(a=>{const b=typeof c(a)===U;return b});b.wbg.__wbindgen_number_get=((a,b)=>{const d=c(b);const e=typeof d===S?d:K;t()[a/8+ M]=p(e)?Q:e;r()[a/X+ Q]=!p(e)});b.wbg.__wbindgen_is_undefined=(a=>{const b=c(a)===K;return b});b.wbg.__wbindgen_is_null=(a=>{const b=c(a)===L;return b});b.wbg.__wbindgen_boolean_get=(a=>{const b=c(a);const d=typeof b===T?(b?M:Q):2;return d});b.wbg.__wbg_instanceof_Window_99dc9805eaa2614b=(a=>{let b;try{b=c(a) instanceof Window}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_document_5257b70811e953c0=(a=>{const b=c(a).document;return p(b)?Q:e(b)});b.wbg.__wbg_location_0f233324e8e8c699=(a=>{const b=c(a).location;return e(b)});b.wbg.__wbg_history_370f36be0803466b=function(){return B((a=>{const b=c(a).history;return e(b)}),arguments)};b.wbg.__wbg_scrollTo_fa65cff02c3e7c5a=((a,b,d)=>{c(a).scrollTo(b,d)});b.wbg.__wbg_requestAnimationFrame_1820a8e6b645ec5a=function(){return B(((a,b)=>{const d=c(a).requestAnimationFrame(c(b));return d}),arguments)};b.wbg.__wbg_body_3eb73da919b867a1=(a=>{const b=c(a).body;return p(b)?Q:e(b)});b.wbg.__wbg_createComment_ce9f467394242d45=((a,b,d)=>{var f=C(b,d);const g=c(a).createComment(f);return e(g)});b.wbg.__wbg_createDocumentFragment_229f723f44e69ab9=(a=>{const b=c(a).createDocumentFragment();return e(b)});b.wbg.__wbg_createElement_1a136faad4101f43=function(){return B(((a,b,d)=>{var f=C(b,d);const g=c(a).createElement(f);return e(g)}),arguments)};b.wbg.__wbg_createElementNS_d47e0c50fa2904e0=function(){return B(((a,b,d,f,g)=>{var h=C(b,d);var i=C(f,g);const j=c(a).createElementNS(h,i);return e(j)}),arguments)};b.wbg.__wbg_createTextNode_dbdd908f92bae1b1=((a,b,d)=>{var f=C(b,d);const g=c(a).createTextNode(f);return e(g)});b.wbg.__wbg_getElementById_00904c7c4a32c23b=((a,b,d)=>{var f=C(b,d);const g=c(a).getElementById(f);return p(g)?Q:e(g)});b.wbg.__wbg_setinnerHTML_99deeacfff0ae4cc=((a,b,d)=>{var e=C(b,d);c(a).innerHTML=e});b.wbg.__wbg_getAttribute_14ccdc738c4f7d95=((b,d,e,f)=>{var g=C(e,f);const h=c(d).getAttribute(g);var i=p(h)?Q:o(h,a.__wbindgen_malloc,a.__wbindgen_realloc);var j=l;r()[b/X+ M]=j;r()[b/X+ Q]=i});b.wbg.__wbg_hasAttribute_40947267e195b26c=((a,b,d)=>{var e=C(b,d);const f=c(a).hasAttribute(e);return f});b.wbg.__wbg_removeAttribute_5c264e727b67dbdb=function(){return B(((a,b,d)=>{var e=C(b,d);c(a).removeAttribute(e)}),arguments)};b.wbg.__wbg_scrollIntoView_31df5d1d2bc727ab=(a=>{c(a).scrollIntoView()});b.wbg.__wbg_setAttribute_0918ea45d5a1c663=function(){return B(((a,b,d,e,f)=>{var g=C(b,d);var h=C(e,f);c(a).setAttribute(g,h)}),arguments)};b.wbg.__wbg_before_bed7b7b6e53dd469=function(){return B(((a,b)=>{c(a).before(c(b))}),arguments)};b.wbg.__wbg_remove_ed2f62f1a8be044b=(a=>{c(a).remove()});b.wbg.__wbg_append_517583bac5b5bb16=function(){return B(((a,b)=>{c(a).append(c(b))}),arguments)};b.wbg.__wbg_defaultPrevented_37035afe196e2aa0=(a=>{const b=c(a).defaultPrevented;return b});b.wbg.__wbg_composedPath_d94a39b8c8f6eed1=(a=>{const b=c(a).composedPath();return e(b)});b.wbg.__wbg_preventDefault_d2c7416966cb0632=(a=>{c(a).preventDefault()});b.wbg.__wbg_ctrlKey_0d75e0e9028bd999=(a=>{const b=c(a).ctrlKey;return b});b.wbg.__wbg_shiftKey_12353f0e19b21d6a=(a=>{const b=c(a).shiftKey;return b});b.wbg.__wbg_altKey_a076f8612103d7e8=(a=>{const b=c(a).altKey;return b});b.wbg.__wbg_metaKey_4e3f6e986f2802b1=(a=>{const b=c(a).metaKey;return b});b.wbg.__wbg_button_8a97c55db17c7314=(a=>{const b=c(a).button;return b});b.wbg.__wbg_addEventListener_2f891d22985fd3c8=function(){return B(((a,b,d,e)=>{var f=C(b,d);c(a).addEventListener(f,c(e))}),arguments)};b.wbg.__wbg_removeEventListener_07715e6f464823fc=function(){return B(((a,b,d,e)=>{var f=C(b,d);c(a).removeEventListener(f,c(e))}),arguments)};b.wbg.__wbg_childNodes_75d3da5f3a7bb985=(a=>{const b=c(a).childNodes;return e(b)});b.wbg.__wbg_previousSibling_4cd9e84aeb4df529=(a=>{const b=c(a).previousSibling;return p(b)?Q:e(b)});b.wbg.__wbg_nextSibling_13e9454ef5323f1a=(a=>{const b=c(a).nextSibling;return p(b)?Q:e(b)});b.wbg.__wbg_appendChild_bd383ec5356c0bdb=function(){return B(((a,b)=>{const d=c(a).appendChild(c(b));return e(d)}),arguments)};b.wbg.__wbg_cloneNode_80501c66ab115588=function(){return B((a=>{const b=c(a).cloneNode();return e(b)}),arguments)};b.wbg.__wbg_length_d5ed87010607a669=(a=>{const b=c(a).length;return b});b.wbg.__wbg_setdata_4d5b377238fff97c=((a,b,d)=>{var e=C(b,d);c(a).data=e});b.wbg.__wbg_state_cabf8868613a7bdb=function(){return B((a=>{const b=c(a).state;return e(b)}),arguments)};b.wbg.__wbg_pushState_90b0a1cf59505502=function(){return B(((a,b,d,e,f,g)=>{var h=C(d,e);var i=C(f,g);c(a).pushState(c(b),h,i)}),arguments)};b.wbg.__wbg_replaceState_594fcc05da10fe45=function(){return B(((a,b,d,e,f,g)=>{var h=C(d,e);var i=C(f,g);c(a).replaceState(c(b),h,i)}),arguments)};b.wbg.__wbg_instanceof_HtmlAnchorElement_afbec6867d883d45=(a=>{let b;try{b=c(a) instanceof HTMLAnchorElement}catch(a){b=!1}const d=b;return d});b.wbg.__wbg_target_a012ddff98c775bb=((b,d)=>{const e=c(d).target;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f});b.wbg.__wbg_href_596cc35e5517d06b=((b,d)=>{const e=c(d).href;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f});b.wbg.__wbg_origin_a66ff95a994d7e40=function(){return B(((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f}),arguments)};b.wbg.__wbg_pathname_2cd8b46817926b06=function(){return B(((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f}),arguments)};b.wbg.__wbg_search_eb68df82d26f8761=function(){return B(((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f}),arguments)};b.wbg.__wbg_hash_9bd16c0f666cdf27=function(){return B(((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f}),arguments)};b.wbg.__wbg_origin_028cdb6d9987f6c3=((b,d)=>{const e=c(d).origin;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f});b.wbg.__wbg_pathname_d0d5b2fd2c7d8243=((b,d)=>{const e=c(d).pathname;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f});b.wbg.__wbg_search_b5c7b044aaf64616=((b,d)=>{const e=c(d).search;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f});b.wbg.__wbg_searchParams_40c1f8d0a85de0aa=(a=>{const b=c(a).searchParams;return e(b)});b.wbg.__wbg_hash_286eced2921b7b34=((b,d)=>{const e=c(d).hash;const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f});b.wbg.__wbg_newwithbase_1151109a3f062f92=function(){return B(((a,b,c,d)=>{var f=C(a,b);var g=C(c,d);const h=new URL(f,g);return e(h)}),arguments)};b.wbg.__wbg_error_1f4e3e298a7c97f6=(a=>{console.error(c(a))});b.wbg.__wbg_warn_0e0204547af47087=(a=>{console.warn(c(a))});b.wbg.__wbg_queueMicrotask_118eeb525d584d9a=(a=>{queueMicrotask(c(a))});b.wbg.__wbg_queueMicrotask_26a89c14c53809c0=(a=>{const b=c(a).queueMicrotask;return e(b)});b.wbg.__wbindgen_is_function=(a=>{const b=typeof c(a)===R;return b});b.wbg.__wbg_get_c43534c00f382c8a=((a,b)=>{const d=c(a)[b>>>Q];return e(d)});b.wbg.__wbg_length_d99b680fd68bf71b=(a=>{const b=c(a).length;return b});b.wbg.__wbg_newnoargs_5859b6d41c6fe9f7=((a,b)=>{var c=C(a,b);const d=new Function(c);return e(d)});b.wbg.__wbindgen_is_object=(a=>{const b=c(a);const d=typeof b===`object`&&b!==L;return d});b.wbg.__wbg_next_1938cf110c9491d4=(a=>{const b=c(a).next;return e(b)});b.wbg.__wbg_next_267398d0e0761bf9=function(){return B((a=>{const b=c(a).next();return e(b)}),arguments)};b.wbg.__wbg_done_506b44765ba84b9c=(a=>{const b=c(a).done;return b});b.wbg.__wbg_value_31485d8770eb06ab=(a=>{const b=c(a).value;return e(b)});b.wbg.__wbg_iterator_364187e1ee96b750=(()=>{const a=Symbol.iterator;return e(a)});b.wbg.__wbg_get_5027b32da70f39b1=function(){return B(((a,b)=>{const d=Reflect.get(c(a),c(b));return e(d)}),arguments)};b.wbg.__wbg_call_a79f1973a4f07d5e=function(){return B(((a,b)=>{const d=c(a).call(c(b));return e(d)}),arguments)};b.wbg.__wbg_self_086b5302bcafb962=function(){return B((()=>{const a=self.self;return e(a)}),arguments)};b.wbg.__wbg_window_132fa5d7546f1de5=function(){return B((()=>{const a=window.window;return e(a)}),arguments)};b.wbg.__wbg_globalThis_e5f801a37ad7d07b=function(){return B((()=>{const a=globalThis.globalThis;return e(a)}),arguments)};b.wbg.__wbg_global_f9a61fce4af6b7c1=function(){return B((()=>{const a=global.global;return e(a)}),arguments)};b.wbg.__wbg_decodeURI_4bf318e70843e94f=function(){return B(((a,b)=>{var c=C(a,b);const d=decodeURI(c);return e(d)}),arguments)};b.wbg.__wbg_isArray_fbd24d447869b527=(a=>{const b=V(c(a));return b});b.wbg.__wbg_call_f6a2bc58c19c53c6=function(){return B(((a,b,d)=>{const f=c(a).call(c(b),c(d));return e(f)}),arguments)};b.wbg.__wbg_is_a5728dbfb61c82cd=((a,b)=>{const d=Object.is(c(a),c(b));return d});b.wbg.__wbg_exec_70dc2e84f7a29584=((a,b,d)=>{var f=C(b,d);const g=c(a).exec(f);return p(g)?Q:e(g)});b.wbg.__wbg_new_c62202c5c4bd9009=((a,b,c,d)=>{var f=C(a,b);var g=C(c,d);const h=new RegExp(f,g);return e(h)});b.wbg.__wbg_resolve_97ecd55ee839391b=(a=>{const b=Promise.resolve(c(a));return e(b)});b.wbg.__wbg_then_7aeb7c5f1536640f=((a,b)=>{const d=c(a).then(c(b));return e(d)});b.wbg.__wbindgen_debug_string=((b,d)=>{const e=u(c(d));const f=o(e,a.__wbindgen_malloc,a.__wbindgen_realloc);const g=l;r()[b/X+ M]=g;r()[b/X+ Q]=f});b.wbg.__wbindgen_throw=((a,b)=>{throw new P(k(a,b))});b.wbg.__wbindgen_closure_wrapper701=((a,b,c)=>{const d=w(a,b,234,x);return e(d)});b.wbg.__wbindgen_closure_wrapper887=((a,b,c)=>{const d=w(a,b,338,y);return e(d)});b.wbg.__wbindgen_closure_wrapper1041=((a,b,c)=>{const d=w(a,b,377,z);return e(d)});b.wbg.__wbindgen_closure_wrapper2297=((a,b,c)=>{const d=w(a,b,394,A);return e(d)});return b});var A=((b,c,d)=>{a._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h1d3728951b3ab26d(b,c,e(d))});var t=(()=>{if(s===L||s.byteLength===Q){s=new Float64Array(a.memory.buffer)};return s});var y=((b,c,d)=>{a.wasm_bindgen__convert__closures__invoke1_mut__h56a5c88df51420fa(b,c,e(d))});var f=(a=>{if(a<132)return;b[a]=d;d=a});var w=((b,c,d,e)=>{const f={a:b,b:c,cnt:M,dtor:d};const g=(...b)=>{f.cnt++;const c=f.a;f.a=Q;try{return e(c,f.b,...b)}finally{if(--f.cnt===Q){a.__wbindgen_export_2.get(f.dtor)(c,f.b);v.unregister(f)}else{f.a=c}}};g.original=f;v.register(g,f,f);return g});var o=((a,b,c)=>{if(c===K){const c=m.encode(a);const d=b(c.length,M)>>>Q;j().subarray(d,d+ c.length).set(c);l=c.length;return d};let d=a.length;let e=b(d,M)>>>Q;const f=j();let g=Q;for(;g<d;g++){const b=a.charCodeAt(g);if(b>127)break;f[e+ g]=b};if(g!==d){if(g!==Q){a=a.slice(g)};e=c(e,d,d=g+ a.length*3,M)>>>Q;const b=j().subarray(e+ g,e+ d);const f=n(a,b);g+=f.written};l=g;return e});var j=(()=>{if(i===L||i.byteLength===Q){i=new Uint8Array(a.memory.buffer)};return i});var k=((a,b)=>{a=a>>>Q;return h.decode(j().subarray(a,a+ b))});var G=((b,c)=>{a=b.exports;I.__wbindgen_wasm_module=c;s=L;q=L;i=L;a.__wbindgen_start();return a});let a;const b=new J(128).fill(K);b.push(K,L,!0,!1);let d=b.length;const h=typeof TextDecoder!==N?new TextDecoder(O,{ignoreBOM:!0,fatal:!0}):{decode:()=>{throw P(`TextDecoder not available`)}};if(typeof TextDecoder!==N){h.decode()};let i=L;let l=Q;const m=typeof TextEncoder!==N?new TextEncoder(O):{encode:()=>{throw P(`TextEncoder not available`)}};const n=typeof m.encodeInto===R?((a,b)=>m.encodeInto(a,b)):((a,b)=>{const c=m.encode(a);b.set(c);return {read:a.length,written:c.length}});let q=L;let s=L;const v=new FinalizationRegistry(b=>{a.__wbindgen_export_2.get(b.dtor)(b.a,b.b)});export default I;export{H as initSync}