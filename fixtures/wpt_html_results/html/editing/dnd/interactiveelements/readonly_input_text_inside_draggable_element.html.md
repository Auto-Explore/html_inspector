# html/editing/dnd/interactiveelements/readonly_input_text_inside_draggable_element.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/interactiveelements/readonly_input_text_inside_draggable_element.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Readonly text input inside draggable element</title>
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

    <p>Press your mouse button down on the orange block and drag downwards. Use your mouse to attempt to select part of the dummy text. It should <strong>not</strong> drag the block or text in either case.</p>
    <div draggable="true" id="element">
      <input value="Dummy text" id="inner" readonly>
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
        t.add_cleanup(() => element.removeEventListener("dragstart", dragStartListener));

        element.addEventListener("mouseup", () => {
          resolve(didReceiveDragStart);
        }, { once: true });

        const centerH = inner.getBoundingClientRect().height / 2;
        new test_driver.Actions()
          // Move pointer to the center of the top-border.
          .pointerMove(0, 10 - centerH, { origin: inner })
          .pointerDown()
          // Move pointer to the center of the input.
          .pointerMove(0, 0, { origin: inner })
          .pointerUp()
          .send();
      }).then(didReceiveDragStart => {
        assert_false(didReceiveDragStart, "dragstart should not fire");
      });
    }, "Readonly input text inside draggable element should not be draggable");

    promise_test(t => {
      return new Promise((resolve, reject) => {
        let didReceiveDragStart = false;

        const dragStartListener = ev => {
          ev.preventDefault();
          didReceiveDragStart = true;
        };
        element.addEventListener("dragstart", dragStartListener, { once: true });
        t.add_cleanup(() => element.removeEventListener("dragstart", dragStartListener));

        element.addEventListener("mouseup", () => {
          resolve(didReceiveDragStart);
        }, { once: true });

        const centerW = inner.getBoundingClientRect().width / 2;
        new test_driver.Actions()
          // Move pointer to the start of the text.
          .pointerMove(5 - centerW, 10, { origin: inner })
          .pointerDown()
          // Move pointer to the center of the input body.
          .pointerMove(0, 10, { origin: inner })
          .pointerUp()
          .send();
      }).then(didReceiveDragStart => {
        assert_false(didReceiveDragStart, "dragstart should not fire");
      });
    }, "Readonly input text inside draggable element should not be draggable while attempting to select text");
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
        "byte_end": 411,
        "byte_start": 388,
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
  "source_name": "html/editing/dnd/interactiveelements/readonly_input_text_inside_draggable_element.html"
}
```
