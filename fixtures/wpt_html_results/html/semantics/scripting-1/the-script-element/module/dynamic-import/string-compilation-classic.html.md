# html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-classic.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-classic.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="utf-8">
<title>import() inside compiled strings inside a classic script</title>
<link rel="author" title="Domenic Denicola" href="mailto:d@domenic.me">

<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<div id="dummy"></div>

<script>
function createTestPromise() {
  return new Promise((resolve, reject) => {
    window.continueTest = resolve;
    window.errorTest = reject;
  });
}

const dummyDiv = document.querySelector("#dummy");

const evaluators = {
  eval,
  setTimeout,
  "the Function constructor"(x) {
    Function(x)();
  },
  "reflected inline event handlers"(x) {
    dummyDiv.setAttribute("onclick", x);
    dummyDiv.onclick();
  },
  "inline event handlers triggered via UA code"(x) {
    dummyDiv.setAttribute("onclick", x);
    dummyDiv.click(); // different from .**on**click()
  }
};

for (const [label, evaluator] of Object.entries(evaluators)) {
  promise_test(t => {
    t.add_cleanup(() => {
      dummyDiv.removeAttribute("onclick");
      delete window.evaluated_imports_a;
    });

    const promise = createTestPromise();

    evaluator(`import('../imports-a.js?label=${label}').then(window.continueTest, window.errorTest);`);

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
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-classic.html"
}
```
