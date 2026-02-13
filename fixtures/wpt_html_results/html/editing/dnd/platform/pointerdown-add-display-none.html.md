# html/editing/dnd/platform/pointerdown-add-display-none.html

Counts:
- errors: 1
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/pointerdown-add-display-none.html",
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
<title>Test dragging still occurs when pointerdown adds display:none to the dragged element</title>
<style>
.dragging {
  display: none;
}

#dragBox {
  width: 200px;
  height: 200px;
  background-color: #4CAF50;
  color: white;
  border-radius: 8px;
  cursor: grab;
  user-select: none;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.inner {
  background: rgba(255, 255, 255, 0.2);
  padding: 10px;
  margin-top: 10px;
  border-radius: 4px;
}
</style>
</head>
<body>
<div id="dragBox" draggable="true">
  Drag me
  <div class="inner" id="innerButton">Click or press here</div>
</div>
<script>

promise_test(function() {
  return new Promise(r => {
    innerButton.addEventListener("pointerdown", function() {
      innerButton.classList.add("dragging");
    });

    dragBox.addEventListener("dragstart", function(e) {
      assert_equals(e.target, dragBox);
      r();
    });

    const buttonRect = innerButton.getBoundingClientRect();
    new test_driver.Actions()
    .pointerMove(0, 0, {origin: innerButton})
    .pointerDown()
    .pointerMove(buttonRect.left + 10, buttonRect.top + 10)
    .pointerUp()
    .send();
  });
}, "dragstart should still fire when the dragged element gets display:none in its pointerdown");
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
        "byte_end": 1594,
        "byte_start": 1587,
        "col": 1,
        "line": 65
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
  "source_name": "html/editing/dnd/platform/pointerdown-add-display-none.html"
}
```
