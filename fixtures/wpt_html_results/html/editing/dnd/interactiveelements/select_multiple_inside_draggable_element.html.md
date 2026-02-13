# html/editing/dnd/interactiveelements/select_multiple_inside_draggable_element.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/interactiveelements/select_multiple_inside_draggable_element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Select multiple input inside draggable element</title>
    <script src="/resources/testharness.js"></script>
    <script src="/resources/testharnessreport.js"></script>
    <script src="/resources/testdriver.js"></script>
    <script src="/resources/testdriver-vendor.js"></script>
    <script src="/resources/testdriver-actions.js"></script>
    <style type="text/css">
#inner { border: 1px solid orange; border-top-width: 20px; }
    </style>
  </head>
  <body>

    <p>Press your mouse button down on the orange block and drag downwards. It should open and select items in the dropdown, and should <strong>not</strong> drag the block or text.</p>
    <div draggable="true" id="element">
      <select multiple size="3" id="inner">
        <option>Option 1</option>
        <option>Option 2</option>
        <option>Option 3</option>
      </select>
    </div>

    <script>
    const element = document.getElementById("element");
    const inner = document.getElementById("inner");

    promise_test(t => {
      return new Promise((resolve, reject) => {
        let didReceiveDragStart = false;

        const dragStartListener = ev => {
          ev.preventDefault();
          didReceiveDragStart = true;
        };
        element.addEventListener("dragstart", dragStartListener, { once: true });
        t.add_cleanup(() => {
          element.removeEventListener("dragstart", dragStartListener);
          // Click on center to dismiss the dropdown.
          new test_driver.Actions()
            .pointerMove(0, 0, { origin: document.body })
            .pointerDown()
            .pointerUp()
            .send();
        });

        element.addEventListener("mouseup", () => {
          resolve(didReceiveDragStart);
        }, { once: true });

        const centerH = inner.getBoundingClientRect().height / 2;
        new test_driver.Actions()
          // Move pointer to the center of the top-border.
          .pointerMove(0, 10 - centerH, { origin: inner })
          .pointerDown()
          // Move pointer to the center of the select.
          .pointerMove(0, 0, { origin: inner })
          .pointerUp()
          .send();
      }).then(didReceiveDragStart => {
        assert_false(didReceiveDragStart, "dragstart should not fire");
      });
    }, "Select multiple input inside draggable element should not be draggable from border");
    </script>

  </body>
</html>
```

```json
{
  "messages": [
    {
      "category": "Html",
      "code": "html.style.type.unnecessary",
      "message": "The “type” attribute for the “style” element is not needed and should be omitted.",
      "severity": "Warning",
      "span": {
        "byte_end": 413,
        "byte_start": 390,
        "col": 5,
        "line": 10
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
  "source_name": "html/editing/dnd/interactiveelements/select_multiple_inside_draggable_element.html"
}
```
