# html/semantics/scripting-1/the-script-element/microtasks/evaluation-order-1.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/microtasks/evaluation-order-1.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="resources/evaluation-order-setup.js"></script>

<script>
// Spec: https://html.spec.whatwg.org/C/#run-a-classic-script
setupTest("Classic script queueing a microtask then throwing an exception", [
  "body",         // Step 6.
  "global-error", // "Report the exception" at Step 7.3.
  "microtask",    // "Clean up after running script" at Step 7.2.
  ]);
</script>
<script src="resources/evaluation-order-1-throw.js"
    onerror="unreachable()" onload="testDone()"></script>

<script>
// Spec: https://html.spec.whatwg.org/C/#run-a-classic-script
setupTest("Classic script queueing a microtask", [
  "body",         // Step 6.
  "microtask",    // "Clean up after running script" at Step 7.2.
  ]);
</script>
<script src="resources/evaluation-order-1-nothrow.js"
    onerror="unreachable()" onload="testDone()"></script>


<script type="module" src="resources/evaluation-order-1-throw-setup.js"></script>
<script type="module" src="resources/evaluation-order-1-throw.js"
    onerror="unreachable()" onload="testDone()"></script>

<script type="module" src="resources/evaluation-order-1-nothrow-setup.js"></script>
<script type="module" src="resources/evaluation-order-1-nothrow.js"
    onerror="unreachable()" onload="testDone()"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/microtasks/evaluation-order-1.html"
}
```
