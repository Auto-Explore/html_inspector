# html/browsers/the-window-object/named-access-on-the-window-object/cross-global-npo.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/the-window-object/named-access-on-the-window-object/cross-global-npo.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset=utf-8>
<title>Named access across globals</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id=log></div>
<script>
async_test(function() {
  var iframe = document.createElement("iframe");
  iframe.src = "cross-global-support.html";
  document.body.appendChild(iframe);
  iframe.onload = this.step_func_done(function() {
    var name = "named";
    var win = iframe.contentWindow;
    var element = win.document.getElementById(name);

    var expectedValues = [
      // [value, is own property]
      [element, false, "window"],
      [element, false, "Window.prototype"],
      [element, true, "named prototype object"],
      [undefined, false, "EventTarget.prototype"],
      [undefined, false, "Object.prototype"],
    ];
    for (var object = win; object; object = Object.getPrototypeOf(object)) {
      var expected = expectedValues.shift();
      assert_equals(object[name], expected[0], "[[Get]] on " + expected[2]);
      var desc = Object.getOwnPropertyDescriptor(object, name);
      if (expected[1]) {
        assert_not_equals(desc, undefined, "[[GetOwnProperty]] on " + expected[2] + " should return something");
        assert_equals(desc.value, element, "[[GetOwnProperty]] on " + expected[2]);
      } else {
        assert_equals(desc, undefined, "[[GetOwnProperty]] on " + expected[2]);
      }
    }
  });
});
</script>
```

```json
{
  "messages": [
    {
      "category": "I18n",
      "code": "i18n.lang.missing",
      "message": "Consider adding a “lang” attribute to the “html” start tag to declare the language of this document.",
      "severity": "Warning",
      "span": null
    }
  ],
  "source_name": "html/browsers/the-window-object/named-access-on-the-window-object/cross-global-npo.html"
}
```
