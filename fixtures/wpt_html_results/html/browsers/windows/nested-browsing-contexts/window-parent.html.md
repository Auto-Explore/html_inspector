# html/browsers/windows/nested-browsing-contexts/window-parent.html

Counts:
- errors: 0
- warnings: 1
- infos: 0

```json
{
  "format_version": 1,
  "file": "html/browsers/windows/nested-browsing-contexts/window-parent.html",
  "validated_html_truncated": false,
  "validated_html_max_bytes": 16384
}
```

Validated HTML:
```html
<!doctype html>
<meta charset="utf-8">
<title>window.parent</title>
<script src="/resources/testharness.js"></script>
<script src="/resources/testharnessreport.js"></script>
<div id="log"></div>
<script>
 test(function() {
   assert_equals(window, parent)
 }, '`window.parent` for top-level browsing context');

async_test(t => {
  var iframe = document.createElement('iframe');
  iframe.onload  = t.step_func_done(function () {
    var iWindow = iframe.contentWindow;
    assert_equals(iWindow.parent, window);
  });
  document.body.appendChild(iframe);
}, '`window.parent` on single nested browsing context');

async_test(t => {
  var iframe = document.createElement('iframe');
  var iframe2;

  var testFunc = t.step_func_done(function () {
    var frameWindow = iframe.contentWindow;
    var frame2Window = iframe2.contentWindow;
    assert_equals(frameWindow.parent, window);
    assert_equals(frame2Window.parent, frameWindow);
    assert_not_equals(frame2Window.parent, window);
  });

  var nestFrame = function () {
    iframe2 = iframe.contentDocument.createElement('iframe');
    // Workaround for https://bugzilla.mozilla.org/show_bug.cgi?id=1229707
    iframe2.src = '/common/blank.html';
    iframe2.addEventListener('load', testFunc);
    iframe.contentDocument.body.appendChild(iframe2);
  };

  iframe.addEventListener('load', nestFrame);
  document.body.appendChild(iframe);
}, '`window.parent` for multiple nested browsing contexts');
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
  "source_name": "html/browsers/windows/nested-browsing-contexts/window-parent.html"
}
```
