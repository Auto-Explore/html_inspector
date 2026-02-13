# html/editing/dnd/interactiveelements/draggable_input_image.html

Counts:
- errors: 0
- warnings: 3
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/editing/dnd/interactiveelements/draggable_input_image.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<html>
  <head>
    <title>Draggable input image</title>
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

    <p>Press your mouse button down on the orange block and drag downwards. It should drag the input image.</p>
    <p>Press your mouse button down on the image and drag downwards. It should drag the input image.</p>
    <div id="outer">
      <input type="image" src="../resources/circle.png" value="Dummy text" draggable="true" id="inner">
    </div>

    <script>
    const outer = document.getElementById("outer");
    const inner = document.getElementById("inner");

    function testDragInputImageButton(dragActionFun, msg) {
      promise_test(t => {
        return new Promise((resolve, reject) => {
          let result = {};

          const dragStartListener = ev => {
            ev.preventDefault();
            result.didReceiveDragStart = true;
            result.targetId = ev.target.id;
            result.dataTransferItemsCount = ev.dataTransfer.items.length;
            result.dataTransferTypesCount = ev.dataTransfer.types.length;
          };
          outer.addEventListener("dragstart", dragStartListener, { once: true });
          t.add_cleanup(() => outer.removeEventListener("dragstart", dragStartListener));

          outer.addEventListener("mouseup", () => {
            resolve(result);
          }, { once: true });

          dragActionFun();
        }).then(result => {
          assert_true(result.didReceiveDragStart, "dragstart should not fire");
          assert_equals(result.targetId, "inner", "should drag inner element");
          assert_equals(result.dataTransferItemsCount, 0, "dataTransfer should have no items");
          assert_equals(result.dataTransferTypesCount, 0, "dataTransfer should have no types");
        });
      }, msg);
    }

    promise_test(async t => {
      await new EventWatcher(t, window, "load").wait_for("load");
    }, "Wait for image load");

    testDragInputImageButton(async () => {
      const centerH = inner.getBoundingClientRect().height / 2;
      new test_driver.Actions()
        // Move pointer to the center of the top-border.
        .pointerMove(0, 10 - centerH, { origin: inner })
        .pointerDown()
        // Move pointer to the center of the button.
        .pointerMove(0, 0, { origin: inner })
        .pointerUp()
        .send();
    }, "Draggable input image button should be draggable from border");

    testDragInputImageButton(async () => {
      new test_driver.Actions()
        // Move pointer to the center of the image.
        .pointerMove(0, 10, { origin: inner })
        .pointerDown()
        // Move pointer downwards.
        .pointerMove(0, 30, { origin: inner })
        .pointerUp()
        .send();
    }, "Draggable input image button should be draggable from image");
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
        "byte_end": 388,
        "byte_start": 365,
        "col": 5,
        "line": 10
      }
    },
    {
      "category": "Html",
      "code": "html.input.image.alt.missing",
      "message": "Element “input” is missing required attribute “alt”.",
      "severity": "Warning",
      "span": {
        "byte_end": 824,
        "byte_start": 727,
        "col": 7,
        "line": 19
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
  "source_name": "html/editing/dnd/interactiveelements/draggable_input_image.html"
}
```
