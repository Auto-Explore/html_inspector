# html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-base-url-external-classic.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-base-url-external-classic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>import() inside compiled strings uses the script base URL inside a classic script that is loaded from a file</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="dummy"></div>

<base href="scripts/foo/">
<script>
// Tweak the base URL of the document here to distinguish:
// - document URL
// - document base URL = ../
// - External scripts' base URL = ./scripts/eval.js etc.
// - This inline script's base URL = ./scripts/foo/
document.querySelector("base").remove();
const base = document.createElement("base");
base.setAttribute("href", "../");
document.body.appendChild(base);

function load(scriptSrc) {
  const el = document.createElement("script");
  el.src = scriptSrc;
  document.body.appendChild(el);
}

function createTestPromise() {
  return new Promise((resolve, reject) => {
    window.dummyDiv.removeAttribute("onclick");
    delete window.evaluated_imports_a;
    delete window.label;

    window.continueTest = resolve;
    window.errorTest = reject;
  });
}

window.dummyDiv = document.querySelector("#dummy");

const evaluators = [
  "setTimeout",
  "eval",
  "Function",
  "reflected-inline-event-handlers",
  "inline-event-handlers-UA-code"
];

for (const label of evaluators) {
  promise_test(() => {
    const promise = createTestPromise();

    window.label = label;
    load(`dynamic-import/scripts/${label}.js`);

    return promise.then(module => {
      assert_true(window.evaluated_imports_a, "The module must have been evaluated");
      assert_equals(module.A.from, "imports-a.js", "The module namespace object must be correct");
    });
  }, label + " should successfully import");
};
</script>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.base.not_in_body",
      "message": "Element “base” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 393,
        "byte_start": 367,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 393,
        "byte_start": 367,
        "col": 1,
        "line": 11
      }
    },
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-base-url-external-classic.html"
}
```
