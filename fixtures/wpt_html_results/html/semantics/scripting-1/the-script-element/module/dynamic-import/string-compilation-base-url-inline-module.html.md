# html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-base-url-inline-module.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-base-url-inline-module.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>import() inside compiled strings uses the script base URL (= document base URL) inside an inline module script</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<body>
<div id="dummy"></div>

<base href="scripts/foo/">
<script type="module">
// Tweak the base URL of the document here to distinguish:
// - document URL
// - document base URL = ../
// - This inline script's base URL = ./scripts/foo/
document.querySelector("base").remove();
const base = document.createElement("base");
base.setAttribute("href", "../");
document.body.appendChild(base);

function createTestPromise() {
  return new Promise((resolve, reject) => {
    window.continueTest = resolve;
    window.errorTest = reject;
  });
}

const dummyDiv = document.querySelector("#dummy");

function doTest(label, evaluator, path) {
  promise_test(t => {
    t.add_cleanup(() => {
      dummyDiv.removeAttribute("onclick");
      delete window.evaluated_imports_a;
    });

    const promise = createTestPromise();

    evaluator(`import('${path}/imports-a.js?label=${label}').then(window.continueTest, window.errorTest);`);

    return promise.then(module => {
      assert_true(window.evaluated_imports_a, "The module must have been evaluated");
      assert_equals(module.A.from, "imports-a.js", "The module namespace object must be correct");
    });
  }, label + " should successfully import");
}

// Inline script's base URL should be used.
doTest("setTimeout", setTimeout, "../../..");
doTest("eval", eval, "../../..");
doTest("the Function constructor",
  (x) => {
    Function(x)();
  },
  "../../..");

// Document's base URL should be used, as there are no active scripts.
doTest("reflected inline event handlers",
  (x) => {
    dummyDiv.setAttribute("onclick", x);
    dummyDiv.onclick();
  },
  ".");

doTest("inline event handlers triggered via UA code",
  (x) => {
    dummyDiv.setAttribute("onclick", x);
    dummyDiv.click(); // different from .**on**click()
  },
  ".");
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
        "byte_end": 402,
        "byte_start": 376,
        "col": 1,
        "line": 12
      }
    },
    {
      "category": "Html",
      "code": "html.base.must_come_before_link_or_script",
      "message": "The “base” element must come before any “link” or “script” elements in the document.",
      "severity": "Warning",
      "span": {
        "byte_end": 402,
        "byte_start": 376,
        "col": 1,
        "line": 12
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-base-url-inline-module.html"
}
```
