# html/browsers/origin/cross-origin-objects/cross-origin-objects.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/origin/cross-origin-objects/cross-origin-objects.html",
  "validated_html_truncated": true,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<meta name="timeout" content="long">
<title>Cross-origin behavior of Window and Location</title>
<link rel="author" title="Bobby Holley (:bholley)" href="bobbyholley@gmail.com">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#security-window">
<link rel="help" href="https://html.spec.whatwg.org/multipage/#security-location">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/common/get-host-info.sub.js"></script>
<div id=log></div>
<iframe id="B"></iframe>
<iframe id="C"></iframe>
<iframe id="D"></iframe>
<iframe id="E"></iframe>
<iframe id="F"></iframe>
<iframe id="G"></iframe>
<iframe id="H"></iframe>
<script>

/*
 * Setup boilerplate. This gives us a same-origin window "B", cross-origin
 * windows "C" and "D", initially same-origin but then changing document.domain
 * windows "E" and "F", and not-same-site (also cross-origin, of course) windows
 * "G" and "H".
 */
var host_info = get_host_info();

setup({explicit_done: true});
path = location.pathname.substring(0, location.pathname.lastIndexOf('/')) + '/frame.html';
pathWithThen = location.pathname.substring(0, location.pathname.lastIndexOf('/')) + '/frame-with-then.html';
var B = document.getElementById('B').contentWindow;
var C = document.getElementById('C').contentWindow;
var D = document.getElementById('D').contentWindow;
var E = document.getElementById('E').contentWindow;
var F = document.getElementById('F').contentWindow;
var G = document.getElementById('G').contentWindow;
var H = document.getElementById('H').contentWindow;
B.frameElement.uriToLoad = path;
C.frameElement.uriToLoad = get_host_info().HTTP_REMOTE_ORIGIN + path;
D.frameElement.uriToLoad = get_host_info().HTTP_REMOTE_ORIGIN + pathWithThen;
E.frameElement.uriToLoad = path + "?setdomain";
F.frameElement.uriToLoad = pathWithThen + "?setdomain";
G.frameElement.uriToLoad = get_host_info().HTTP_NOTSAMESITE_ORIGIN + path;
H.frameElement.uriToLoad = get_host_info().HTTP_NOTSAMESITE_ORIGIN + pathWithThen;

function winName(win) {
  var iframes = document.getElementsByTagName('iframe');
  iframes.find = Array.prototype.find;
  var found = iframes.find(function (ifr) {
    return ifr.contentWindow == win;
  });
  if (found) {
    return found.id;
  }
  return "UNKNOWN";
}

function reloadSubframes(cb) {
  var iframes = document.getElementsByTagName('iframe');
  iframes.forEach = Array.prototype.forEach;
  var count = 0;
  function frameLoaded() {
    this.onload = null;
    if (++count == iframes.length)
      cb();
  }
  iframes.forEach(function(ifr) { ifr.onload = frameLoaded; ifr.setAttribute('src', ifr.uriToLoad); });
}
function isObject(x) { return Object(x) === x; }

/*
 * Note: we eschew assert_equals in a lot of these tests, since the harness ends
 * up throwing when it tries to format a message involving a cross-origin object.
 */

/*
 * List of tests.  Each test is actually a pair: an array of tests to run and a
 * boolean for whether these are promise tests.  We reload all the subframes in
 * between running each toplevel test.  This is done to avoid having to reload
 * all the subframes for every single test, which is overkill: some of these
 * tests are known to touch only one subframe.  And doing it makes the test
 * really slow.
 */
var testList = [];
function addTest(func, desc) {
  testList.push(
    {tests: [
      {func: func.bind(null, C),
       desc: desc + " (cross-origin)"},
      {func: func.bind(null, E),
       desc: desc + " (same-origin + document.domain)"},
      {func: func.bind(null, G),
       desc: desc + " (cross-site)"}],
     promiseTest: false});
}

function addPromiseTest(func, desc) {
  testList.push(
    {tests: [
      {func: func.bind(null, C),
       desc: desc + " (cross-origin)"},
      {func: func.bind(null, E),
       desc: desc + " (same-origin + document.domain)"},
      {func: func.bind(null, G),
       desc: desc + " (cross-site)"}],
     promiseTest: true});
}

