# html/semantics/scripting-1/the-script-element/module/dynamic-import/no-active-script-module-manual.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/no-active-script-module-manual.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>Dynamic import when there is no active script</title>
<link rel="help" href="https://github.com/whatwg/html/pull/4181">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<p>Click these buttons in sequence:</p>

<button id="button1">Click me 1</button>

<button id="button2">Click me 2</button>

<p>The result will be pass/fail per the testharness.js results</p>

<!-- We set the attributes from a separate script to specifically make
     sure that it's not that script's base URL that gets used, but instead this page's. -->
<script src="scripts/no-active-script.js" type="module"></script>

<script type="module">
"use strict";
setup({ explicit_timeout: true });

promise_test(t => {
  t.add_cleanup(() => {
    delete window.evaluated_imports_a;
  });

  const promise = new Promise((resolve, reject) => {
    window.continueTest1 = resolve;
    window.errorTest1 = reject;
  });

  return promise.then(module => {
    assert_true(window.evaluated_imports_a, "The module must have been evaluated");
    assert_equals(module.A.from, "imports-a.js", "The module namespace object must be correct");
  });
}, "onclick that directly imports should successfully import, using page's base URL");

promise_test(t => {
  t.add_cleanup(() => {
    delete window.evaluated_imports_a;
  });

  const promise = new Promise((resolve, reject) => {
    window.continueTest2 = resolve;
    window.errorTest2 = reject;
  });

  return promise.then(module => {
    assert_true(window.evaluated_imports_a, "The module must have been evaluated");
    assert_equals(module.A.from, "imports-a.js", "The module namespace object must be correct");
  });
}, "onclick that indirectly imports after a task should successfully import, using page's base URL");
</script>
```

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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/no-active-script-module-manual.html"
}
```
