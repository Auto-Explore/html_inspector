# html/webappapis/scripting/events/compile-event-handler-symbol-unscopables.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/events/compile-event-handler-symbol-unscopables.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<meta charset="UTF-8" />
<title>Inline event handler scopes exclude unscopable properties</title>
<link rel="author" title="ExE Boss" href="https://ExE-Boss.tech" />
<link rel="help" href="https://html.spec.whatwg.org/multipage/webappapis.html#getting-the-current-value-of-the-event-handler" />
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>

<script>
  "use strict";
  window.testVariable = {};
</script>

<!-- test case 1: element, document, and window -->
<div id="test1_target" onclick='
  "use strict";

  window.testResults.testVariable = testVariable;
'></div>

<script>
  "use strict";

  test(() => {
    const results = window.testResults = {};

    document[Symbol.unscopables].testVariable = true;
    document.testVariable = "FAIL (document)";

    document.getElementById("test1_target").click();
    assert_equals(results.testVariable, window.testVariable);
  }, "unscopable `document.testVariable` doesn't shadow `window.testVariable`");

  test(() => {
    const results = window.testResults = {};
    const element = document.getElementById("test1_target");

    element[Symbol.unscopables].testVariable = true;
    element.testVariable = "FAIL (element)";

    element.click();
    assert_equals(results.testVariable, window.testVariable);
  }, "unscopable `element.testVariable` doesn't shadow `window.testVariable`");
</script>

<!-- test case 2: element, form owner, document, and window -->
<form id="test2_form_owner" onsubmit="return false;">
  <!-- <button> is a form-associated element and has a form owner.
      https://html.spec.whatwg.org/C/#form-associated-element -->
  <button id="test2_target" onclick='
    "use strict";

    window.testResults.testVariable = testVariable;
  '></button>
</form>

<script>
  "use strict";

  test(() => {
    const results = window.testResults = {};
    const element = document.getElementById("test2_target");
    const formOwner = document.getElementById("test2_form_owner");

    formOwner[Symbol.unscopables].testVariable = true;
    formOwner.testVariable = "FAIL (formOwner)";

    element.click();
    assert_equals(results.testVariable, window.testVariable);
  }, "unscopable `formOwner.testVariable` doesn't shadow `window.testVariable`")
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
  "source_name": "html/webappapis/scripting/events/compile-event-handler-symbol-unscopables.html"
}
```
