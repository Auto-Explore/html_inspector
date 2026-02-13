# html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.constructor.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.constructor.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!DOCTYPE html>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script src="/html/canvas/resources/canvas-tests.js"></script>
<link rel="help" href="https://html.spec.whatwg.org/#dom-offscreencanvas">
<script>

test(function() {
  assert_throws_js(
    TypeError,
    () => OffscreenCanvas(100, 50),
    "Calling OffscreenCanvas constructor without 'new' must throw"
  );
}, "OffscreenCanvas constructor called as normal function");

test(function() {
    var offscreenCanvas = new OffscreenCanvas(100, 50);
    assert_equals(offscreenCanvas.width, 100);
    assert_equals(offscreenCanvas.height, 50);

    offscreenCanvas.width = 50;
    offscreenCanvas.height = 100;
    assert_equals(offscreenCanvas.width, 50);
    assert_equals(offscreenCanvas.height, 100);
}, "Test that calling OffscreenCanvas's constructor generates correct width and height.");

test(function() {
    var offscreenCanvas1 = new OffscreenCanvas(1, 1);

    offscreenCanvas1.width = null;
    offscreenCanvas1.height = null;
    assert_equals(offscreenCanvas1.width, 0);
    assert_equals(offscreenCanvas1.height, 0);

    assert_throws_js(TypeError, function() { new OffscreenCanvas(-1, -1); });

    var offscreenCanvas2 = new OffscreenCanvas(null, null);
    assert_equals(offscreenCanvas2.width, 0);
    assert_equals(offscreenCanvas2.height, 0);

    assert_throws_js(TypeError, function() { offscreenCanvas2.width = -1; });
    assert_throws_js(TypeError, function() { offscreenCanvas2.height = -1; });

    var obj = {Name: "John Doe", Age: 30};
    assert_throws_js(TypeError, function() { offscreenCanvas2.width = obj; });
    assert_throws_js(TypeError, function() { offscreenCanvas2.height = obj; });
    assert_throws_js(TypeError, function() { new OffscreenCanvas(obj, obj); });
}, "Test that OffscreenCanvas constructor handles invalid arguments correctly");

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
  "source_name": "html/canvas/offscreen/manual/the-offscreen-canvas/offscreencanvas.constructor.html"
}
```