/**
 * Similar helpers, but for the subframes that load frame-with-then.html
 */
function addThenTest(func, desc) {
  testList.push(
    {tests: [
      {func: func.bind(null, D),
       desc: desc + " (cross-origin)"},
      {func: func.bind(null, F),
       desc: desc + " (same-origin + document.domain)"},
      {func: func.bind(null, H),
       desc: desc + " (cross-site)"}],
     promiseTest: false});
}

function addPromiseThenTest(func, desc) {
  testList.push(
    {tests: [
      {func: func.bind(null, D),
       desc: desc + " (cross-origin)"},
      {func: func.bind(null, F),
       desc: desc + " (same-origin + document.domain)"},
      {func: func.bind(null, H),
       desc: desc + " (cross-site)"}],
     promiseTest: true});
}

/*
 * Basic smoke tests for same-origin and cross-origin behaviors.
 */

addTest(function(win) {
  // Note: we do not check location.host as its default port semantics are hard to reflect statically
  assert_equals(location.hostname, host_info.ORIGINAL_HOST, 'Need to run the top-level test from domain ' + host_info.ORIGINAL_HOST);
  assert_equals(get_port(location), host_info.HTTP_PORT, 'Need to run the top-level test from port ' + host_info.HTTP_PORT);
  assert_equals(B.parent, window, "window.parent works same-origin");
  assert_equals(win.parent, window, "window.parent works cross-origin");
  assert_equals(B.location.pathname, path, "location.href works same-origin");
  assert_throws_dom("SecurityError", function() { win.location.pathname; }, "location.pathname throws cross-origin");
  assert_equals(B.frames, 'override', "Overrides visible in the same-origin case");
  assert_equals(win.frames, win, "Overrides invisible in the cross-origin case");
  assert_equals(B.focus, 'override', "Overrides visible in the same-origin case");
  checkFunction(win.focus, Function.prototype);
  assert_not_equals(win.focus, focus, "Overrides invisible in the cross-origin case");
}, "Basic sanity-checking");

/*
 * Tests regarding which properties are allowed cross-origin.
 *
 * Also tests for [[GetOwnProperty]] and [[HasOwnProperty]] behavior.
 */

var allowedSymbols = [Symbol.toStringTag, Symbol.hasInstance,
                          Symbol.isConcatSpreadable];
var windowAllowlists = {
  namedFrames: ['donotleakme'],
  indices: ['0', '1'],
  getters: ['location', 'window', 'frames', 'self', 'top', 'parent',
            'opener', 'closed', 'length'],
  setters: ['location'],
  methods: ['postMessage', 'close', 'blur', 'focus'],
  // These are methods which return promises and, therefore, when called with a
  // cross-origin `this` object, do not throw immediately, but instead return a
  // Promise which rejects with the same SecurityError that they would
  // otherwise throw. They are not, however, cross-origin accessible.
  promiseMethods: ['createImageBitmap', 'fetch'],
}
windowAllowlists.propNames = Array.from(new Set([...windowAllowlists.indices,
                                                 ...windowAllowlists.getters,
                                                 ...windowAllowlists.setters,
                                                 ...windowAllowlists.methods,
                                                 'then'])).sort();
windowAllowlists.props = windowAllowlists.propNames.concat(allowedSymbols);

var locationAllowlists = {
  getters: [],
  setters: ['href'],
  methods: ['replace'],
  promiseMethods: [],
}
locationAllowlists.propNames = Array.from(new Set([...locationAllowlists.getters,
                                                   ...locationAllowlists.setters,
                                                   ...locationAllowlists.methods,
                                                   'then'])).sort();

