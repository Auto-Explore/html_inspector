# html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-other-document.html

Counts:
- errors: 2
- warnings: 0
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-other-document.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Check import() works when active script is in another document</title>
<link rel="author" title="Jon Coppeard" href="mailto:jcoppeard@mozilla.com">
<script src=/resources/testharness.js></script>
<script src=/resources/testharnessreport.js></script>

<iframe id="frame" src="resources/empty-iframe.html"></iframe>

<script>

function startTest() {
  const otherWindow = document.getElementById("frame").contentWindow;
  const otherDiv = otherWindow.document.getElementById("dummy");

  function createTestPromise() {
    return new Promise((resolve, reject) => {
      otherWindow.continueTest = resolve;
      otherWindow.errorTest = reject;
    });
  }

  const evaluators = {
    eval: otherWindow.eval,
    setTimeout: otherWindow.setTimeout,
    "the Function constructor"(x) {
      otherWindow.Function(x)();
    },
  };

  for (const [label, evaluator] of Object.entries(evaluators)) {
    promise_test(t => {
      t.add_cleanup(() => {
        otherDiv.removeAttribute("onclick");
        delete otherWindow.evaluated_imports_a;
      });

      const promise = createTestPromise();

      evaluator(`import('../imports-a.js?label=${label}').then(window.continueTest, window.errorTest);`);

      return promise.then(module => {
        assert_true(otherWindow.evaluated_imports_a, "The module must have been evaluated");
        assert_equals(module.A.from, "imports-a.js", "The module namespace object must be correct");
      });
    }, label + " should successfully import");
  };

  const eventHandlerEvaluators = {
    "reflected inline event handlers"(x) {
      otherDiv.setAttribute("onclick", x);
      otherDiv.onclick();
    },
    "inline event handlers triggered by JS"(x) {
      otherDiv.setAttribute("onclick", x);
      otherDiv.click(); // different from .**on**click()
    }
  };

  for (const [label, evaluator] of Object.entries(eventHandlerEvaluators)) {
    promise_test(t => {
      t.add_cleanup(() => {
        otherDiv.removeAttribute("onclick");
        delete otherWindow.evaluated_imports_a;
      });

      const promise = createTestPromise();

      evaluator(`import('../../imports-a.js?label=${label}').then(window.continueTest, window.errorTest);`);

      return promise.then(module => {
        assert_true(otherWindow.evaluated_imports_a, "The module must have been evaluated");
        assert_equals(module.A.from, "imports-a.js", "The module namespace object must be correct");
      });
    }, label + " should successfully import");
  };
}
</script>
<body onLoad="startTest()"></body>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.body.already_open",
      "message": "Start tag “body” seen but an element of the same type was already open.",
      "severity": "Error",
      "span": {
        "byte_end": 2575,
        "byte_start": 2548,
        "col": 1,
        "line": 79
      }
    },
    {
      "category": "Html",
      "code": "html.parser.cannot_recover",
      "message": "Cannot recover after last error. Any further errors will be ignored.",
      "severity": "Error",
      "span": {
        "byte_end": 2575,
        "byte_start": 2548,
        "col": 1,
        "line": 79
      }
    }
  ],
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/string-compilation-other-document.html"
}
```
