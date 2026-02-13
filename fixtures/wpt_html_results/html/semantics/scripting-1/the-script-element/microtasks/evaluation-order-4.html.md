# html/semantics/scripting-1/the-script-element/microtasks/evaluation-order-4.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/microtasks/evaluation-order-4.html",
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
// Spec: https://html.spec.whatwg.org/C/#run-a-module-script
setupTest("Module script queueing a microtask then top-level await", [
  "step-4.1-1", "step-4.1-2",
  "microtask-4.1",
  "script-load",
  "step-4.2-1", "step-4.2-2",
  "microtask-4.2",
]);
window.onerror = testDone;
</script>
<script type="module" src="resources/evaluation-order-4.1.mjs"
    onerror="unreachable()" onload="log.push('script-load')"></script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/microtasks/evaluation-order-4.html"
}
```