// Define various sets of arguments to call cross-origin methods with. Arguments
// for any cross-origin-callable method must be valid, and should aim to have no
// side-effects. Any method without an entry in this list will be called with
// an empty arguments list.
var methodArgs = new Map(Object.entries({
  // As a basic smoke test, we call one cross-origin-inaccessible method with
  // both valid and invalid arguments to make sure that it rejects with the
  // same SecurityError regardless.
  assign: [
    [],
    ["javascript:undefined"],
  ],
  // Note: If we post a message to frame.html with a matching origin, its
  // "onmessage" handler will change its `document.domain`, and potentially
  // invalidate subsequent tests, so be sure to only pass non-matching origins.
  postMessage: [
    ["foo", "http://does-not.exist/"],
    ["foo", {}],
  ],
  replace: [["javascript:undefined"]],
}));

addTest(function(win) {
  for (var prop in window) {
    if (windowAllowlists.props.indexOf(prop) != -1) {
      win[prop]; // Shouldn't throw.
      Object.getOwnPropertyDescriptor(win, prop); // Shouldn't throw.
      assert_true(Object.prototype.hasOwnProperty.call(win, prop), "hasOwnProperty for " + String(prop));
    } else {
      assert_throws_dom("SecurityError", function() { win[prop]; }, "Should throw when accessing " + String(prop) + " on Window");
      assert_throws_dom("SecurityError", function() { Object.getOwnPropertyDescriptor(win, prop); },
                        "Should throw when accessing property descriptor for " + prop + " on Window");
      assert_throws_dom("SecurityError", function() { Object.prototype.hasOwnProperty.call(win, prop); },
                        "Should throw when invoking hasOwnProperty for " + prop + " on Window");
    }
    if (prop != 'location')
      assert_throws_dom("SecurityError", function() { win[prop] = undefined; }, "Should throw when writing to " + prop + " on Window");
  }
  for (var prop of windowAllowlists.namedFrames) {
    win[prop]; // Shouldn't throw.
    var desc = Object.getOwnPropertyDescriptor(win, prop);
    assert_false(desc.writable, "[[Writable]] for named frame " + String(prop));
    assert_false(desc.enumerable, "[[Enumerable]] for named frame " + String(prop));
    assert_true(desc.configurable, "[[Configurable]] for named frame " + String(prop));
    assert_true(Object.prototype.hasOwnProperty.call(win, prop), "hasOwnProperty for " + String(prop));
  }
  for (var prop in location) {
    if (prop == 'replace') {
      win.location[prop]; // Shouldn't throw.
      Object.getOwnPropertyDescriptor(win.location, prop); // Shouldn't throw.
      assert_true(Object.prototype.hasOwnProperty.call(win.location, prop), "hasOwnProperty for " + prop);
      assert_throws_dom("SecurityError", function() { win.location[prop] = undefined; }, "Should throw when writing to " + prop + " on Location");
    }
    else if (prop == 'href') {
      Object.getOwnPropertyDescriptor(win.location, prop); // Shouldn't throw.
      assert_true(Object.prototype.hasOwnProperty.call(win.location, prop), "hasOwnProperty for " + prop);
      assert_throws_dom("SecurityError", function() { win.location[prop] },
                        "Should throw reading href on Location");
    }
    else {
      assert_throws_dom("SecurityError", function() { win.location[prop]; }, "Should throw when accessing " + prop + " on Location");
      assert_throws_dom("SecurityError", function() { Object.getOwnPropertyDescriptor(win.location, prop); },
                        "Should throw when accessing property descriptor for " + prop + " on Location");
      assert_throws_dom("SecurityError", function() { Object.prototype.hasOwnProperty.call(win.location, prop); },
                        "Should throw when invoking hasOwnProperty for " + prop + " on Location");
      assert_throws_dom("SecurityError", function() { win.location[prop] = undefined; }, "Should throw when writing to " + prop + " on Location");
    }
  }
}, "Only certain properties are accessible cross-origin");

