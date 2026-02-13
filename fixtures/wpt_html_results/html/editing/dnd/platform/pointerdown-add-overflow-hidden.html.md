# html/editing/dnd/platform/pointerdown-add-overflow-hidden.html

Counts:
- errors: 1
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/platform/pointerdown-add-overflow-hidden.html",
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
<title>Test dragging still occurs when pointerdown adds overflow:hidden to the dragged element</title>
<style>
.dragging {
  overflow: hidden;
}
</style>
</head>
<body>
<li id="item" draggable="true">
  Item 1
</li>
<script>

promise_test(function() {
  return new Promise(r => {
    item.addEventListener("pointerdown", function() {
      item.classList.add("dragging");
    });

    item.addEventListener("dragstart", function(e) {
      assert_equals(e.target, item);
      r();
    });

    new test_driver.Actions()
    .pointerMove(0, 0, {origin: item})
    .pointerDown()
    .pointerMove(10, 10, {origin: item})
    .pointerUp()
    .send();
  });
}, "dragstart should still fire when the dragged element gets overflow:hidden in its pointerdown");
</script>
</body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.li.parent.disallowed",
      "message": "Element “li” not allowed as child of “body” in this context.",
      "severity": "Warning",
      "span": {
        "byte_end": 491,
        "byte_start": 460,
        "col": 1,
        "line": 16
      }
    },
    {
      "category": "Html",
      "code": "html.parser.stray_end_tag",
      "message": "Stray end tag “html”.",
      "severity": "Error",
      "span": {
        "byte_end": 1072,
        "byte_start": 1065,
        "col": 1,
        "line": 42
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
  "source_name": "html/editing/dnd/platform/pointerdown-add-overflow-hidden.html"
}
```
