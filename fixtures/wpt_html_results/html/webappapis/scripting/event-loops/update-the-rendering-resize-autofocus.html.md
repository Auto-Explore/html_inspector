# html/webappapis/scripting/event-loops/update-the-rendering-resize-autofocus.html

Counts:
- errors: 0
- warnings: 2
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/webappapis/scripting/event-loops/update-the-rendering-resize-autofocus.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<link rel="help" href="https://html.spec.whatwg.org/#event-loop-processing-model">
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<script>
let t = async_test("resize steps happen after autofocus, but before animation callbacks");
let resized = false;
let resizedOnRAF = false;

let runTest = t.step_func(function() {
  requestAnimationFrame(t.step_func(function() {
    resizedOnRAF = resized;
    requestAnimationFrame(t.step_func_done(function() {
      assert_true(!resized || resizedOnRAF, "If resize happened, it happened before animation callbacks");
    }));
  }));

  // Resize the frame. The callback should happen before the next RAF.
  let frame = document.getElementById("frame");
  frame.contentDocument.documentElement.getBoundingClientRect();
  frame.contentWindow.stop(); // Prevent async about:blank load.
  frame.contentWindow.addEventListener("resize", t.step_func(function() {
    resized = true;
  }), { once: true });
  frame.style.width = "600px";
  frame.contentDocument.documentElement.getBoundingClientRect();
  assert_false(resized, "Resize event shouldn't fire sync");
});
</script>
<iframe id="frame" width="300" height="300"></iframe>
<input autofocus onfocus="runTest()">
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
  "source_name": "html/webappapis/scripting/event-loops/update-the-rendering-resize-autofocus.html"
}
```