addPromiseTest(async function(win, test_obj) {
  async function checkProperties(objName, allowedlists) {
    var localObj = window[objName];
    var otherObj = win[objName];

    for (var prop in localObj) {
      let desc;
      for (let obj = localObj; !desc; obj = Object.getPrototypeOf(obj)) {
        desc = Object.getOwnPropertyDescriptor(obj, prop);

      }

      if ("value" in desc) {
        if (typeof desc.value === "function" && String(desc.value).includes("[native code]")) {
          if (allowedlists.promiseMethods.includes(prop)) {
            await promise_rejects_dom(test_obj, "SecurityError", desc.value.call(otherObj),
                                  `Should throw when calling ${objName}.${prop} with cross-origin this object`);
          } else if (!allowedlists.methods.includes(prop)) {
            for (let args of methodArgs.get(prop) || [[]]) {
              assert_throws_dom("SecurityError", desc.value.bind(otherObj, ...args),
                                `Should throw when calling ${objName}.${prop} with cross-origin this object`);
            }

          } else {
            for (let args of methodArgs.get(prop) || [[]]) {
              desc.value.apply(otherObj, args); // Shouldn't throw.
            }
          }
        }
      } else {
        if (desc.get) {
          if (allowedlists.getters.includes(prop)) {
            desc.get.call(otherObj); // Shouldn't throw.
          } else {
            assert_throws_dom("SecurityError", desc.get.bind(otherObj),
                              `Should throw when calling ${objName}.${prop} getter with cross-origin this object`);
          }
        }
        if (desc.set) {
          if (allowedlists.setters.includes(prop)) {
            desc.set.call(otherObj, "javascript:undefined"); // Shouldn't throw.
          } else {
            assert_throws_dom("SecurityError", desc.set.bind(otherObj, "foo"),
                              `Should throw when calling ${objName}.${prop} setter with cross-origin this object`);
          }
        }
      }
    }
  }

  await checkProperties("location", locationAllowlists);
  await checkProperties("window", windowAllowlists);
}, "Only certain properties are usable as cross-origin this objects");

/*
 * ES Internal Methods.
 */

/*
 * [[GetPrototypeOf]]
 */
addTest(function(win) {
  assert_equals(Object.getPrototypeOf(win), null, "cross-origin Window proto is null");
  assert_equals(Object.getPrototypeOf(win.location), null, "cross-origin Location proto is null (__proto__)");
  var protoGetter = Object.getOwnPropertyDescriptor(Object.prototype, '__proto__').get;
  assert_equals(protoGetter.call(win), null, "cross-origin Window proto is null");
  assert_equals(protoGetter.call(win.location), null, "cross-origin Location proto is null (__proto__)");
  assert_throws_dom("SecurityError", function() { win.__proto__; }, "__proto__ property not available cross-origin");
  assert_throws_dom("SecurityError", function() { win.location.__proto__; }, "__proto__ property not available cross-origin");

}, "[[GetPrototypeOf]] should return null");

/*
 * [[SetPrototypeOf]]
 */
addTest(function(win) {
  assert_throws_dom("SecurityError", function() { win.__proto__ = new Object(); }, "proto set on cross-origin Window");
  assert_throws_dom("SecurityError", function() { win.location.__proto__ = new Object(); }, "proto set on cross-origin Location");
  var setters = [Object.getOwnPropertyDescriptor(Object.prototype, '__proto__').set];
  if (Object.setPrototypeOf)
    setters.push(function(p) { Object.setPrototypeOf(this, p); });
  setters.forEach(function(protoSetter) {
    assert_throws_js(TypeError, function() { protoSetter.call(win, new Object()); }, "proto setter |call| on cross-origin Window");
    assert_throws_js(TypeError, function() { protoSetter.call(win.location, new Object()); }, "proto setter |call| on cross-origin Location");
  });
  // Hack to avoid "duplicate test name" harness issues.
  setters.forEach(function(protoSetter) {
    test(function() { protoSetter.call(win, null); },
         "proto setter |call| on cross-origin Window with null (" + protoSetter + ", " + winName(win) + ")");
    test(function() { protoSetter.call(win.location, null); },
         "proto setter |call| on cross-origin Location with null (" + protoSetter + ", " + winName(win) + ")");
  });
  if (Reflect.setPrototypeOf) {
    assert_false(Reflect.setPrototypeOf(win, new Object()),
                 "Reflect.setPrototypeOf on cross-origin Window");
    assert_true(Reflect.setPrototypeOf(win, null),
                "
```
(validated HTML truncated to first 16384 bytes)

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/origin/cross-origin-objects/cross-origin-objects.html"
}
```
