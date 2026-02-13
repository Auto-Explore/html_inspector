# html/semantics/scripting-1/the-script-element/module/dynamic-import/inline-event-handler.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/semantics/scripting-1/the-script-element/module/dynamic-import/inline-event-handler.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id='div' onmousedown='import("./../imports-a.js").then(window.continueTest);'></div>
<script>
const div = document.getElementById('div');

promise_test(t => {
  const promise = new Promise(resolve => window.continueTest = resolve);

  const event = new MouseEvent('mousedown', {'button': 1});
  div.dispatchEvent(event);

  return promise.then(() => {
    assert_true(window.evaluated_imports_a);
    div.parentNode.removeChild(div);
  });
}, "dynamic import should work when triggered from inline event handlers");
</script>
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
  "source_name": "html/semantics/scripting-1/the-script-element/module/dynamic-import/inline-event-handler.html"
}
```
