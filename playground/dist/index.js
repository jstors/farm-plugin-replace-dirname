(globalThis || window || global)['96c70b1dc0455bd80da8756721c6f6a7'] = {__FARM_TARGET_ENV__: 'browser'};(function(r,e){var t={};function n(r){return Promise.resolve(o(r))}function o(e){if(t[e])return t[e].exports;var i={id:e,exports:{}};t[e]=i;r[e](i,i.exports,o,n);return i.exports}o(e)})({"0825bed7":function  (e,t,r,o){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),function(e,t){for(var r in t)Object.defineProperty(e,r,{enumerable:!0,get:t[r]});}(t,{ResourceLoader:function(){return n;},__farm_global_this__:function(){return s;},isBrowser:function(){return l;},targetEnv:function(){return i;}});let s=(globalThis||window||global)["96c70b1dc0455bd80da8756721c6f6a7"],i=s.__FARM_TARGET_ENV__||"node",l="browser"===i&&(globalThis||window).document;class n{moduleSystem;_loadedResources;_loadingResources;publicPaths;constructor(e,t){this.moduleSystem=e,this._loadedResources={},this._loadingResources={},this.publicPaths=t;}load(e,t=0){if(!l){let t=this.moduleSystem.pluginContainer.hookBail("loadResource",e);if(t)return t.then(t=>{if(!t.success&&t.retryWithDefaultResourceLoader){if("script"===e.type)return this._loadScript(`./${e.path}`);if("link"===e.type)return this._loadLink(`./${e.path}`);}else if(!t.success)throw Error(`[Farm] Failed to load resource: "${e.path}, type: ${e.type}". Original Error: ${t.err}`);});if("script"===e.type)return this._loadScript(`./${e.path}`);if("link"===e.type)return this._loadLink(`./${e.path}`);}let r=this.publicPaths[t],o=`${r.endsWith("/")?r.slice(0,-1):r}/${e.path}`;if(this._loadedResources[e.path])return;if(this._loadingResources[e.path])return this._loadingResources[e.path];let s=this.moduleSystem.pluginContainer.hookBail("loadResource",e);return s?s.then(r=>{if(r.success)this.setLoadedResource(e.path);else if(r.retryWithDefaultResourceLoader)return this._load(o,e,t);else throw Error(`[Farm] Failed to load resource: "${e.path}, type: ${e.type}". Original Error: ${r.err}`);}):this._load(o,e,t);}setLoadedResource(e,t=!0){this._loadedResources[e]=t;}isResourceLoaded(e){return this._loadedResources[e];}_load(e,t,r){let o=Promise.resolve();return"script"===t.type?o=this._loadScript(e):"link"===t.type&&(o=this._loadLink(e)),this._loadingResources[t.path]=o,o.then(()=>{this._loadedResources[t.path]=!0,this._loadingResources[t.path]=null;}).catch(o=>{if(console.warn(`[Farm] Failed to load resource "${e}" using publicPath: ${this.publicPaths[r]}`),++r<this.publicPaths.length)return this._load(e,t,r);throw this._loadingResources[t.path]=null,Error(`[Farm] Failed to load resource: "${t.path}, type: ${t.type}". ${o}`);}),o;}_loadScript(e){return l?new Promise((t,r)=>{let o=document.createElement("script");o.src=e,document.body.appendChild(o),o.onload=()=>{t();},o.onerror=e=>{r(e);};}):import(e);}_loadLink(e){return l?new Promise((t,r)=>{let o=document.createElement("link");o.rel="stylesheet",o.href=e,document.head.appendChild(o),o.onload=()=>{t();},o.onerror=e=>{r(e);};}):Promise.resolve();}}},"12fc1fc6":function  (e,t,n,r){"use strict";function o(e,t){return Object.keys(e).forEach(function(n){"default"===n||Object.prototype.hasOwnProperty.call(t,n)||Object.defineProperty(t,n,{enumerable:!0,get:function(){return e[n];}});}),e;}Object.defineProperty(t,"__esModule",{value:!0}),function(e,t){for(var n in t)Object.defineProperty(e,n,{enumerable:!0,get:t[n]});}(t,{_:function(){return o;},_export_star:function(){return o;}});},"22cbb9e3":function  (e,t,o,r){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),Object.defineProperty(t,"default",{enumerable:!0,get:function(){return d;}});let u="undefined"!=typeof globalThis?globalThis:window,d={name:"farm-runtime-import-meta",_moduleSystem:{},bootstrap(e){this._moduleSystem=e;},moduleCreated(e){e.meta.env={...{NODE_ENV:"production"}??{},mode:"production",dev:!1,prod:!0};let t=this._moduleSystem.publicPaths?.[0]||"",{location:o}=u,r=o?`${o.protocol}//${o.host}${t.replace(/\/$/,"")}/${e.id}?t=${Date.now()}`:e.resource_pot;e.meta.url=r;}};},"2fa519c4":function  (e,t,i,l){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),Object.defineProperty(t,"FarmRuntimePluginContainer",{enumerable:!0,get:function(){return n;}});class n{plugins=[];constructor(e){this.plugins=e;}hookSerial(e,...t){for(let i of this.plugins){let l=i[e];l&&l.apply(i,t);}}hookBail(e,...t){for(let i of this.plugins){let l=i[e];if(l){let e=l.apply(i,t);if(e)return e;}}}}},"70c88acc":function  (e,r,t,i){"use strict";Object.defineProperty(r,"__esModule",{value:!0}),Object.defineProperty(r,"ModuleSystem",{enumerable:!0,get:function(){return p;}});let s=t("81b9883b"),o=t("2fa519c4"),l=t("0825bed7"),u=t("f83e67c8"),n=t("eb60fc4c"),h=t("12fc1fc6"),d=globalThis||window,c={"@swc/helpers/_/_interop_require_default":{default:u._interop_require_default,_:u._interop_require_default},"@swc/helpers/_/_interop_require_wildcard":{default:n._interop_require_wildcard,_:n._interop_require_wildcard},"@swc/helpers/_/_export_star":{default:h._export_star,_:h._export_star}};class p{modules;cache;externalModules;reRegisterModules;publicPaths;dynamicModuleResourcesMap;resourceLoader;pluginContainer;targetEnv;constructor(){this.modules={},this.cache={},this.publicPaths=[],this.dynamicModuleResourcesMap={},this.resourceLoader=new l.ResourceLoader(this,this.publicPaths),this.pluginContainer=new o.FarmRuntimePluginContainer([]),this.targetEnv=l.targetEnv,this.externalModules={},this.reRegisterModules=!1;}require(e,r=!1){if(c[e])return c[e];if(this.cache[e]&&!this.pluginContainer.hookBail("readModuleCache",this.cache[e]))return this.cache[e].exports;let t=this.modules[e];if(!t){if(this.externalModules[e]){let t=this.externalModules[e];return r&&t.default||t;}return("node"===this.targetEnv||!l.isBrowser)&&nodeRequire?nodeRequire(e):(this.pluginContainer.hookSerial("moduleNotFound",e),console.debug(`[Farm] Module "${e}" is not registered`),{});}let i=new s.Module(e,this.require.bind(this));i.resource_pot=t.__farm_resource_pot__,this.pluginContainer.hookSerial("moduleCreated",i),this.cache[e]=i,(globalThis||global||window||{}).require||((globalThis||global||window||{require:undefined}).require=this.require.bind(this));let o=t(i,i.exports,this.require.bind(this),this.farmDynamicRequire.bind(this));return o&&o instanceof Promise?o.then(()=>(this.pluginContainer.hookSerial("moduleInitialized",i),i.exports)):(this.pluginContainer.hookSerial("moduleInitialized",i),i.exports);}farmDynamicRequire(e){if(this.modules[e]){let r=this.require(e);return r.__farm_async?r.default:Promise.resolve(r);}return this.loadDynamicResources(e);}loadDynamicResources(e,r=!1){let t=this.dynamicModuleResourcesMap[e];if(!t||0===t.length)throw Error(`Dynamic imported module "${e}" does not belong to any resource`);return r&&(this.reRegisterModules=!0,this.clearCache(e)),Promise.all(t.map(e=>(r&&this.resourceLoader.setLoadedResource(e.path,!1),this.resourceLoader.load(e)))).then(()=>{if(!this.modules[e])throw Error(`Dynamic imported module "${e}" is not registered.`);this.reRegisterModules=!1;let r=this.require(e);return r.__farm_async?r.default:r;}).catch(r=>{throw console.error(`[Farm] Error loading dynamic module "${e}"`,r),r;});}register(e,r){if(this.modules[e]&&!this.reRegisterModules){console.warn(`Module "${e}" has registered! It should not be registered twice`);return;}this.modules[e]=r;}update(e,r){this.modules[e]=r,this.clearCache(e);}delete(e){return!!this.modules[e]&&(this.clearCache(e),delete this.modules[e],!0);}getModuleUrl(e){let r=this.publicPaths[0]??"";return d.location?`${d.location.protocol}//${d.location.host}${r.endsWith("/")?r.slice(0,-1):r}/${this.modules[e].__farm_resource_pot__}`:this.modules[e].__farm_resource_pot__;}getCache(e){return this.cache[e];}clearCache(e){return!!this.cache[e]&&(delete this.cache[e],!0);}setInitialLoadedResources(e){for(let r of e)this.resourceLoader.setLoadedResource(r);}setDynamicModuleResourcesMap(e){this.dynamicModuleResourcesMap=e;}setPublicPaths(e){this.publicPaths=e,this.resourceLoader.publicPaths=this.publicPaths;}setPlugins(e){this.pluginContainer.plugins=e;}addPlugin(e){this.pluginContainer.plugins.every(r=>r.name!==e.name)&&this.pluginContainer.plugins.push(e);}removePlugin(e){this.pluginContainer.plugins=this.pluginContainer.plugins.filter(r=>r.name!==e);}setExternalModules(e){Object.assign(this.externalModules,e||{});}bootstrap(){this.pluginContainer.hookSerial("bootstrap",this);}}},"81b9883b":function  (e,t,r,i){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),Object.defineProperty(t,"Module",{enumerable:!0,get:function(){return s;}});class s{id;exports;resource_pot;meta;require;constructor(e,t){this.id=e,this.exports={},this.meta={env:{}},this.require=t;}}},"eaa15105":function  (e,t,n,u){"use strict";Object.defineProperty(t,"__esModule",{value:!0}),function(e,t){for(var n in t)Object.defineProperty(e,n,{enumerable:!0,get:t[n]});}(t,{ModuleSystem:function(){return l.ModuleSystem;},Plugin:function(){return r.FarmRuntimePlugin;}});let _=n("f83e67c8")._(n("22cbb9e3")),l=n("70c88acc"),r=n("2fa519c4");n("0825bed7").__farm_global_this__.__farm_module_system__=(function(){let e=new l.ModuleSystem;return function(){return e;};})()(),(globalThis||window||global)["96c70b1dc0455bd80da8756721c6f6a7"].__farm_module_system__.setPlugins([_.default]);},"eb60fc4c":function  (e,t,r,n){"use strict";function o(e){if("function"!=typeof WeakMap)return null;var t=new WeakMap,r=new WeakMap;return(o=function(e){return e?r:t;})(e);}function u(e,t){if(!t&&e&&e.__esModule)return e;if(null===e||"object"!=typeof e&&"function"!=typeof e)return{default:e};var r=o(t);if(r&&r.has(e))return r.get(e);var n={__proto__:null},u=Object.defineProperty&&Object.getOwnPropertyDescriptor;for(var f in e)if("default"!==f&&Object.prototype.hasOwnProperty.call(e,f)){var i=u?Object.getOwnPropertyDescriptor(e,f):null;i&&(i.get||i.set)?Object.defineProperty(n,f,i):n[f]=e[f];}return n.default=e,r&&r.set(e,n),n;}Object.defineProperty(t,"__esModule",{value:!0}),function(e,t){for(var r in t)Object.defineProperty(e,r,{enumerable:!0,get:t[r]});}(t,{_:function(){return u;},_interop_require_wildcard:function(){return u;}});},"f83e67c8":function  (e,n,t,r){"use strict";function u(e){return e&&e.__esModule?e:{default:e};}Object.defineProperty(n,"__esModule",{value:!0}),function(e,n){for(var t in n)Object.defineProperty(e,t,{enumerable:!0,get:n[t]});}(n,{_:function(){return u;},_interop_require_default:function(){return u;}});},},"eaa15105");(function(_){for(var r in _){_[r].__farm_resource_pot__='index_e727.js';(globalThis || window || global)['96c70b1dc0455bd80da8756721c6f6a7'].__farm_module_system__.register(r,_[r])}})({"da0b6852":function  (o,l,n,c){console.log(lib);},});(globalThis || window || global)['96c70b1dc0455bd80da8756721c6f6a7'].__farm_module_system__.setInitialLoadedResources([]);(globalThis || window || global)['96c70b1dc0455bd80da8756721c6f6a7'].__farm_module_system__.setDynamicModuleResourcesMap({  });var farmModuleSystem = (globalThis || window || global)['96c70b1dc0455bd80da8756721c6f6a7'].__farm_module_system__;farmModuleSystem.bootstrap();var entry = farmModuleSystem.require("da0b6852");export default entry;
//# sourceMappingURL=index.js.map