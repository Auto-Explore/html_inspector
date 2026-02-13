# html/webappapis/scripting/event-loops/microtask_before_prepare_the_script_element-01.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/event-loops/microtask_before_prepare_the_script_element-01.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<head>
<link rel=help href="https://html.spec.whatwg.org/#parsing-main-incdata">
<link rel=help href="https://html.spec.whatwg.org/#perform-a-microtask-checkpoint">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
</head>
<body>
<script>

let classicScriptRan = false;
let moduleScriptRan = false;
let asyncScriptRan = false;
let deferSCriptRan = false;

new MutationObserver((mutations, observer) => {
  mutations.forEach(({ addedNodes }) => {
    addedNodes.forEach(node => {
      if (node.nodeType == 1 && node.tagName == "SCRIPT") {
        node.setAttribute("type", "text/plain");
      }
    });
  });
}).observe(document.body, { childList: true, subtree: true });

let test = async_test("Microtask before prepare-the-script-element steps");
window.onload = test.step_func_done(function(e) {
  assert_false(classicScriptRan, "Classic script should not run");
  assert_false(moduleScriptRan, "Module script should not run");
  assert_false(asyncScriptRan, "Async script should not run");
  assert_false(deferSCriptRan, "Defer script should not run");
});

</script>
<script>classicScriptRan = true;</script>
<script type="module">moduleScriptRan = true;</script>
<script src="resources/async.js" async></script>
<script src="resources/defer.js" defer></script>
</body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.head.title.missing",
      "message": "Element “head” is missing a required instance of child element “title”.",
      "severity": "Warning",
      "span": null
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/webappapis/scripting/event-loops/microtask_before_prepare_the_script_element-01.html"
}
```
