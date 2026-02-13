# html/editing/dnd/platform/mousedown-move-inner-when-dragging-2.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/mousedown-move-inner-when-dragging-2.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/resources/testdriver.js"></script>
<script src="/resources/testdriver-vendor.js"></script>
<script src="/resources/testdriver-actions.js"></script>
<head>
<title>Test dragging still occurs when mousedown moves the inner element</title>
</head>
<body>
<div id="container">
  <template shadowrootmode="open">
    <div id="element" draggable="true" style="width: 40px; height: 40px; background-color:red;">
      <slot id="slot"></slot>
    </div>
    <div id="element2"></div>
  </template>
  <div id="inner" style="width: 30px; height: 30px; background-color:black;"></div>
</div>
<script>

promise_test(function() {
  return new Promise(r => {
    const element = container.shadowRoot.getElementById("element");

    element.addEventListener("dragstart", function(e) {
      assert_equals(e.target, element);
      r();
    });

    element.addEventListener("mousedown", function() {
      const element2 = container.shadowRoot.getElementById("element2");
      const slot = container.shadowRoot.getElementById("slot");
      element2.appendChild(slot);
    });

    new test_driver.Actions()
    .pointerMove(0, 0, {origin: inner})
    .pointerDown()
    .pointerMove(10, 10, {origin:inner})
    .pointerUp()
    .send();
  });
}, "dragstart should still fire when the mousedown event moves the container of the inner element around");
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1498,
        "byte_start": 1491,
        "col": 1,
        "line": 47
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
  "source_name": "html/editing/dnd/platform/mousedown-move-inner-when-dragging-2.html"
}
```
